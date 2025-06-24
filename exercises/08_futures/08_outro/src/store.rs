use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use crate::model::Ticket;
use uuid::Uuid;
use tokio::fs;
use serde::{Serialize, Deserialize};

pub type TicketStore = Arc<RwLock<HashMap<Uuid, Ticket>>>;

const STORE_FILE: &str = "tickets.json";

#[derive(Serialize, Deserialize)]
struct SerializableTicketStore(HashMap<Uuid, Ticket>);

pub async fn save_store(store: &TicketStore) -> Result<(), std::io::Error> {
    let store = store.read().await;
    let data = serde_json::to_string_pretty(&SerializableTicketStore(store.clone()))?;
    fs::write(STORE_FILE, data).await
}

pub async fn load_store() -> TicketStore {
    match fs::read_to_string(STORE_FILE).await {
        Ok(data) => {
            match serde_json::from_str::<SerializableTicketStore>(&data) {
                Ok(SerializableTicketStore(map)) => Arc::new(RwLock::new(map)),
                Err(_) => Arc::new(RwLock::new(HashMap::new())),
            }
        }
        Err(_) => Arc::new(RwLock::new(HashMap::new())),
    }
}

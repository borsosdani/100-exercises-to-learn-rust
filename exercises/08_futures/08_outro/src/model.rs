use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Ticket {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: TicketStatus,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TicketStatus {
    Open,
    InProgress,
    Closed,
}

#[derive(Deserialize)]
pub struct CreateTicketRequest {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct PatchTicketRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TicketStatus>,
}
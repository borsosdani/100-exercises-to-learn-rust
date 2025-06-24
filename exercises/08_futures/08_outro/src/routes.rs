use axum::{Router, routing::{post, get, patch,delete}};
use crate::handlers::{create_ticket, get_ticket, patch_ticket, delete_ticket};
use crate::store::TicketStore;


pub fn create_routes(store: TicketStore) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket))
        .route("/tickets/:id", get(get_ticket).patch(patch_ticket).delete(delete_ticket))
        .with_state(store)
}

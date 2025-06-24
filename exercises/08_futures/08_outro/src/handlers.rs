use axum::{
    extract::{Path,State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::model::{Ticket, TicketStatus, CreateTicketRequest, PatchTicketRequest};
use crate::store::TicketStore;

pub async fn create_ticket(
    State(store): State<TicketStore>,
    Json(payload): Json<CreateTicketRequest>,
) -> impl IntoResponse {
    // Manual validation
    if payload.title.trim().is_empty() || payload.title.len() > 100 {
        return (StatusCode::BAD_REQUEST, "Invalid title").into_response();
    }

    if payload.description.trim().is_empty() || payload.description.len() > 500 {
        return (StatusCode::BAD_REQUEST, "Invalid description").into_response();
    }

    let ticket = Ticket {
        id: Uuid::new_v4(),
        title: payload.title,
        description: payload.description,
        status: TicketStatus::Open,
    };

    store.write().await.insert(ticket.id, ticket.clone());

    (StatusCode::CREATED, Json(ticket)).into_response()
}

pub async fn patch_ticket(
    (Path(id), State(store), Json(payload)): (Path<Uuid>, State<TicketStore>, Json<PatchTicketRequest>)
) -> impl IntoResponse {
    let mut store = store.write().await;

    match store.get_mut(&id) {
        Some(ticket) => {
            if let Some(title) = payload.title {
                ticket.title = title;
            }
            if let Some(desc) = payload.description {
                ticket.description = desc;
            }
            if let Some(status) = payload.status {
                ticket.status = status;
            }

            (StatusCode::OK, Json(ticket.clone())).into_response()
        }
        None => (StatusCode::NOT_FOUND, "Ticket not found").into_response(),
    }
}

pub async fn get_ticket(
    Path(id): Path<Uuid>,
    State(store): State<TicketStore>,
) -> impl IntoResponse {
    let store = store.read().await;

    if let Some(ticket) = store.get(&id) {
        (StatusCode::OK, Json(ticket.clone())).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Ticket not found").into_response()
    }
}

pub async fn delete_ticket(
    Path(id): Path<Uuid>,
    State(store): State<TicketStore>,
) -> impl IntoResponse {
    let mut store = store.write().await;

    if store.remove(&id).is_some() {
        StatusCode::NO_CONTENT.into_response()
    } else {
        (StatusCode::NOT_FOUND, "Ticket not found").into_response()
    }
}
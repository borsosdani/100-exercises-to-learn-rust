use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`
use serde_json::json;

use ticket_api::{routes::create_routes, store::TicketStore};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[tokio::test]
async fn test_create_ticket_success() {
    // Arrange
    let store: TicketStore = Arc::new(RwLock::new(HashMap::new()));
    let app = create_routes(store);

    let payload = json!({
        "title": "Test Ticket",
        "description": "A description of the test ticket"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/tickets")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    // Act
    let response = app.oneshot(request).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::CREATED);

    // (Optional) Extract and parse the body to assert fields
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let ticket: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(ticket["title"], "Test Ticket");
    assert_eq!(ticket["description"], "A description of the test ticket");
    assert_eq!(ticket["status"], "Open");
}

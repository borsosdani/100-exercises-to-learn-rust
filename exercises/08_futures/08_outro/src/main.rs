mod model;
mod store;
mod handlers;
mod routes;

use axum::serve;
use tokio::{net::TcpListener, signal};
use store::{load_store, save_store};

#[tokio::main]
async fn main() {
    // 🟢 Load from disk (if exists)
    let store = load_store().await;
    let app = routes::create_routes(store.clone());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Listening on http://localhost:3000");

    // 🛑 Save on shutdown
    let server = serve(listener, app);
    let shutdown = async move {
        signal::ctrl_c().await.expect("failed to install signal handler");
        println!("\n💾 Shutting down. Saving store...");
        save_store(&store).await.expect("failed to save store");
    };

    // Graceful shutdown
    server.with_graceful_shutdown(shutdown).await.unwrap();
}

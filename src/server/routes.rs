use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

use super::handlers::{safe_monitor_handler, unsafe_monitor_handler};

pub fn create_router() -> Router {
    Router::new()
        .route("/api/unsafe", get(unsafe_monitor_handler))
        .route("/api/safe", get(safe_monitor_handler))
        .layer(CorsLayer::permissive())
        .fallback_service(tower_http::services::ServeDir::new("static"))
}

pub async fn run_server() {
    let app = create_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://localhost:3000");

    println!("\nServer logs:");
    println!("=============");

    axum::serve(
        tokio::net::TcpListener::bind(&addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}

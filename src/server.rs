use crate::{safe_monitor, unsafe_monitor};
use axum::{http::StatusCode, response::Json, routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

pub async fn run_server() {
    let app = Router::new()
        .route("/api/unsafe", get(unsafe_monitor_handler))
        .route("/api/safe", get(safe_monitor_handler))
        .layer(CorsLayer::permissive())
        .fallback_service(tower_http::services::ServeDir::new("static"));

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

async fn unsafe_monitor_handler() -> Result<Json<Vec<String>>, StatusCode> {
    let mut monitor = unsafe_monitor::StablecoinMonitor::new();

    println!("\n[UNSAFE] Attempting operation without proper setup...");
    println!("[UNSAFE] Trying to fetch data without connecting first...");

    let output = match monitor.fetch_data().await {
        Ok(_) => {
            println!("[UNSAFE] ❌ This should never succeed!");
            vec!["[ERROR] This should never succeed - we didn't connect!".to_string()]
        }
        Err(e) => {
            println!("[UNSAFE] 💥 Runtime Error: {}", e);
            vec![format!("[RUNTIME ERROR] {}", e)]
        }
    };

    Ok(Json(output))
}

async fn safe_monitor_handler() -> Result<Json<Vec<String>>, StatusCode> {
    println!("\n[SAFE] Starting type-state enforced operation...");

    let monitor = safe_monitor::StablecoinMonitor::new()
        .connect()
        .await
        .map_err(|e| {
            println!("[SAFE] ❌ Connection error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .fetch_data()
        .await
        .map_err(|e| {
            println!("[SAFE] ❌ Fetch error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    println!("[SAFE] ✅ Operation completed successfully (compiler enforced correct order)");

    Ok(Json(monitor.display_results()))
}

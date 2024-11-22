use crate::monitors::{SafeMonitor, UnsafeMonitor};
use axum::{http::StatusCode, response::Json};

pub async fn unsafe_monitor_handler() -> Result<Json<Vec<String>>, StatusCode> {
    let mut monitor = UnsafeMonitor::new();

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

pub async fn safe_monitor_handler() -> Result<Json<Vec<String>>, StatusCode> {
    println!("\n[SAFE] Starting type-state enforced operation...");

    let monitor = SafeMonitor::new()
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

use typestate_pattern_usecase::{unsafe_monitor, safe_monitor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating WITHOUT Type States:");
    println!("=================================");
    let mut unsafe_monitor = unsafe_monitor::StablecoinMonitor::new();
    
    println!("Trying to display results before fetching (will fail):");
    match unsafe_monitor.display_results() {
        Ok(_) => println!("This shouldn't happen!"),
        Err(e) => println!("Expected error: {}", e),
    }

    println!("\nNow doing it in the correct order:");
    unsafe_monitor.connect().await?;
    unsafe_monitor.fetch_data().await?;
    unsafe_monitor.display_results()?;
    
    println!("\nDemonstrating WITH Type States:");
    println!("===============================");
    // Type states enforcing the correct order at COMPILE time:
    let safe_monitor = safe_monitor::StablecoinMonitor::new()
        .connect().await?
        .fetch_data().await?;
    
    safe_monitor.display_results();
    
    Ok(())
}
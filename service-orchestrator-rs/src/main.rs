use anyhow::Result;
use serde_json::json;
use service_orchestrator_rs::scheduling::{ConnectorRegistry, Scheduler};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the registry
    let registry = Arc::new(ConnectorRegistry);

    // Initialize the scheduler
    let scheduler = Scheduler::new(registry).await?;

    // Add a social media job
    scheduler
        .add_social_job(
            "tiktok",
            "0 9 * * *", // Every day at 9 AM
            json!({ "video": "path/to/video.mp4", "caption": "Daily insight from Phoenix ORCH!" }),
        )
        .await?;

    println!("Scheduler initialized and job added.");

    // Start the scheduler
    scheduler.start().await?;

    // Keep the main thread alive
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}

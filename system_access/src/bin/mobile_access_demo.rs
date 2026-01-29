use system_access::mobile_access::security;
use system_access::mobile_access::DeviceController;
use system_access::mobile_access::Orchestrator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();

    let orch = Orchestrator::new();

    // List devices for convenience.
    if let Some(android) = orch.android.as_ref() {
        let devices = android.detect()?;
        eprintln!("Detected Android devices: {}", devices.len());
        for d in devices {
            eprintln!(
                "- {} model='{}' status='{}' consent={} ",
                d.id,
                d.model,
                d.status,
                security::check_consent(&d.id)
            );
        }
    }

    // Demo task flow.
    // NOTE: This will fail with ConsentRequired until you add the device id to
    // ~/.mobile_access/config.json under `authorized_devices`.
    let result = orch.run_task("pull photos").await;
    match result {
        Ok(msg) => {
            println!("{msg}");
            Ok(())
        }
        Err(e) => {
            eprintln!("Task failed: {e}");
            Ok(())
        }
    }
}

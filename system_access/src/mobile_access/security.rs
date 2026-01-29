use crate::mobile_access::{setup, Config, MobileError};
use chrono::Utc;
use log::info;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;

pub fn check_consent(device_id: &str) -> bool {
    match setup::load_config() {
        Ok(cfg) => cfg
            .authorized_devices
            .iter()
            .any(|d| d.eq_ignore_ascii_case(device_id)),
        Err(_) => false,
    }
}

pub fn ensure_consent(device_id: &str) -> Result<(), MobileError> {
    if check_consent(device_id) {
        Ok(())
    } else {
        Err(MobileError::ConsentRequired(device_id.to_string()))
    }
}

pub fn grant_consent(device_id: &str) -> Result<(), MobileError> {
    let mut cfg: Config = setup::load_config()?;
    if !cfg
        .authorized_devices
        .iter()
        .any(|d| d.eq_ignore_ascii_case(device_id))
    {
        cfg.authorized_devices.push(device_id.to_string());
        setup::save_config(&cfg)?;
        info!("Granted consent for device_id={device_id}");
    }
    Ok(())
}

pub fn log_action(action: &str, result: &Result<(), MobileError>) {
    match result {
        Ok(()) => info!("mobile_access action={action} result=ok"),
        Err(e) => info!("mobile_access action={action} result=err error={e}"),
    }

    // Best-effort audit trail to disk.
    if let Ok(cfg) = setup::load_config() {
        if cfg.audit_log_enabled {
            let _ = append_audit_line(action, result);
        }
    }
}

#[derive(Debug, Serialize)]
struct AuditLine<'a> {
    ts_utc: String,
    action: &'a str,
    ok: bool,
    error: Option<String>,
}

fn append_audit_line(action: &str, result: &Result<(), MobileError>) -> Result<(), MobileError> {
    setup::ensure_dirs()?;
    let path = setup::logs_dir()?.join("actions.jsonl");

    let line = AuditLine {
        ts_utc: Utc::now().to_rfc3339(),
        action,
        ok: result.is_ok(),
        error: result.as_ref().err().map(|e| e.to_string()),
    };

    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    let json = serde_json::to_string(&line)?;
    f.write_all(json.as_bytes())?;
    f.write_all(b"\n")?;
    Ok(())
}

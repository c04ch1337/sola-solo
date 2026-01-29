use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

fn now_ts() -> String {
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format!("ts_ms={}", ms)
}

pub fn logs_dir() -> Result<PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    Ok(cwd.join("logs"))
}

pub fn append_line(file_name: &str, message: &str) -> Result<(), String> {
    let dir = logs_dir()?;
    create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join(file_name);
    append_line_to_path(&path, message)
}

pub fn append_line_to_path(path: &Path, message: &str) -> Result<(), String> {
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    writeln!(f, "{} {}", now_ts(), message).map_err(|e| e.to_string())?;
    Ok(())
}


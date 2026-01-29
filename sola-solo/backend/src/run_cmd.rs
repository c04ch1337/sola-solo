use serde::Serialize;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

#[derive(Debug, Clone, Serialize)]
pub struct CmdResult {
    pub ok: bool,
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u128,
}

/// Run a command in `cwd`, capturing stdout/stderr for agent self-correction.
pub fn run_cmd(cwd: &Path, program: &str, args: &[&str]) -> std::io::Result<CmdResult> {
    let started = Instant::now();

    let output = Command::new(program)
        .current_dir(cwd)
        .args(args)
        .output()?;

    let duration_ms = started.elapsed().as_millis();
    let status = output.status.code().unwrap_or(-1);
    let ok = output.status.success();

    Ok(CmdResult {
        ok,
        status,
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        duration_ms,
    })
}


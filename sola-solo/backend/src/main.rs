use actix_cors::Cors;
use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;

mod run_cmd;
mod agent_tools_api;
mod scheduler;
mod sensory;
mod security;

use run_cmd::run_cmd;
use scheduler::{ScheduledTask, Scheduler};
use cron::Schedule;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    scheduler: Scheduler,
}

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
    /// Optional model override per-request.
    model: Option<String>,
    /// Optional system prompt.
    system: Option<String>,
}

#[derive(Deserialize)]
struct EvolveRequest {
    /// Project-relative path like "frontend/src/..." or "backend/src/...".
    path: String,
    /// Full new file contents.
    code: String,
    /// Optional human-readable reason/prompt for the change.
    note: Option<String>,
}

#[derive(Deserialize)]
struct RepairRequest {
    /// Project-relative path like "frontend/src/..." or "backend/src/...".
    path: String,
    /// The file contents that failed to compile.
    code: String,
    /// Compiler/build stderr produced by the validation step.
    stderr: String,
}

#[derive(Deserialize)]
struct ReadFileQuery {
    /// Project-relative path like "frontend/src/..." or "backend/src/...".
    path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EvolutionEntry {
    timestamp_ms: u128,
    path: String,
    snapshot_commit: String,
    status: String,
    build_status: Option<i32>,
    build_duration_ms: Option<u128>,
    build_stderr_excerpt: Option<String>,
    note: Option<String>,
}

#[derive(Deserialize)]
struct RestoreRequest {
    /// Git commit hash to restore from.
    commit_hash: String,
    /// Project-relative file path to restore.
    path: String,
}

#[derive(Deserialize)]
struct SimulateRequest {
    /// Project-relative path like "frontend/src/..." or "backend/src/...".
    path: String,
    /// Full new file contents (applied only inside sandbox).
    code: String,
}

#[derive(Debug, Clone, Serialize)]
struct SimulateResponse {
    sandbox_rel_path: String,
    result: run_cmd::CmdResult,
}

#[derive(Deserialize)]
struct BenchmarkRequest {
    /// Iteration count for bench.mjs (optional; defaults inside script if omitted).
    iters: Option<u64>,
    /// Warmup iterations (optional; defaults inside script if omitted).
    warmup: Option<u64>,
    /// Number of measurement trials for stability scoring (optional; defaults to 5).
    trials: Option<u64>,
    /// Module path to benchmark (e.g., "./src/components/Aura.tsx").
    bench_module: Option<String>,
    /// Export name to benchmark (e.g., "calculateAuraIntensity").
    bench_export: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct BenchmarkReport {
    baseline: run_cmd::CmdResult,
    mutation: run_cmd::CmdResult,
    /// Median total time in ms (filters OS context-switching outliers).
    baseline_median_ms: Option<f64>,
    mutation_median_ms: Option<f64>,
    delta_pct: Option<f64>,
    /// Stability classification: "stable", "moderate_noise", or "high_noise".
    baseline_stability: Option<String>,
    mutation_stability: Option<String>,
    /// Coefficient of variation (%) for baseline and mutation.
    baseline_cv_pct: Option<f64>,
    mutation_cv_pct: Option<f64>,
    /// True if either baseline or mutation has CV > 15% (high variance = unreliable).
    low_confidence: bool,
}

/// Parse bench.mjs JSON output for median_ms (preferred) or mean_ms (fallback).
fn parse_bench_median_ms(stdout: &str) -> Option<f64> {
    let v: serde_json::Value = serde_json::from_str(stdout.trim()).ok()?;
    // Prefer median_ms (filters outliers), fall back to mean_ms for backward compat
    v.get("median_ms")
        .and_then(|v| v.as_f64())
        .or_else(|| v.get("mean_ms").and_then(|v| v.as_f64()))
        .or_else(|| v.get("total_ms").and_then(|v| v.as_f64()))
}

/// Parse bench.mjs JSON output for stability classification.
fn parse_bench_stability(stdout: &str) -> Option<String> {
    let v: serde_json::Value = serde_json::from_str(stdout.trim()).ok()?;
    v.get("stability")?.as_str().map(|s| s.to_string())
}

/// Parse bench.mjs JSON output for coefficient of variation (cv_pct).
fn parse_bench_cv_pct(stdout: &str) -> Option<f64> {
    let v: serde_json::Value = serde_json::from_str(stdout.trim()).ok()?;
    v.get("cv_pct")?.as_f64()
}

fn sandbox_root(repo_root: &Path) -> PathBuf {
    repo_root.join("sandbox")
}

fn ensure_clean_sandbox(repo_root: &Path) -> std::io::Result<()> {
    let root = sandbox_root(repo_root);
    if root.exists() {
        let _ = std::fs::remove_dir_all(&root);
    }
    std::fs::create_dir_all(&root)
}

#[derive(Debug, Clone, Serialize)]
struct AuditHotspot {
    path: String,
    total: u64,
    build_failed: u64,
    error: u64,
    ok: u64,
}

#[derive(Debug, Clone, Serialize)]
struct AuditReport {
    total_evolutions: u64,
    ok: u64,
    build_failed: u64,
    error: u64,
    success_rate_pct: f64,
    total_build_duration_ms: u128,
    avg_build_duration_ms: f64,
    hotspots: Vec<AuditHotspot>,
}

fn analyze_evolution_trends(entries: Vec<EvolutionEntry>) -> AuditReport {
    use std::collections::HashMap;

    let total = entries.len() as u64;
    let ok = entries.iter().filter(|e| e.status == "build_ok").count() as u64;
    let build_failed = entries
        .iter()
        .filter(|e| e.status == "build_failed")
        .count() as u64;
    let error = entries.iter().filter(|e| e.status == "error").count() as u64;

    let success_rate_pct = if total == 0 {
        0.0
    } else {
        (ok as f64 / total as f64) * 100.0
    };

    let total_build_duration_ms: u128 = entries
        .iter()
        .filter_map(|e| e.build_duration_ms)
        .sum();

    let avg_build_duration_ms = if total == 0 {
        0.0
    } else {
        (total_build_duration_ms as f64) / (total as f64)
    };

    let mut by_path: HashMap<String, AuditHotspot> = HashMap::new();
    for e in entries.iter() {
        let hs = by_path.entry(e.path.clone()).or_insert_with(|| AuditHotspot {
            path: e.path.clone(),
            total: 0,
            build_failed: 0,
            error: 0,
            ok: 0,
        });
        hs.total += 1;
        match e.status.as_str() {
            "build_ok" => hs.ok += 1,
            "build_failed" => hs.build_failed += 1,
            "error" => hs.error += 1,
            _ => {}
        }
    }

    let mut hotspots: Vec<AuditHotspot> = by_path.into_values().collect();
    hotspots.sort_by(|a, b| {
        let af = a.build_failed + a.error;
        let bf = b.build_failed + b.error;
        bf.cmp(&af)
            .then_with(|| b.total.cmp(&a.total))
            .then_with(|| a.path.cmp(&b.path))
    });

    AuditReport {
        total_evolutions: total,
        ok,
        build_failed,
        error,
        success_rate_pct,
        total_build_duration_ms,
        avg_build_duration_ms,
        hotspots,
    }
}

fn manifest_path(repo_root: &Path) -> PathBuf {
    repo_root.join("evolution_manifest.json")
}

fn load_manifest(repo_root: &Path) -> Vec<EvolutionEntry> {
    let p = manifest_path(repo_root);
    let raw = std::fs::read_to_string(p).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&raw).unwrap_or_else(|_| Vec::new())
}

fn append_manifest(repo_root: &Path, entry: EvolutionEntry) {
    let mut v = load_manifest(repo_root);
    v.push(entry);
    let p = manifest_path(repo_root);
    if let Ok(s) = serde_json::to_string_pretty(&v) {
        let _ = std::fs::write(p, s);
    }
}

fn excerpt(s: &str, max: usize) -> String {
    let mut out = s.trim().to_string();
    if out.len() > max {
        out.truncate(max);
        out.push_str("…");
    }
    out
}

fn is_safe_project_relative_path(p: &str) -> bool {
    // Reject absolute paths and any traversal attempts.
    if p.contains("..") {
        return false;
    }
    let path = Path::new(p);
    !path.is_absolute()
}

#[post("/api/evolve")]
async fn evolve_handler(req: web::Json<EvolveRequest>) -> impl Responder {
    if !is_safe_project_relative_path(&req.path) {
        return HttpResponse::Forbidden().body("Invalid path");
    }

    // Backend runs from `sola-solo/backend`; anchor evolution operations at `sola-solo/`.
    let repo_root = PathBuf::from("..");
    let target_path = repo_root.join(&req.path);

    // 1) Path Guardrail: Restrict to allowed directories
    let allow_frontend = repo_root.join("frontend/src");
    let allow_backend = repo_root.join("backend/src");
    if !target_path.starts_with(&allow_frontend) && !target_path.starts_with(&allow_backend) {
        return HttpResponse::Forbidden().body("Path not in allowlist");
    }

    // 0) Git Snapshot: checkpoint current state BEFORE attempting any file writes.
    // Use allow-empty to guarantee a rollback target even when the working tree is clean.
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let msg = format!("EVO_BACKUP: {} {}", now_ms, req.path);

    let add_res = run_cmd(Path::new(&repo_root), "git", &["add", "."]);
    match add_res {
        Ok(out) if out.ok => {}
        Ok(out) => return HttpResponse::InternalServerError().json(out),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    }

    // Note: passing a reference to the formatted message is safe for this call.
    let commit_res = run_cmd(
        Path::new(&repo_root),
        "git",
        &["commit", "--allow-empty", "-m", &msg],
    );
    match commit_res {
        Ok(out) if out.ok => {}
        Ok(out) => return HttpResponse::InternalServerError().json(out),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    }

    // Record the snapshot commit hash (HEAD after commit).
    let snapshot_commit = match run_cmd(Path::new(&repo_root), "git", &["rev-parse", "HEAD"]) {
        Ok(out) if out.ok => out.stdout.trim().to_string(),
        Ok(out) => {
            return HttpResponse::InternalServerError().json(out);
        }
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    // Ensure parent directory exists
    if let Some(parent) = target_path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return HttpResponse::InternalServerError().body(format!("Create dir failed: {}", e));
        }
    }

    // 2) Atomic Flip (safe): write tmp, swap in, validate, and roll back on failure.
    let tmp_path = target_path.with_extension("tmp");
    if let Err(e) = std::fs::write(&tmp_path, &req.code) {
        return HttpResponse::InternalServerError().body(format!("Write failed: {}", e));
    }

    // Prepare a unique backup path if the target exists.
    let backup_path = if target_path.exists() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        Some(target_path.with_extension(format!("bak-{}", now)))
    } else {
        None
    };

    // Move current file out of the way (if present), then move tmp into place.
    if let Some(bak) = backup_path.as_ref() {
        if let Err(e) = std::fs::rename(&target_path, bak) {
            let _ = std::fs::remove_file(&tmp_path);
            return HttpResponse::InternalServerError().body(format!("Backup rename failed: {}", e));
        }
    }

    if let Err(e) = std::fs::rename(&tmp_path, &target_path) {
        // Best-effort restore if we had a backup.
        if let Some(bak) = backup_path.as_ref() {
            let _ = std::fs::rename(bak, &target_path);
        }
        let _ = std::fs::remove_file(&tmp_path);
        return HttpResponse::InternalServerError().body(format!("Commit rename failed: {}", e));
    }

    // 3) Validation Build: Trigger the correct toolchain
    let (cwd, prog, args): (PathBuf, &str, Vec<&str>) = if req.path.contains("frontend/") {
        (repo_root.join("frontend"), "npm", vec!["run", "build"])
    } else {
        (repo_root.join("backend"), "cargo", vec!["check"])
    };

    let result = run_cmd(&cwd, prog, &args);

    match result {
        Ok(res) if res.ok => {
            append_manifest(
                &repo_root,
                EvolutionEntry {
                    timestamp_ms: now_ms,
                    path: req.path.clone(),
                    snapshot_commit,
                    status: "build_ok".to_string(),
                    build_status: Some(res.status),
                    build_duration_ms: Some(res.duration_ms),
                    build_stderr_excerpt: None,
                    note: req.note.clone(),
                },
            );
            // Success: remove backup (if any)
            if let Some(bak) = backup_path.as_ref() {
                let _ = std::fs::remove_file(bak);
            }
            HttpResponse::Ok().json(res)
        }
        Ok(res) => {
            append_manifest(
                &repo_root,
                EvolutionEntry {
                    timestamp_ms: now_ms,
                    path: req.path.clone(),
                    snapshot_commit,
                    status: "build_failed".to_string(),
                    build_status: Some(res.status),
                    build_duration_ms: Some(res.duration_ms),
                    build_stderr_excerpt: Some(excerpt(&res.stderr, 4000)),
                    note: req.note.clone(),
                },
            );
            // Failure: roll back to backup if present, otherwise delete the created file.
            if let Some(bak) = backup_path.as_ref() {
                let _ = std::fs::remove_file(&target_path);
                let _ = std::fs::rename(bak, &target_path);
            } else {
                let _ = std::fs::remove_file(&target_path);
            }
            HttpResponse::UnprocessableEntity().json(res)
        }
        Err(e) => {
            append_manifest(
                &repo_root,
                EvolutionEntry {
                    timestamp_ms: now_ms,
                    path: req.path.clone(),
                    snapshot_commit,
                    status: "error".to_string(),
                    build_status: None,
                    build_duration_ms: None,
                    build_stderr_excerpt: Some(excerpt(&e.to_string(), 2000)),
                    note: req.note.clone(),
                },
            );
            // Treat wrapper failure as fatal; roll back as above.
            if let Some(bak) = backup_path.as_ref() {
                let _ = std::fs::remove_file(&target_path);
                let _ = std::fs::rename(bak, &target_path);
            } else {
                let _ = std::fs::remove_file(&target_path);
            }
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[post("/api/rollback")]
async fn rollback_handler() -> impl Responder {
    // Backend runs from `sola-solo/backend`; anchor git operations at `sola-solo/`.
    let repo_root = PathBuf::from("..");
    let res = run_cmd(Path::new(&repo_root), "git", &["reset", "--hard", "HEAD~1"]);

    match res {
        Ok(out) if out.ok => HttpResponse::Ok().json(out),
        Ok(out) => HttpResponse::InternalServerError().json(out),
        Err(_) => HttpResponse::InternalServerError().body("Rollback failed"),
    }
}

#[get("/api/file")]
async fn read_file_handler(q: web::Query<ReadFileQuery>) -> impl Responder {
    if !is_safe_project_relative_path(&q.path) {
        return HttpResponse::Forbidden().body("Invalid path");
    }

    // Backend runs from `sola-solo/backend`; anchor read operations at `sola-solo/`.
    let repo_root = PathBuf::from("..");
    let target_path = repo_root.join(&q.path);

    // Restrict to allowed directories (match evolve allowlist).
    let allow_frontend = repo_root.join("frontend/src");
    let allow_backend = repo_root.join("backend/src");
    if !target_path.starts_with(&allow_frontend) && !target_path.starts_with(&allow_backend) {
        return HttpResponse::Forbidden().body("Path not in allowlist");
    }

    match std::fs::read_to_string(&target_path) {
        Ok(code) => HttpResponse::Ok().json(json!({ "path": q.path, "code": code })),
        Err(e) => HttpResponse::NotFound().body(format!("Read failed: {}", e)),
    }
}

#[get("/api/manifest")]
async fn manifest_handler() -> impl Responder {
    let repo_root = PathBuf::from("..");
    HttpResponse::Ok().json(load_manifest(&repo_root))
}

#[post("/api/restore")]
async fn restore_handler(req: web::Json<RestoreRequest>) -> impl Responder {
    if !is_safe_project_relative_path(&req.path) {
        return HttpResponse::Forbidden().body("Invalid path");
    }

    let repo_root = PathBuf::from("..");
    let target_path = repo_root.join(&req.path);

    // Restrict to allowed directories (match evolve allowlist).
    let allow_frontend = repo_root.join("frontend/src");
    let allow_backend = repo_root.join("backend/src");
    if !target_path.starts_with(&allow_frontend) && !target_path.starts_with(&allow_backend) {
        return HttpResponse::Forbidden().body("Path not in allowlist");
    }

    // Restore a specific file from a historical commit.
    let res = run_cmd(
        Path::new(&repo_root),
        "git",
        &["checkout", &req.commit_hash, "--", &req.path],
    );

    match res {
        Ok(out) if out.ok => HttpResponse::Ok().json(out),
        Ok(out) => HttpResponse::InternalServerError().json(out),
        Err(_) => HttpResponse::InternalServerError().body("Restore failed"),
    }
}

#[get("/api/audit")]
async fn audit_handler() -> impl Responder {
    // Backend runs from `sola-solo/backend`; anchor read operations at `sola-solo/`.
    let repo_root = PathBuf::from("..");
    let entries = load_manifest(&repo_root);
    let report = analyze_evolution_trends(entries);
    HttpResponse::Ok().json(report)
}

#[post("/api/simulate")]
async fn simulate_handler(req: web::Json<SimulateRequest>) -> impl Responder {
    if !is_safe_project_relative_path(&req.path) {
        return HttpResponse::Forbidden().body("Invalid path");
    }

    // Backend runs from `sola-solo/backend`; anchor sandbox operations at `sola-solo/`.
    let repo_root = PathBuf::from("..");
    let target_path = repo_root.join(&req.path);

    // Restrict to allowed directories (match evolve allowlist).
    let allow_frontend = repo_root.join("frontend/src");
    let allow_backend = repo_root.join("backend/src");
    if !target_path.starts_with(&allow_frontend) && !target_path.starts_with(&allow_backend) {
        return HttpResponse::Forbidden().body("Path not in allowlist");
    }

    // Reset sandbox per simulation to guarantee isolation.
    if let Err(e) = ensure_clean_sandbox(&repo_root) {
        return HttpResponse::InternalServerError().body(format!("Sandbox init failed: {}", e));
    }

    // Copy only the relevant project subtree into sandbox.
    // This keeps it fast and avoids touching the real src/.
    let sandbox = sandbox_root(&repo_root);
    let (subdir, build_prog, build_args): (&str, &str, Vec<&str>) = if req.path.contains("frontend/") {
        ("frontend", "npm", vec!["run", "build"])
    } else {
        ("backend", "cargo", vec!["check"])
    };

    let src_dir = repo_root.join(subdir);
    let dst_dir = sandbox.join(subdir);

    // Use `xcopy` on Windows for a simple recursive copy.
    // /E: directories+subdirs, including empty; /I: assume dir; /Y: overwrite.
    let copy_from = src_dir.to_string_lossy().to_string();
    let copy_to = dst_dir.to_string_lossy().to_string();
    let copy_res = run_cmd(
        Path::new(&repo_root),
        "cmd",
        &["/C", "xcopy", "/E", "/I", "/Y", &copy_from, &copy_to],
    );

    match copy_res {
        Ok(out) if out.ok => {}
        Ok(out) => return HttpResponse::InternalServerError().json(out),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    }

    // Apply mutation only inside sandbox.
    let sandbox_file = dst_dir.join(req.path.splitn(2, '/').nth(1).unwrap_or(&req.path));
    if let Some(parent) = sandbox_file.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return HttpResponse::InternalServerError().body(format!("Sandbox mkdir failed: {}", e));
        }
    }
    if let Err(e) = std::fs::write(&sandbox_file, &req.code) {
        return HttpResponse::InternalServerError().body(format!("Sandbox write failed: {}", e));
    }

    // Run build inside sandbox subtree.
    let build_cwd = sandbox.join(subdir);
    let res = match run_cmd(&build_cwd, build_prog, &build_args) {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Sandbox build failed: {}", e));
        }
    };

    HttpResponse::Ok().json(SimulateResponse {
        sandbox_rel_path: format!("sandbox/{}/{}", subdir, sandbox_file.file_name().unwrap_or_default().to_string_lossy()),
        result: res,
    })
}

#[post("/api/benchmark")]
async fn benchmark_handler(req: web::Json<BenchmarkRequest>) -> impl Responder {
    // Compare baseline (real) vs mutation (sandbox) using bench.mjs with JIT warmup.
    let repo_root = PathBuf::from("..");

    // Build CLI args for bench.mjs
    let iters = req.iters.unwrap_or(10_000);
    let warmup = req.warmup.unwrap_or(5_000);
    let trials = req.trials.unwrap_or(5);

    let iters_str = iters.to_string();
    let warmup_str = warmup.to_string();
    let trials_str = trials.to_string();

    let args: Vec<&str> = vec![
        "bench.mjs",
        "--iters", &iters_str,
        "--warmup", &warmup_str,
        "--trials", &trials_str,
    ];

    // Use tsx for TypeScript source files, node for JS
    let use_tsx = req.bench_module.as_ref().map_or(false, |m| m.ends_with(".ts") || m.ends_with(".tsx"));
    let runner = if use_tsx { "npx" } else { "node" };
    let tsx_args: Vec<&str> = if use_tsx { vec!["tsx", "--expose-gc"] } else { vec!["--expose-gc"] };

    let baseline_cwd = repo_root.join("frontend");
    let mutation_cwd = sandbox_root(&repo_root).join("frontend");

    // Build environment variables for auto-targeting
    let mut env_vars: Vec<(String, String)> = Vec::new();
    if let Some(ref module) = req.bench_module {
        env_vars.push(("BENCH_MODULE".to_string(), module.clone()));
    }
    if let Some(ref export) = req.bench_export {
        env_vars.push(("BENCH_EXPORT".to_string(), export.clone()));
    }

    // Helper to run benchmark with environment variables
    let run_bench = |cwd: &Path| -> Result<run_cmd::CmdResult, String> {
        use std::process::Command;

        let mut cmd = if use_tsx {
            let mut c = Command::new(runner);
            for arg in &tsx_args {
                c.arg(arg);
            }
            for arg in &args {
                c.arg(arg);
            }
            c
        } else {
            let mut c = Command::new(runner);
            for arg in &tsx_args {
                c.arg(arg);
            }
            for arg in &args {
                c.arg(arg);
            }
            c
        };

        cmd.current_dir(cwd);
        for (k, v) in &env_vars {
            cmd.env(k, v);
        }

        let start = std::time::Instant::now();
        let output = cmd.output().map_err(|e| e.to_string())?;
        let duration_ms = start.elapsed().as_millis();

        Ok(run_cmd::CmdResult {
            ok: output.status.success(),
            status: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            duration_ms,
        })
    };

    let baseline = match run_bench(&baseline_cwd) {
        Ok(r) => r,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Baseline bench failed: {}", e)),
    };
    let mutation = match run_bench(&mutation_cwd) {
        Ok(r) => r,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Mutation bench failed: {}", e)),
    };

    let baseline_median_ms = parse_bench_median_ms(&baseline.stdout);
    let mutation_median_ms = parse_bench_median_ms(&mutation.stdout);
    let delta_pct = match (baseline_median_ms, mutation_median_ms) {
        (Some(b), Some(m)) if b > 0.0 => Some(((m - b) / b) * 100.0),
        _ => None,
    };

    let baseline_stability = parse_bench_stability(&baseline.stdout);
    let mutation_stability = parse_bench_stability(&mutation.stdout);
    let baseline_cv_pct = parse_bench_cv_pct(&baseline.stdout);
    let mutation_cv_pct = parse_bench_cv_pct(&mutation.stdout);

    // Low confidence if either baseline or mutation has CV > 15% (high variance).
    let low_confidence = baseline_cv_pct.map_or(false, |cv| cv > 15.0)
        || mutation_cv_pct.map_or(false, |cv| cv > 15.0);

    HttpResponse::Ok().json(BenchmarkReport {
        baseline,
        mutation,
        baseline_median_ms,
        mutation_median_ms,
        delta_pct,
        baseline_stability,
        mutation_stability,
        baseline_cv_pct,
        mutation_cv_pct,
        low_confidence,
    })
}

#[post("/api/repair")]
async fn repair_handler(req: web::Json<RepairRequest>) -> impl Responder {
    if !is_safe_project_relative_path(&req.path) {
        return HttpResponse::Forbidden().body("Invalid path");
    }

    // Reuse OpenRouter credentials and headers from the chat handler.
    let api_key = match std::env::var("OPENROUTER_API_KEY") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => return HttpResponse::InternalServerError().json(json!({"error": "Missing OPENROUTER_API_KEY"})),
    };

    let default_model = std::env::var("OPENROUTER_MODEL")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "anthropic/claude-3.5-sonnet".to_string());

    let http_referer = std::env::var("OPENROUTER_HTTP_REFERER")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "http://localhost:5173".to_string());
    let app_name = std::env::var("OPENROUTER_APP_NAME")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "sola-solo-local".to_string());

    // Strong instruction: return only the full corrected file content.
    let system = "You are a senior software engineer. Fix the provided file so the build succeeds. Return ONLY the full corrected file contents, with no markdown fences, no commentary.";
    let user = format!(
        "Your previous evolution failed for path: {}\n\nBuild error (stderr):\n{}\n\nHere is the full current file content:\n{}\n\nReturn the full corrected file content only.",
        req.path, req.stderr, req.code
    );

    let payload = json!({
        "model": default_model,
        "stream": false,
        "messages": [
            {"role": "system", "content": system},
            {"role": "user", "content": user}
        ],
        "temperature": 0.2
    });

    let client = reqwest::Client::new();
    let upstream = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("HTTP-Referer", http_referer)
        .header("X-Title", app_name)
        .json(&payload)
        .send()
        .await;

    let response = match upstream {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::BadGateway().json(json!({
                "error": "Failed to reach OpenRouter",
                "detail": e.to_string()
            }))
        }
    };

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return HttpResponse::BadGateway().json(json!({
            "error": "OpenRouter returned non-2xx",
            "status": status.as_u16(),
            "body": body
        }));
    }

    let v: serde_json::Value = match response.json().await {
        Ok(v) => v,
        Err(e) => {
            return HttpResponse::BadGateway().json(json!({
                "error": "Failed to parse OpenRouter response",
                "detail": e.to_string()
            }))
        }
    };

    let content = v
        .get("choices")
        .and_then(|c| c.get(0))
        .and_then(|c0| c0.get("message"))
        .and_then(|m| m.get("content"))
        .and_then(|c| c.as_str())
        .unwrap_or("")
        .to_string();

    if content.trim().is_empty() {
        return HttpResponse::BadGateway().json(json!({
            "error": "OpenRouter returned empty content",
            "raw": v
        }));
    }

    HttpResponse::Ok().json(json!({ "code": content }))
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({ "ok": true }))
}

#[derive(Debug, Deserialize)]
struct ScheduleAddRequest {
    name: String,
    /// Cron expression (seconds granularity; e.g. "0 0 3 * * *").
    cron: String,
    /// Opaque JSON payload describing what to do when the task fires.
    payload: serde_json::Value,
    /// If true, execute immediately on startup if a misfire is detected.
    #[serde(default)]
    critical: bool,
}

#[post("/api/agent/schedule/add")]
async fn schedule_add(state: web::Data<AppState>, req: web::Json<ScheduleAddRequest>) -> impl Responder {
    if req.name.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({"error": "name must be non-empty"}));
    }
    if req.cron.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({"error": "cron must be non-empty"}));
    }

    let now = chrono::Utc::now();
    let next_run_at = Schedule::from_str(req.cron.trim())
        .ok()
        .and_then(|s| s.upcoming(chrono::Utc).next())
        .unwrap_or(now);

    let task = ScheduledTask {
        id: Uuid::new_v4(),
        name: req.name.trim().to_string(),
        cron: req.cron.trim().to_string(),
        payload: req.payload.clone(),
        created_at: now,
        next_run_at,
        last_run_at: None,
        enabled: true,
        critical: req.critical,
    };

    match state.scheduler.add_task(task) {
        Ok(t) => HttpResponse::Ok().json(t),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e.to_string()})),
    }
}

#[get("/api/agent/schedule/list")]
async fn schedule_list(state: web::Data<AppState>) -> impl Responder {
    match state.scheduler.list_tasks() {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[delete("/api/agent/schedule/{id}")]
async fn schedule_cancel(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = match Uuid::parse_str(path.as_str()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(json!({"error": "invalid uuid"})),
    };

    match state.scheduler.cancel_task(id) {
        Ok(existed) => HttpResponse::Ok().json(json!({"ok": true, "existed": existed})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

fn sse_data_json(value: serde_json::Value) -> actix_web::web::Bytes {
    // SSE format: each event is separated by a blank line.
    let line = format!("data: {}\n\n", value.to_string());
    actix_web::web::Bytes::from(line)
}

#[post("/api/chat")]
async fn chat(req: web::Json<ChatRequest>) -> impl Responder {
    let api_key = match std::env::var("OPENROUTER_API_KEY") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            return HttpResponse::InternalServerError().json(json!({
                "error": "Missing OPENROUTER_API_KEY"
            }))
        }
    };

    let default_model = std::env::var("OPENROUTER_MODEL")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "anthropic/claude-3.5-sonnet".to_string());
    let model = req.model.clone().unwrap_or(default_model);

    // OpenRouter requires attribution headers.
    let http_referer = std::env::var("OPENROUTER_HTTP_REFERER")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "http://localhost:5173".to_string());
    let app_name = std::env::var("OPENROUTER_APP_NAME")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "sola-solo-local".to_string());

    let mut messages = Vec::new();
    if let Some(system) = req.system.as_ref().filter(|s| !s.trim().is_empty()) {
        messages.push(json!({"role": "system", "content": system}));
    }
    messages.push(json!({"role": "user", "content": req.message}));

    let payload = json!({
        "model": model,
        "stream": true,
        "messages": messages,
        // Keep defaults conservative; frontend can always refine later.
        "temperature": 0.7
    });

    let client = reqwest::Client::new();
    let upstream = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("HTTP-Referer", http_referer)
        .header("X-Title", app_name)
        .json(&payload)
        .send()
        .await;

    let response = match upstream {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::BadGateway().json(json!({
                "error": "Failed to reach OpenRouter",
                "detail": e.to_string()
            }))
        }
    };

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return HttpResponse::BadGateway().json(json!({
            "error": "OpenRouter returned non-2xx",
            "status": status.as_u16(),
            "body": body
        }));
    }

    // Transform OpenRouter's OpenAI-compatible streamed lines into simple SSE JSON events:
    //   data: {"type":"delta","text":"..."}
    //   data: {"type":"done"}
    let stream = async_stream::stream! {
        let mut upstream = response.bytes_stream();
        let mut buffer: Vec<u8> = Vec::new();

        // Tell the UI we are connected and streaming.
        yield Ok::<actix_web::web::Bytes, actix_web::Error>(sse_data_json(json!({"type": "start"})));

        while let Some(item) = upstream.next().await {
            let chunk = match item {
                Ok(b) => b,
                Err(e) => {
                    yield Ok(sse_data_json(json!({"type": "error", "message": e.to_string()})));
                    break;
                }
            };

            buffer.extend_from_slice(&chunk);

            while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                let mut line = buffer.drain(..=pos).collect::<Vec<u8>>();
                // line includes the '\n'
                if line.ends_with(b"\n") {
                    line.pop();
                }
                if line.ends_with(b"\r") {
                    line.pop();
                }

                if line.is_empty() {
                    continue;
                }

                // OpenAI/OpenRouter streams as: "data: {...}"\n\n
                const PREFIX: &[u8] = b"data: ";
                if !line.starts_with(PREFIX) {
                    continue;
                }

                let data = &line[PREFIX.len()..];
                if data == b"[DONE]" {
                    yield Ok(sse_data_json(json!({"type": "done"})));
                    return;
                }

                let v: serde_json::Value = match serde_json::from_slice(data) {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                let delta = v
                    .get("choices")
                    .and_then(|c| c.get(0))
                    .and_then(|c0| c0.get("delta"))
                    .and_then(|d| d.get("content"))
                    .and_then(|c| c.as_str())
                    .unwrap_or("");

                if !delta.is_empty() {
                    yield Ok(sse_data_json(json!({"type": "delta", "text": delta})));
                }
            }
        }

        yield Ok(sse_data_json(json!({"type": "done"})));
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/event-stream"))
        .append_header(("Cache-Control", "no-cache"))
        .append_header(("Connection", "keep-alive"))
        .append_header(("X-Accel-Buffering", "no"))
        .streaming(stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    // Persistent scheduler DB (Sled).
    let _ = std::fs::create_dir_all("data");
    let scheduler_db = sled::open("data/scheduler_db")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let scheduler = Scheduler::new(Arc::new(scheduler_db));

    // Ensure default schedules exist.
    if let Ok(tasks) = scheduler.list_tasks() {
        let now = chrono::Utc::now();

        // Daily KB pruning @ 03:00:00
        let kb_prune_name = "Daily KB Pruning";
        let kb_prune_cron = "0 0 3 * * *";
        if !tasks.iter().any(|t| t.name == kb_prune_name && t.cron == kb_prune_cron) {
            let next_run_at = Schedule::from_str(kb_prune_cron)
                .ok()
                .and_then(|s| s.upcoming(chrono::Utc).next())
                .unwrap_or(now);
            let _ = scheduler.add_task(ScheduledTask {
                id: Uuid::new_v4(),
                name: kb_prune_name.to_string(),
                cron: kb_prune_cron.to_string(),
                payload: json!({"type": "kb_prune"}),
                created_at: now,
                next_run_at,
                last_run_at: None,
                enabled: true,
                critical: false,
            });
        }

        // Weekly Failure Analysis @ Sunday 00:00:00 (critical)
        let failure_analysis_name = "Weekly Failure Analysis";
        let failure_analysis_cron = "0 0 0 * * SUN";
        if !tasks.iter().any(|t| t.name == failure_analysis_name && t.cron == failure_analysis_cron) {
            let next_run_at = Schedule::from_str(failure_analysis_cron)
                .ok()
                .and_then(|s| s.upcoming(chrono::Utc).next())
                .unwrap_or(now);
            let _ = scheduler.add_task(ScheduledTask {
                id: Uuid::new_v4(),
                name: failure_analysis_name.to_string(),
                cron: failure_analysis_cron.to_string(),
                payload: json!({"type": "failure_analysis", "scope": "weekly"}),
                created_at: now,
                next_run_at,
                last_run_at: None,
                enabled: true,
                critical: true, // Misfire recovery enabled
            });
        }

        // Hourly Presence Scan (identity recognition)
        let presence_scan_name = "Hourly Presence Scan";
        let presence_scan_cron = "0 0 * * * *"; // Every hour at :00
        if !tasks.iter().any(|t| t.name == presence_scan_name && t.cron == presence_scan_cron) {
            let next_run_at = Schedule::from_str(presence_scan_cron)
                .ok()
                .and_then(|s| s.upcoming(chrono::Utc).next())
                .unwrap_or(now);
            let _ = scheduler.add_task(ScheduledTask {
                id: Uuid::new_v4(),
                name: presence_scan_name.to_string(),
                cron: presence_scan_cron.to_string(),
                payload: json!({
                    "type": "presence_scan",
                    "action": "identify_presence",
                    "confidence_threshold": 0.8
                }),
                created_at: now,
                next_run_at,
                last_run_at: None,
                enabled: true,
                critical: false,
            });
        }
    }

    // Start tick loop; for now we just log fired/misfire events.
    let (mut sched_rx, _sched_handle) = scheduler
        .clone()
        .start(chrono::Duration::seconds(30))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    tokio::spawn(async move {
        while let Some(ev) = sched_rx.recv().await {
            eprintln!("[scheduler] event: {:?}", ev);
        }
    });

    // Initialize sensory hub (shared across all requests).
    let sensory_hub = Arc::new(sensory::SensoryHub::new());

    // Conditionally start sensory subsystems when the feature is enabled.
    #[cfg(feature = "sensory")]
    {
        let sensory_clone = sensory_hub.clone();
        tokio::spawn(async move {
            sensory_clone.start_all().await;
        });
    }

    // Create agent tools state with sensory hub reference.
    let agent_tools_state = web::Data::new(agent_tools_api::AgentToolsState::new(
        sensory_hub.clone(),
    ));

    // Vite dev origin (pinned)
    let cors_origin = "http://127.0.0.1:3000";

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(cors_origin)
            .allowed_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Content-Type"]) // keep minimal
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(AppState {
                scheduler: scheduler.clone(),
            }))
            .app_data(agent_tools_state.clone())
            .wrap(cors)
            .service(health)
            .service(chat)
            .configure(agent_tools_api::configure)
            .service(schedule_add)
            .service(schedule_list)
            .service(schedule_cancel)
            .service(evolve_handler)
            .service(repair_handler)
            .service(rollback_handler)
            .service(read_file_handler)
            .service(manifest_handler)
            .service(restore_handler)
            .service(audit_handler)
            .service(simulate_handler)
            .service(benchmark_handler)
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}

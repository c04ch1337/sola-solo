use crate::mobile_access::{Config, MobileError};
use log::{info, warn};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const USER_AGENT: &str = "phoenix-mobile-access/0.1";

fn home_dir() -> Result<PathBuf, MobileError> {
    if let Ok(p) = std::env::var("USERPROFILE") {
        return Ok(PathBuf::from(p));
    }
    if let Ok(p) = std::env::var("HOME") {
        return Ok(PathBuf::from(p));
    }
    Err(MobileError::Config(
        "Unable to determine home directory (HOME/USERPROFILE not set)".to_string(),
    ))
}

pub fn base_dir() -> Result<PathBuf, MobileError> {
    Ok(home_dir()?.join(".mobile_access"))
}

pub fn config_path() -> Result<PathBuf, MobileError> {
    Ok(base_dir()?.join("config.json"))
}

pub fn tools_dir() -> Result<PathBuf, MobileError> {
    Ok(base_dir()?.join("tools"))
}

pub fn data_dir() -> Result<PathBuf, MobileError> {
    Ok(base_dir()?.join("data"))
}

pub fn logs_dir() -> Result<PathBuf, MobileError> {
    Ok(base_dir()?.join("logs"))
}

pub fn ensure_dirs() -> Result<(), MobileError> {
    fs::create_dir_all(tools_dir()?)?;
    fs::create_dir_all(data_dir()?)?;
    fs::create_dir_all(logs_dir()?)?;
    Ok(())
}

fn path_delimiter() -> char {
    if cfg!(windows) {
        ';'
    } else {
        ':'
    }
}

fn prepend_to_path(dir: &Path) {
    let dir_s = match dir.to_str() {
        Some(s) => s,
        None => return,
    };

    let key = if cfg!(windows) { "Path" } else { "PATH" };
    let current = std::env::var(key).unwrap_or_default();
    let delim = path_delimiter();
    let mut parts: Vec<String> = current.split(delim).map(|s| s.to_string()).collect();
    if parts.iter().any(|p| p.eq_ignore_ascii_case(dir_s)) {
        return;
    }
    parts.insert(0, dir_s.to_string());
    let new_val = parts.join(&delim.to_string());
    std::env::set_var(key, new_val);
}

pub fn ensure_adb(cfg: &mut Config) -> Result<PathBuf, MobileError> {
    if let Some(p) = cfg.adb_path.clone() {
        if p.exists() {
            if let Some(parent) = p.parent() {
                prepend_to_path(parent);
            }
            return Ok(p);
        }
    }

    let adb = deploy_adb()?;
    if let Some(parent) = adb.parent() {
        prepend_to_path(parent);
    }
    cfg.adb_path = Some(adb.clone());
    save_config(cfg)?;
    Ok(adb)
}

pub fn ensure_scrcpy(cfg: &mut Config) -> Result<PathBuf, MobileError> {
    if let Some(p) = cfg.scrcpy_path.clone() {
        if p.exists() {
            return Ok(p);
        }
    }

    let scrcpy = deploy_scrcpy()?;
    cfg.scrcpy_path = Some(scrcpy.clone());
    save_config(cfg)?;
    Ok(scrcpy)
}

pub fn load_config() -> Result<Config, MobileError> {
    ensure_dirs()?;
    let path = config_path()?;
    if !path.exists() {
        return Ok(Config::default());
    }
    let text = fs::read_to_string(&path)?;
    let cfg = serde_json::from_str::<Config>(&text)?;
    Ok(cfg)
}

pub fn save_config(cfg: &Config) -> Result<(), MobileError> {
    ensure_dirs()?;
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let text = serde_json::to_string_pretty(cfg)?;
    fs::write(path, text)?;
    Ok(())
}

fn http_client() -> Result<Client, MobileError> {
    Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .map_err(|e| MobileError::Http(e.to_string()))
}

fn download_to(url: &str, dest: &Path) -> Result<(), MobileError> {
    if dest.exists() {
        return Ok(());
    }
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    info!("Downloading {url} -> {}", dest.display());
    let client = http_client()?;
    let mut resp = client
        .get(url)
        .send()
        .map_err(|e| MobileError::Http(e.to_string()))?
        .error_for_status()
        .map_err(|e| MobileError::Http(e.to_string()))?;

    let tmp = dest.with_extension("download");
    let mut out = BufWriter::new(fs::File::create(&tmp)?);
    io::copy(&mut resp, &mut out)?;
    out.flush()?;
    fs::rename(tmp, dest)?;
    Ok(())
}

fn extract_archive(archive_path: &Path, dest_dir: &Path) -> Result<(), MobileError> {
    fs::create_dir_all(dest_dir)?;

    let ext = archive_path
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("")
        .to_ascii_lowercase();

    #[cfg(windows)]
    {
        if ext == "zip" {
            // PowerShell Expand-Archive is available on modern Windows.
            let status = Command::new("powershell")
                .arg("-NoProfile")
                .arg("-Command")
                .arg(format!(
                    "Expand-Archive -Force -LiteralPath '{}' -DestinationPath '{}'",
                    archive_path.display(),
                    dest_dir.display()
                ))
                .status()
                .map_err(|e| {
                    MobileError::Deployment(format!("Expand-Archive spawn failed: {e}"))
                })?;
            if !status.success() {
                return Err(MobileError::Deployment(format!(
                    "Expand-Archive failed (exit={})",
                    status.code().unwrap_or(-1)
                )));
            }
            return Ok(());
        }
    }

    #[cfg(not(windows))]
    {
        if ext == "zip" {
            let status = Command::new("unzip")
                .arg("-o")
                .arg(archive_path)
                .arg("-d")
                .arg(dest_dir)
                .status()
                .map_err(|e| MobileError::Deployment(format!("unzip spawn failed: {e}")))?;
            if !status.success() {
                return Err(MobileError::Deployment(format!(
                    "unzip failed (exit={})",
                    status.code().unwrap_or(-1)
                )));
            }
            return Ok(());
        }
        if archive_path
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("")
            .ends_with(".tar.gz")
        {
            let status = Command::new("tar")
                .arg("-xzf")
                .arg(archive_path)
                .arg("-C")
                .arg(dest_dir)
                .status()
                .map_err(|e| MobileError::Deployment(format!("tar spawn failed: {e}")))?;
            if !status.success() {
                return Err(MobileError::Deployment(format!(
                    "tar failed (exit={})",
                    status.code().unwrap_or(-1)
                )));
            }
            return Ok(());
        }
    }

    Err(MobileError::Deployment(format!(
        "Unsupported archive format: {}",
        archive_path.display()
    )))
}

fn find_file_recursive(dir: &Path, file_name: &str) -> Result<Option<PathBuf>, MobileError> {
    if !dir.exists() {
        return Ok(None);
    }
    for entry in walkdir::WalkDir::new(dir).follow_links(true) {
        let entry = entry.map_err(|e| MobileError::Io(io::Error::other(e)))?;
        if entry.file_type().is_file()
            && entry
                .file_name()
                .to_string_lossy()
                .eq_ignore_ascii_case(file_name)
        {
            return Ok(Some(entry.path().to_path_buf()));
        }
    }
    Ok(None)
}

pub fn deploy_adb() -> Result<PathBuf, MobileError> {
    ensure_dirs()?;

    let url = if cfg!(windows) {
        "https://dl.google.com/android/repository/platform-tools-latest-windows.zip"
    } else if cfg!(target_os = "macos") {
        "https://dl.google.com/android/repository/platform-tools-latest-darwin.zip"
    } else {
        "https://dl.google.com/android/repository/platform-tools-latest-linux.zip"
    };

    let tools = tools_dir()?;
    let archive = tools.join("platform-tools.zip");
    let dest = tools.join("platform-tools");

    download_to(url, &archive)?;
    if !dest.join("platform-tools").exists() {
        // Extract into a container dir so repeated extractions do not clutter.
        extract_archive(&archive, &dest)?;
    }

    let adb_name = if cfg!(windows) { "adb.exe" } else { "adb" };
    let adb = dest.join("platform-tools").join(adb_name);
    if adb.exists() {
        Ok(adb)
    } else {
        Err(MobileError::Deployment(format!(
            "ADB not found after extraction at {}",
            adb.display()
        )))
    }
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

fn fetch_latest_scrcpy_asset_url() -> Result<String, MobileError> {
    let client = http_client()?;
    let resp = client
        .get("https://api.github.com/repos/Genymobile/scrcpy/releases/latest")
        .send()
        .map_err(|e| MobileError::Http(e.to_string()))?
        .error_for_status()
        .map_err(|e| MobileError::Http(e.to_string()))?;

    let release = resp
        .json::<GithubRelease>()
        .map_err(|e| MobileError::Http(e.to_string()))?;

    let wanted = if cfg!(windows) {
        vec!["win64", "windows"]
    } else if cfg!(target_os = "macos") {
        vec!["macos", "mac"]
    } else {
        vec!["linux"]
    };

    let asset = release.assets.into_iter().find(|a| {
        let n = a.name.to_ascii_lowercase();
        wanted.iter().any(|k| n.contains(k)) && (n.ends_with(".zip") || n.ends_with(".tar.gz"))
    });

    match asset {
        Some(a) => Ok(a.browser_download_url),
        None => Err(MobileError::Deployment(
            "Could not find a suitable scrcpy release asset for this platform".to_string(),
        )),
    }
}

pub fn deploy_scrcpy() -> Result<PathBuf, MobileError> {
    ensure_dirs()?;

    let url = fetch_latest_scrcpy_asset_url()?;
    let tools = tools_dir()?;

    let archive_ext = if url.to_ascii_lowercase().ends_with(".tar.gz") {
        "tar.gz"
    } else {
        "zip"
    };
    let archive = tools.join(format!("scrcpy.{archive_ext}"));
    let dest = tools.join("scrcpy");

    download_to(&url, &archive)?;
    if !dest.exists()
        || fs::read_dir(&dest)
            .map(|mut i| i.next().is_none())
            .unwrap_or(true)
    {
        // Extract into the dest directory.
        extract_archive(&archive, &dest)?;
    }

    let scrcpy_name = if cfg!(windows) {
        "scrcpy.exe"
    } else {
        "scrcpy"
    };
    if let Some(found) = find_file_recursive(&dest, scrcpy_name)? {
        Ok(found)
    } else {
        warn!(
            "scrcpy extracted but binary not found under {}",
            dest.display()
        );
        Err(MobileError::Deployment(format!(
            "scrcpy binary not found after extraction under {}",
            dest.display()
        )))
    }
}

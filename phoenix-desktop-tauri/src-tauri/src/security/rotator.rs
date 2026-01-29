use crate::{audit, vault};
use rand_core::RngCore;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;
use zeroize::Zeroize;

fn tmp_path(path: &Path) -> PathBuf {
    let mut p = path.to_path_buf();
    p.set_extension("sola.tmp");
    p
}

fn bak_path(path: &Path) -> PathBuf {
    let mut p = path.to_path_buf();
    p.set_extension("sola.bak");
    p
}

/// Re-encrypt all `*.sola` blobs under `profiles_dir` using a shadow-buffer rename strategy.
pub async fn rotate_profiles_dir(
    profiles_dir: &Path,
    old_key: &[u8; vault::AES256_KEY_LEN],
    new_key: &[u8; vault::AES256_KEY_LEN],
) -> Result<u64, String> {
    tokio::fs::create_dir_all(profiles_dir).await.map_err(|e| e.to_string())?;

    let mut rotated: u64 = 0;
    let mut rd = tokio::fs::read_dir(profiles_dir).await.map_err(|e| e.to_string())?;
    while let Some(entry) = rd.next_entry().await.map_err(|e| e.to_string())? {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("sola") {
            continue;
        }

        let blob = tokio::fs::read(&path).await.map_err(|e| e.to_string())?;
        let plaintext = vault::decrypt_persona_data(old_key, &blob).map_err(|e| e.to_string())?;
        let reblob = vault::encrypt_persona_data(new_key, &plaintext, None).map_err(|e| e.to_string())?;

        let tmp = tmp_path(&path);
        let bak = bak_path(&path);

        // 1) write tmp
        {
            let mut f = tokio::fs::File::create(&tmp).await.map_err(|e| e.to_string())?;
            f.write_all(&reblob).await.map_err(|e| e.to_string())?;
            f.flush().await.map_err(|e| e.to_string())?;
        }

        // 2) move original -> bak (shadow)
        if tokio::fs::metadata(&bak).await.is_ok() {
            let _ = tokio::fs::remove_file(&bak).await;
        }
        tokio::fs::rename(&path, &bak).await.map_err(|e| e.to_string())?;

        // 3) move tmp -> original
        tokio::fs::rename(&tmp, &path).await.map_err(|e| e.to_string())?;

        // 4) best-effort remove bak
        let _ = tokio::fs::remove_file(&bak).await;
        rotated = rotated.saturating_add(1);
    }

    Ok(rotated)
}

/// Overwrite all `*.sola` blobs with random noise and then remove them.
///
/// This is destructive and designed for emergency lockdown.
pub async fn purge_profiles_dir(profiles_dir: &Path) -> Result<u64, String> {
    if tokio::fs::metadata(profiles_dir).await.is_err() {
        return Ok(0);
    }

    let mut purged: u64 = 0;
    let mut rd = tokio::fs::read_dir(profiles_dir).await.map_err(|e| e.to_string())?;
    while let Some(entry) = rd.next_entry().await.map_err(|e| e.to_string())? {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("sola") {
            continue;
        }

        let meta = tokio::fs::metadata(&path).await.map_err(|e| e.to_string())?;
        let len = meta.len() as usize;

        // Overwrite in chunks to avoid huge allocations.
        let mut f = tokio::fs::OpenOptions::new()
            .write(true)
            .open(&path)
            .await
            .map_err(|e| e.to_string())?;

        let mut remaining = len;
        let mut buf = vec![0u8; 1024 * 1024];
        let mut rng = rand_core::OsRng;
        while remaining > 0 {
            let n = remaining.min(buf.len());
            rng.fill_bytes(&mut buf[..n]);
            f.write_all(&buf[..n]).await.map_err(|e| e.to_string())?;
            remaining -= n;
        }

        f.flush().await.map_err(|e| e.to_string())?;
        buf.zeroize();

        // Drop headers + content by truncation and deletion.
        let _ = f.shutdown().await;
        let _ = tokio::fs::remove_file(&path).await;
        purged = purged.saturating_add(1);
    }

    Ok(purged)
}

pub fn log_lockdown(reason: &str) -> Result<(), String> {
    audit::append_line(
        "security_lockdown.log",
        &format!("security_lockdown reason={reason}"),
    )
}


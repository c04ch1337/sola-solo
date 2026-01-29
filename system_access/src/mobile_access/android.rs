use crate::mobile_access::{ConnectionMode, DeviceInfo, DeviceType, MobileError};
use log::debug;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct AndroidController {
    pub(crate) adb_path: PathBuf,
    pub(crate) scrcpy_path: Option<PathBuf>,
    pub(crate) current_device_id: Option<String>,
}

impl AndroidController {
    pub fn new(adb_path: PathBuf, scrcpy_path: Option<PathBuf>) -> Self {
        Self {
            adb_path,
            scrcpy_path,
            current_device_id: None,
        }
    }

    pub fn with_current_device(mut self, device_id: impl Into<String>) -> Self {
        self.current_device_id = Some(device_id.into());
        self
    }

    fn run_adb(&self, args: &[&str]) -> Result<String, MobileError> {
        let output = Command::new(&self.adb_path)
            .args(args)
            .output()
            .map_err(|e| MobileError::Subprocess(format!("adb spawn failed: {e}")))?;

        if !output.status.success() {
            return Err(MobileError::Subprocess(format!(
                "adb failed (exit={}): {}",
                output.status.code().unwrap_or(-1),
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn detect_devices(&self) -> Result<Vec<DeviceInfo>, MobileError> {
        // Example output:
        // List of devices attached
        // emulator-5554 device product:sdk_gphone model:sdk_gphone_x86_64 device:generic_x86_64 transport_id:1
        let out = self.run_adb(&["devices", "-l"])?;
        let mut devices = Vec::new();
        for line in out.lines().skip(1) {
            let l = line.trim();
            if l.is_empty() {
                continue;
            }
            let mut parts = l.split_whitespace();
            let Some(id) = parts.next() else { continue };
            let status = parts.next().unwrap_or("unknown").to_string();

            // Parse a best-effort model field.
            let mut model = String::new();
            for p in parts {
                if let Some(v) = p.strip_prefix("model:") {
                    model = v.to_string();
                }
            }

            devices.push(DeviceInfo {
                id: id.to_string(),
                model,
                type_: DeviceType::Android,
                status,
            });
        }
        Ok(devices)
    }

    pub fn connect_device(
        &mut self,
        device_id: &str,
        mode: ConnectionMode,
    ) -> Result<(), MobileError> {
        match mode {
            ConnectionMode::Usb => {
                // No-op beyond selecting the device; actual authorization is reflected in `adb devices`.
                self.current_device_id = Some(device_id.to_string());
                Ok(())
            }
            ConnectionMode::Wifi => {
                // For wifi, `device_id` should be ip:port.
                let _ = self.run_adb(&["connect", device_id])?;
                self.current_device_id = Some(device_id.to_string());
                Ok(())
            }
        }
    }

    pub fn enable_wifi_adb(&self, device_ip: &str) -> Result<(), MobileError> {
        let Some(id) = self.current_device_id.as_deref() else {
            return Err(MobileError::Config(
                "No Android device selected. Call connect() first".to_string(),
            ));
        };

        let _ = self.run_adb(&["-s", id, "tcpip", "5555"])?;
        let _ = self.run_adb(&["connect", &format!("{device_ip}:5555")])?;
        Ok(())
    }

    pub fn exec_shell(&self, cmd: &str) -> Result<String, MobileError> {
        let Some(id) = self.current_device_id.as_deref() else {
            return Err(MobileError::Config(
                "No Android device selected. Call connect() first".to_string(),
            ));
        };
        self.run_adb(&["-s", id, "shell", cmd])
    }

    pub fn pull(&self, remote: &str, local: &Path) -> Result<(), MobileError> {
        let Some(id) = self.current_device_id.as_deref() else {
            return Err(MobileError::Config(
                "No Android device selected. Call connect() first".to_string(),
            ));
        };
        let local_s = local
            .to_str()
            .ok_or_else(|| MobileError::Config("Local path is not valid UTF-8".to_string()))?;
        let _ = self.run_adb(&["-s", id, "pull", remote, local_s])?;
        Ok(())
    }

    pub fn capture_screen_to(&self, out_path: &Path) -> Result<(), MobileError> {
        let Some(id) = self.current_device_id.as_deref() else {
            return Err(MobileError::Config(
                "No Android device selected. Call connect() first".to_string(),
            ));
        };
        let output = Command::new(&self.adb_path)
            .args(["-s", id, "exec-out", "screencap", "-p"])
            .output()
            .map_err(|e| MobileError::Subprocess(format!("adb exec-out failed: {e}")))?;

        if !output.status.success() {
            return Err(MobileError::Subprocess(format!(
                "adb exec-out failed (exit={}): {}",
                output.status.code().unwrap_or(-1),
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(out_path, &output.stdout)?;
        debug!("Saved Android screenshot to {}", out_path.display());
        Ok(())
    }

    /// Starts scrcpy for the currently selected device.
    ///
    /// This is a detached spawn (best-effort). Dropping the child handle should leave scrcpy running.
    pub fn start_scrcpy_detached(&self) -> Result<(), MobileError> {
        let Some(scrcpy) = self.scrcpy_path.as_ref() else {
            return Err(MobileError::Config(
                "scrcpy_path not configured. Deploy scrcpy or set it in config".to_string(),
            ));
        };
        let Some(id) = self.current_device_id.as_deref() else {
            return Err(MobileError::Config(
                "No Android device selected. Call connect() first".to_string(),
            ));
        };

        let mut cmd = Command::new(scrcpy);
        cmd.arg("--serial").arg(id);

        // Conservative defaults for stability.
        cmd.arg("--no-audio");

        // Spawn and intentionally drop the handle.
        cmd.spawn()
            .map(|_child| ())
            .map_err(|e| MobileError::Subprocess(format!("scrcpy spawn failed: {e}")))
    }

    fn python_exe(python: Option<&Path>) -> PathBuf {
        python
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from(if cfg!(windows) { "python" } else { "python3" }))
    }

    /// Best-effort uiautomator2 bootstrap.
    ///
    /// This expects a working Python environment. It runs:
    /// - `python -m pip install -U uiautomator2`
    /// - `python -m uiautomator2 init --serial <id>`
    pub fn uiautomator2_init(&self, python: Option<&Path>) -> Result<(), MobileError> {
        let Some(id) = self.current_device_id.as_deref() else {
            return Err(MobileError::Config(
                "No Android device selected. Call connect() first".to_string(),
            ));
        };

        let py = Self::python_exe(python);

        let pip = Command::new(&py)
            .args(["-m", "pip", "install", "-U", "uiautomator2"])
            .output()
            .map_err(|e| MobileError::Subprocess(format!("python pip spawn failed: {e}")))?;
        if !pip.status.success() {
            return Err(MobileError::Subprocess(format!(
                "pip install uiautomator2 failed (exit={}): {}",
                pip.status.code().unwrap_or(-1),
                String::from_utf8_lossy(&pip.stderr)
            )));
        }

        let init = Command::new(&py)
            .args(["-m", "uiautomator2", "init", "--serial", id])
            .output()
            .map_err(|e| MobileError::Subprocess(format!("uiautomator2 init spawn failed: {e}")))?;
        if !init.status.success() {
            return Err(MobileError::Subprocess(format!(
                "uiautomator2 init failed (exit={}): {}",
                init.status.code().unwrap_or(-1),
                String::from_utf8_lossy(&init.stderr)
            )));
        }

        Ok(())
    }

    /// Dumps a UI hierarchy XML using uiautomator2 (best-effort).
    pub fn uiautomator2_dump_hierarchy(
        &self,
        python: Option<&Path>,
        out_path: &Path,
    ) -> Result<(), MobileError> {
        let Some(id) = self.current_device_id.as_deref() else {
            return Err(MobileError::Config(
                "No Android device selected. Call connect() first".to_string(),
            ));
        };

        let py = Self::python_exe(python);
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let output = Command::new(&py)
            .args(["-m", "uiautomator2", "dump", "--serial", id])
            .output()
            .map_err(|e| MobileError::Subprocess(format!("uiautomator2 dump spawn failed: {e}")))?;

        if !output.status.success() {
            return Err(MobileError::Subprocess(format!(
                "uiautomator2 dump failed (exit={}): {}",
                output.status.code().unwrap_or(-1),
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        // Some versions print the xml path; others print xml itself. Store stdout either way.
        std::fs::write(out_path, &output.stdout)?;
        Ok(())
    }
}

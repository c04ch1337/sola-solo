#[cfg(not(windows))]
use crate::mobile_access::setup;
use crate::mobile_access::{ConnectionMode, DeviceInfo, DeviceType, MobileError};
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct IosController {
    pub(crate) bin_dir: PathBuf,
    pub(crate) current_device_id: Option<String>,
}

impl IosController {
    pub fn new(bin_dir: PathBuf) -> Self {
        Self {
            bin_dir,
            current_device_id: None,
        }
    }

    fn tool(&self, name: &str) -> PathBuf {
        #[cfg(windows)]
        {
            self.bin_dir.join(format!("{name}.exe"))
        }
        #[cfg(not(windows))]
        {
            self.bin_dir.join(name)
        }
    }

    fn run(&self, tool: &str, args: &[&str]) -> Result<String, MobileError> {
        let exe = self.tool(tool);
        let output = Command::new(&exe)
            .args(args)
            .output()
            .map_err(|e| MobileError::Subprocess(format!("{tool} spawn failed: {e}")))?;
        if !output.status.success() {
            return Err(MobileError::Subprocess(format!(
                "{tool} failed (exit={}): {}",
                output.status.code().unwrap_or(-1),
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn detect_devices(&self) -> Result<Vec<DeviceInfo>, MobileError> {
        // libimobiledevice typically provides: idevice_id -l
        let out = self.run("idevice_id", &["-l"])?;
        let mut devices = Vec::new();
        for line in out.lines() {
            let id = line.trim();
            if id.is_empty() {
                continue;
            }
            let model = self
                .device_info_key(id, "ProductType")
                .unwrap_or_default()
                .trim()
                .to_string();
            devices.push(DeviceInfo {
                id: id.to_string(),
                model,
                type_: DeviceType::Ios,
                status: "connected".to_string(),
            });
        }
        Ok(devices)
    }

    pub fn device_info_key(&self, device_id: &str, key: &str) -> Result<String, MobileError> {
        // ideviceinfo -u <udid> -k <key>
        self.run("ideviceinfo", &["-u", device_id, "-k", key])
    }

    pub fn capture_screenshot(&self, out_path: &std::path::Path) -> Result<(), MobileError> {
        let Some(id) = self.current_device_id.as_deref() else {
            return Err(MobileError::Config(
                "No iOS device selected. Call connect() first".to_string(),
            ));
        };

        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // idevicescreenshot -u <udid> <path>
        let exe = self.tool("idevicescreenshot");
        let output = Command::new(&exe)
            .args(["-u", id, out_path.to_string_lossy().as_ref()])
            .output()
            .map_err(|e| MobileError::Subprocess(format!("idevicescreenshot spawn failed: {e}")))?;

        if !output.status.success() {
            return Err(MobileError::Subprocess(format!(
                "idevicescreenshot failed (exit={}): {}",
                output.status.code().unwrap_or(-1),
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        Ok(())
    }

    /// Best-effort file pull for iOS using ifuse (non-Windows only).
    ///
    /// This mounts the device via ifuse and copies the requested path.
    #[cfg(not(windows))]
    pub fn pull_file_ifuse(
        &self,
        remote_path: &str,
        local_path: &std::path::Path,
    ) -> Result<(), MobileError> {
        let Some(id) = self.current_device_id.as_deref() else {
            return Err(MobileError::Config(
                "No iOS device selected. Call connect() first".to_string(),
            ));
        };

        let mount_root = setup::data_dir()?.join("ios_mount").join(id);
        std::fs::create_dir_all(&mount_root)?;

        // Mount
        let status = Command::new("ifuse")
            .args(["--udid", id, mount_root.to_string_lossy().as_ref()])
            .status()
            .map_err(|e| MobileError::Subprocess(format!("ifuse spawn failed: {e}")))?;
        if !status.success() {
            return Err(MobileError::Subprocess(format!(
                "ifuse mount failed (exit={})",
                status.code().unwrap_or(-1)
            )));
        }

        let src = mount_root.join(remote_path.trim_start_matches('/'));
        if let Some(parent) = local_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::copy(&src, local_path)?;

        // Unmount (best-effort)
        let _ = Command::new("umount").arg(&mount_root).status();

        Ok(())
    }

    pub fn connect_device(
        &mut self,
        device_id: &str,
        _mode: ConnectionMode,
    ) -> Result<(), MobileError> {
        // iOS connections are typically USB + trust pairing handled by OS / libimobiledevice tools.
        self.current_device_id = Some(device_id.to_string());
        Ok(())
    }
}

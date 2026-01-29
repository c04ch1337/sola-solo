use crate::mobile_access::{
    android::AndroidController, ios::IosController, DeviceController, MobileError,
};
use crate::mobile_access::{setup, ConnectionMode};
use std::path::PathBuf;

impl DeviceController for AndroidController {
    fn detect(&self) -> Result<Vec<crate::mobile_access::DeviceInfo>, MobileError> {
        self.detect_devices()
    }

    fn connect(&mut self, device_id: &str, mode: ConnectionMode) -> Result<(), MobileError> {
        self.connect_device(device_id, mode)
    }

    fn execute_command(&self, cmd: &str) -> Result<String, MobileError> {
        self.exec_shell(cmd)
    }

    fn pull_file(&self, remote_path: &str, local_path: &str) -> Result<(), MobileError> {
        self.pull(remote_path, std::path::Path::new(local_path))
    }

    fn capture_screen(&self) -> Result<PathBuf, MobileError> {
        let base = setup::data_dir()?;
        let id = self
            .current_device_id
            .clone()
            .unwrap_or_else(|| "unknown-device".to_string());
        let out = base.join("screenshots").join(id).join("screen.png");
        self.capture_screen_to(&out)?;
        Ok(out)
    }
}

impl DeviceController for IosController {
    fn detect(&self) -> Result<Vec<crate::mobile_access::DeviceInfo>, MobileError> {
        self.detect_devices()
    }

    fn connect(&mut self, device_id: &str, mode: ConnectionMode) -> Result<(), MobileError> {
        self.connect_device(device_id, mode)
    }

    fn execute_command(&self, cmd: &str) -> Result<String, MobileError> {
        // Best-effort: support `info:<Key>` as a safe/read-only command.
        // Example: `info:DeviceName` or `info:ProductType`
        let cmd = cmd.trim();
        let Some(rest) = cmd.strip_prefix("info:") else {
            return Err(MobileError::Subprocess(
                "iOS execute_command supports only read-only ideviceinfo via `info:<Key>`"
                    .to_string(),
            ));
        };
        let Some(id) = self.current_device_id.as_deref() else {
            return Err(MobileError::Config(
                "No iOS device selected. Call connect() first".to_string(),
            ));
        };
        self.device_info_key(id, rest)
    }

    fn pull_file(&self, remote_path: &str, local_path: &str) -> Result<(), MobileError> {
        #[cfg(not(windows))]
        {
            self.pull_file_ifuse(remote_path, std::path::Path::new(local_path))
        }

        #[cfg(windows)]
        {
            let _ = remote_path;
            let _ = local_path;
            Err(MobileError::Subprocess(
                "iOS pull_file requires ifuse (not supported on Windows in this implementation)"
                    .to_string(),
            ))
        }
    }

    fn capture_screen(&self) -> Result<PathBuf, MobileError> {
        let base = setup::data_dir()?;
        let id = self
            .current_device_id
            .clone()
            .unwrap_or_else(|| "unknown-device".to_string());
        let out = base.join("screenshots").join(id).join("screen.png");
        self.capture_screenshot(&out)?;
        Ok(out)
    }
}

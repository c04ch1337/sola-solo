use crate::mobile_access::android::AndroidController;
use crate::mobile_access::ios::IosController;
use crate::mobile_access::security;
use crate::mobile_access::setup;
use crate::mobile_access::{Config, ConnectionMode, DeviceController, DeviceInfo, MobileError};
use log::{info, warn};
use std::path::PathBuf;

pub struct Orchestrator {
    pub android: Option<AndroidController>,
    pub ios: Option<IosController>,
    pub config: Config,
}

impl Default for Orchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl Orchestrator {
    pub fn new() -> Self {
        let mut config = setup::load_config().unwrap_or_default();

        // Default-on audit log for safety/traceability.
        if !config.audit_log_enabled {
            config.audit_log_enabled = true;
            let _ = setup::save_config(&config);
        }

        let android = match Self::init_android(&mut config) {
            Ok(c) => Some(c),
            Err(e) => {
                warn!("AndroidController unavailable: {e}");
                None
            }
        };

        let ios = match Self::init_ios(&config) {
            Ok(c) => Some(c),
            Err(e) => {
                warn!("IosController unavailable: {e}");
                None
            }
        };

        Self {
            android,
            ios,
            config,
        }
    }

    fn init_android(cfg: &mut Config) -> Result<AndroidController, MobileError> {
        let adb = setup::ensure_adb(cfg)?;
        let scrcpy = cfg.scrcpy_path.clone();
        Ok(AndroidController::new(adb, scrcpy))
    }

    fn init_ios(cfg: &Config) -> Result<IosController, MobileError> {
        let bin_dir = cfg.libimobiledevice_bin_dir.clone().ok_or_else(|| {
            MobileError::Config("libimobiledevice_bin_dir not configured".to_string())
        })?;
        Ok(IosController::new(bin_dir))
    }

    fn pick_first_authorized(
        devices: Vec<DeviceInfo>,
        cfg: &Config,
    ) -> Result<DeviceInfo, MobileError> {
        for d in devices {
            if cfg
                .authorized_devices
                .iter()
                .any(|id| id.eq_ignore_ascii_case(&d.id))
            {
                return Ok(d);
            }
        }
        Err(MobileError::Unauthorized)
    }

    fn backups_dir() -> Result<PathBuf, MobileError> {
        Ok(setup::data_dir()?.join("backups"))
    }

    pub async fn run_task(&self, task: &str) -> Result<String, MobileError> {
        let task_l = task.trim().to_ascii_lowercase();
        if task_l.is_empty() {
            return Err(MobileError::Config("Empty task".to_string()));
        }

        if (task_l.contains("pull") && task_l.contains("photos"))
            || task_l.contains("backup photos")
        {
            return self.run_backup_photos_android().await;
        }

        if task_l.contains("ios") && task_l.contains("screenshot") {
            return self.run_ios_screenshot().await;
        }

        if task_l.contains("screenshot") {
            return self.run_android_screenshot().await;
        }

        if task_l.contains("mirror") || task_l.contains("scrcpy") {
            return self.run_android_mirror().await;
        }

        if let Some(cmd) = task_l.strip_prefix("shell ") {
            return self.run_android_shell(cmd).await;
        }

        if task_l.contains("uia") && task_l.contains("init") {
            return self.run_android_uia_init().await;
        }

        if task_l.contains("uia") && task_l.contains("dump") {
            return self.run_android_uia_dump().await;
        }

        Err(MobileError::Config(format!(
            "Unknown task: {task}. Supported: pull photos | backup photos | screenshot | ios screenshot | mirror | shell <cmd> | uia init | uia dump"
        )))
    }

    async fn run_backup_photos_android(&self) -> Result<String, MobileError> {
        let mut android = self
            .android
            .clone()
            .ok_or_else(|| MobileError::Config("AndroidController not available".to_string()))?;

        let devices = android.detect()?;
        if devices.is_empty() {
            return Err(MobileError::NotFound);
        }

        let device = Self::pick_first_authorized(devices, &self.config)?;
        security::ensure_consent(&device.id)?;

        android.connect(&device.id, ConnectionMode::Usb)?;

        let out = Self::backups_dir()?.join(&device.id).join("DCIM");
        info!("Pulling /sdcard/DCIM -> {}", out.display());
        android.pull_file("/sdcard/DCIM", out.to_str().unwrap_or("DCIM"))?;

        Ok(format!(
            "Backed up Android photos from device {} to {}",
            device.id,
            out.display()
        ))
    }

    async fn run_android_screenshot(&self) -> Result<String, MobileError> {
        let mut android = self
            .android
            .clone()
            .ok_or_else(|| MobileError::Config("AndroidController not available".to_string()))?;

        let devices = android.detect()?;
        let device = Self::pick_first_authorized(devices, &self.config)?;
        security::ensure_consent(&device.id)?;
        android.connect(&device.id, ConnectionMode::Usb)?;

        let path = android.capture_screen()?;
        Ok(format!("Captured screenshot to {}", path.display()))
    }

    async fn run_android_mirror(&self) -> Result<String, MobileError> {
        let mut config = setup::load_config().unwrap_or_default();
        let mut android = self
            .android
            .clone()
            .ok_or_else(|| MobileError::Config("AndroidController not available".to_string()))?;

        // Ensure scrcpy is present for mirror.
        let scrcpy = setup::ensure_scrcpy(&mut config)?;
        android.scrcpy_path = Some(scrcpy);

        let devices = android.detect()?;
        let device = Self::pick_first_authorized(devices, &self.config)?;
        security::ensure_consent(&device.id)?;
        android.connect(&device.id, ConnectionMode::Usb)?;

        android.start_scrcpy_detached()?;
        Ok("Started scrcpy (detached)".to_string())
    }

    async fn run_android_shell(&self, cmd: &str) -> Result<String, MobileError> {
        let mut android = self
            .android
            .clone()
            .ok_or_else(|| MobileError::Config("AndroidController not available".to_string()))?;

        let devices = android.detect()?;
        let device = Self::pick_first_authorized(devices, &self.config)?;
        security::ensure_consent(&device.id)?;
        android.connect(&device.id, ConnectionMode::Usb)?;

        let out = android.execute_command(cmd)?;
        Ok(out)
    }

    async fn run_android_uia_init(&self) -> Result<String, MobileError> {
        let cfg = setup::load_config().unwrap_or_default();
        if !cfg.uiautomator2_enabled {
            return Err(MobileError::Config(
                "uiautomator2 is disabled in config (set uiautomator2_enabled=true)".to_string(),
            ));
        }

        let python = cfg.python_path.clone();

        let mut android = self
            .android
            .clone()
            .ok_or_else(|| MobileError::Config("AndroidController not available".to_string()))?;
        let devices = android.detect()?;
        let device = Self::pick_first_authorized(devices, &self.config)?;
        security::ensure_consent(&device.id)?;
        android.connect(&device.id, ConnectionMode::Usb)?;

        android.uiautomator2_init(python.as_deref())?;
        Ok("uiautomator2 initialized".to_string())
    }

    async fn run_android_uia_dump(&self) -> Result<String, MobileError> {
        let cfg = setup::load_config().unwrap_or_default();
        if !cfg.uiautomator2_enabled {
            return Err(MobileError::Config(
                "uiautomator2 is disabled in config (set uiautomator2_enabled=true)".to_string(),
            ));
        }
        let python = cfg.python_path.clone();

        let mut android = self
            .android
            .clone()
            .ok_or_else(|| MobileError::Config("AndroidController not available".to_string()))?;
        let devices = android.detect()?;
        let device = Self::pick_first_authorized(devices, &self.config)?;
        security::ensure_consent(&device.id)?;
        android.connect(&device.id, ConnectionMode::Usb)?;

        let out_path = setup::data_dir()?
            .join("uiautomator2")
            .join(&device.id)
            .join("dump.txt");
        android.uiautomator2_dump_hierarchy(python.as_deref(), &out_path)?;
        Ok(format!("uiautomator2 dump saved to {}", out_path.display()))
    }

    async fn run_ios_screenshot(&self) -> Result<String, MobileError> {
        let mut ios = self
            .ios
            .clone()
            .ok_or_else(|| MobileError::Config("IosController not available".to_string()))?;

        let devices = ios.detect()?;
        if devices.is_empty() {
            return Err(MobileError::NotFound);
        }
        let device = Self::pick_first_authorized(devices, &self.config)?;
        security::ensure_consent(&device.id)?;
        ios.connect(&device.id, ConnectionMode::Usb)?;

        let path = ios.capture_screen()?;
        Ok(format!("Captured iOS screenshot to {}", path.display()))
    }
}

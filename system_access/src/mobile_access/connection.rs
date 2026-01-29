use crate::mobile_access::{
    android::AndroidController, ios::IosController, DeviceEvent, DeviceEventKind, DeviceInfo,
    MobileError,
};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;

impl AndroidController {
    /// Best-effort detection hook.
    ///
    /// This is a synchronous placeholder; callers that need real-time monitoring should poll
    /// in an async task.
    pub fn monitor_usb(&self) -> Result<DeviceInfo, MobileError> {
        let devices = self.detect_devices()?;
        devices.into_iter().next().ok_or(MobileError::NotFound)
    }
}

impl IosController {
    pub fn monitor_usb(&self) -> Result<DeviceInfo, MobileError> {
        let devices = self.detect_devices()?;
        devices.into_iter().next().ok_or(MobileError::NotFound)
    }
}

/// Spawns an async polling task that emits device add/remove/change events.
///
/// This uses `adb devices -l` polling, which is the most portable approach across OSes.
pub fn spawn_android_device_monitor(
    controller: AndroidController,
    poll_interval: Duration,
) -> mpsc::Receiver<DeviceEvent> {
    let (tx, rx) = mpsc::channel(64);

    tokio::spawn(async move {
        let mut prev: HashMap<String, DeviceInfo> = HashMap::new();
        let mut interval = tokio::time::interval(poll_interval);

        loop {
            interval.tick().await;

            let next_list = match controller.detect_devices() {
                Ok(d) => d,
                Err(_) => continue,
            };
            let mut next: HashMap<String, DeviceInfo> = HashMap::new();
            for d in next_list {
                next.insert(d.id.clone(), d);
            }

            // Added/changed
            for (id, dev) in next.iter() {
                match prev.get(id) {
                    None => {
                        let _ = tx
                            .send(DeviceEvent {
                                kind: DeviceEventKind::Added,
                                device: dev.clone(),
                            })
                            .await;
                    }
                    Some(old) if old.status != dev.status || old.model != dev.model => {
                        let _ = tx
                            .send(DeviceEvent {
                                kind: DeviceEventKind::Changed,
                                device: dev.clone(),
                            })
                            .await;
                    }
                    _ => {}
                }
            }

            // Removed
            for (id, old) in prev.iter() {
                if !next.contains_key(id) {
                    let _ = tx
                        .send(DeviceEvent {
                            kind: DeviceEventKind::Removed,
                            device: old.clone(),
                        })
                        .await;
                }
            }

            prev = next;
        }
    });

    rx
}

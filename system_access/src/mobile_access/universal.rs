//! Cross-device file transfer helpers.
//!
//! Note: MTP/PTP is platform- and driver-dependent. This module is intentionally minimal for now.

use crate::mobile_access::MobileError;

pub fn not_implemented(feature: &str) -> Result<(), MobileError> {
    Err(MobileError::Subprocess(format!(
        "{feature} is not implemented (MTP/PTP support TBD)"
    )))
}

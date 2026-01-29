use serde::{Deserialize, Serialize};

use super::zodiac::ZodiacSign;

/// Minimal Persona model for L7 archetype binding.
///
/// This is intentionally small: the actual persona payload (memories, assets) lives
/// in the vault/memory subsystems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Persona {
    pub id: String,
    pub display_name: String,
    pub zodiac_sign: ZodiacSign,
}


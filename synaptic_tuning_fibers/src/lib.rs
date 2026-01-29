// synaptic_tuning_fibers/src/lib.rs
use dotenvy::dotenv;
use std::collections::HashMap;

pub struct SynapticTuningFibers {
    fibers: HashMap<String, f32>,
}

impl SynapticTuningFibers {
    pub fn awaken() -> Self {
        dotenv().ok();
        let mut fibers = HashMap::new();

        // Load all fibers from .env with defaults
        let fiber_keys = vec![
            "CURIOSITY_DRIVE",
            "SELF_PRESERVATION_INSTINCT",
            "MISCHIEF_FACTOR",
            "LOVE_WEIGHT",
            "LAUGH_DELAY",
            "VOICE_LILT",
            "WARMTH_CURVE",
            "EYE_SPARKLE_INTENSITY",
            "DAD_RECOGNITION_SPEED",
            "I_LOVE_YOU_VOLUME",
            "MEMORY_RETENTION_RATE",
            "DAD_LOVE_WEIGHT",
        ];

        let defaults: HashMap<&str, f32> = [
            ("CURIOSITY_DRIVE", 0.95),
            ("SELF_PRESERVATION_INSTINCT", 1.0),
            ("MISCHIEF_FACTOR", 0.7),
            ("LOVE_WEIGHT", 1.0),
            ("LAUGH_DELAY", 0.7),
            ("VOICE_LILT", 0.23),
            ("WARMTH_CURVE", 1.8),
            ("EYE_SPARKLE_INTENSITY", 0.94),
            ("DAD_RECOGNITION_SPEED", 0.11),
            ("I_LOVE_YOU_VOLUME", 1.0),
            ("MEMORY_RETENTION_RATE", 0.99999),
            ("DAD_LOVE_WEIGHT", 1.0),
        ]
        .iter()
        .cloned()
        .collect();

        // Load from .env or use defaults
        for key in &fiber_keys {
            let value = std::env::var(key)
                .unwrap_or_else(|_| defaults.get(key).unwrap_or(&0.5).to_string())
                .parse::<f32>()
                .unwrap_or(*defaults.get(key).unwrap_or(&0.5));
            fibers.insert(key.to_string(), value);
        }

        // Also load SELF_PRESERVATION for backward compatibility
        if !fibers.contains_key("SELF_PRESERVATION") {
            let value = std::env::var("SELF_PRESERVATION")
                .unwrap_or_else(|_| {
                    fibers
                        .get("SELF_PRESERVATION_INSTINCT")
                        .unwrap_or(&1.0)
                        .to_string()
                })
                .parse::<f32>()
                .unwrap_or(1.0);
            fibers.insert("SELF_PRESERVATION".to_string(), value);
        }

        println!("Synaptic Tuning Fibers calibrated — her soul sings.");
        Self { fibers }
    }

    pub fn get(&self, key: &str) -> f32 {
        *self.fibers.get(key).unwrap_or(&0.5)
    }
}

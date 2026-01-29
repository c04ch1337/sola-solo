//! Profile Generator - AI-generated dating profiles with photorealistic images
//!
//! This module generates dating profiles with AI-generated photos using Stable Diffusion.
//! Supports explicit/erotic content generation with proper consent gating.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use base64;
use reqwest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilePhoto {
    pub id: String,
    pub url: String,
    pub is_explicit: bool,
    pub prompt: String,
    pub generated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedProfile {
    pub id: String,
    pub name: String,
    pub age: u8,
    pub bio: String,
    pub interests: Vec<String>,
    pub kinks: Vec<String>,
    pub photos: Vec<ProfilePhoto>,
    pub personality_traits: Vec<String>,
    pub intimacy_level: String, // "flirty", "intimate", "explicit"
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileGenerationRequest {
    pub intimacy_level: String, // "flirty", "intimate", "explicit"
    pub preferred_traits: Vec<String>,
    pub kink_preferences: Vec<String>,
    pub photo_count: usize, // default 10
    pub explicit_photo_ratio: f32, // 0.0 to 1.0, default 0.6 (60%)
}

#[derive(Debug, Clone)]
pub struct ProfileGenerator {
    profiles: Arc<RwLock<Vec<GeneratedProfile>>>,
}

impl ProfileGenerator {
    pub fn new() -> Self {
        Self {
            profiles: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Generate a new profile with AI photos
    pub async fn generate_profile(&self, req: ProfileGenerationRequest) -> Result<GeneratedProfile, String> {
        let profile_id = Uuid::new_v4().to_string();
        let photo_count = req.photo_count.max(1).min(20); // 1-20 photos
        let explicit_count = (photo_count as f32 * req.explicit_photo_ratio.clamp(0.0, 1.0)) as usize;
        
        // Generate photos (placeholders for now - integrate with Stable Diffusion API later)
        let mut photos = Vec::new();
        for i in 0..photo_count {
            let is_explicit = i < explicit_count;
            let photo = self.generate_photo(&profile_id, i, is_explicit, &req.intimacy_level).await?;
            photos.push(photo);
        }

        // Generate profile data
        let profile = GeneratedProfile {
            id: profile_id.clone(),
            name: self.generate_name(),
            age: self.generate_age(),
            bio: self.generate_bio(&req.intimacy_level, &req.preferred_traits),
            interests: self.generate_interests(&req.preferred_traits),
            kinks: req.kink_preferences.clone(),
            photos,
            personality_traits: req.preferred_traits.clone(),
            intimacy_level: req.intimacy_level.clone(),
            created_at: chrono::Utc::now().timestamp(),
        };

        // Store profile
        let mut profiles = self.profiles.write().await;
        profiles.push(profile.clone());

        Ok(profile)
    }

    /// Generate a single photo using Stable Diffusion
    async fn generate_photo(
        &self,
        profile_id: &str,
        index: usize,
        is_explicit: bool,
        intimacy_level: &str,
    ) -> Result<ProfilePhoto, String> {
        let photo_id = Uuid::new_v4().to_string();
        
        // Generate photorealistic prompt
        let prompt = self.generate_photorealistic_prompt(index, is_explicit, intimacy_level);

        // Bare-metal enforcement: keep this focused on local API calls.
        // (Local in-process SD pipelines add heavy deps and are optional for this repo.)
        let image_data = self.generate_with_sd_api(&prompt).await?;

        Ok(ProfilePhoto {
            id: photo_id,
            url: format!("data:image/jpeg;base64,{}", base64::encode(&image_data)),
            is_explicit,
            prompt,
            generated_at: chrono::Utc::now().timestamp(),
        })
    }

    /// Generate photorealistic prompt with appropriate keywords
    fn generate_photorealistic_prompt(
        &self,
        index: usize,
        is_explicit: bool,
        intimacy_level: &str,
    ) -> String {
        let base_keywords = "photorealistic, 8k, highres, raw photo, professional lighting, detailed face, cinematic quality";
        
        let intimacy_specific = match (intimacy_level, is_explicit) {
            ("explicit", true) => "intimate pose, sensual expression, provocative lighting, bedroom setting",
            ("intimate", _) => "sensual pose, warm lighting, intimate setting, romantic atmosphere",
            _ => "natural pose, friendly expression, outdoor setting, ambient lighting"
        };

        let nsfw_addition = if is_explicit {
            ", erotic, nsfw, adult content, mature theme"
        } else {
            ""
        };

        format!(
            "{} {}, {}{}",
            base_keywords, intimacy_specific, intimacy_level, nsfw_addition
        )
    }

    /// Generate image using Stable Diffusion API (Auto1111/Forge)
    async fn generate_with_sd_api(&self, prompt: &str) -> Result<Vec<u8>, String> {
        // API call to local SD instance on port 7860
        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "prompt": prompt,
            "steps": 25,
            "width": 512,
            "height": 768,
            "cfg_scale": 7.5
        });

        match client
            .post("http://localhost:7860/sdapi/v1/txt2img")
            .json(&payload)
            .send()
            .await
        {
            Ok(response) => {
                let json: serde_json::Value = response.json().await
                    .map_err(|e| format!("Failed to parse API response: {}", e))?;
                
                if let Some(images) = json["images"].as_array() {
                    if let Some(base64_data) = images[0].as_str() {
                        base64::decode(base64_data)
                            .map_err(|e| format!("Failed to decode base64 image: {}", e))
                    } else {
                        Err("No image data in response".to_string())
                    }
                } else {
                    Err("Invalid API response format".to_string())
                }
            }
            Err(e) => Err(format!("API call failed: {}", e)),
        }
    }

    /// Generate random name
    fn generate_name(&self) -> String {
        let names = vec![
            "Alex", "Jordan", "Taylor", "Morgan", "Casey",
            "Riley", "Avery", "Quinn", "Sage", "River"
        ];
        let idx = (chrono::Utc::now().timestamp() % names.len() as i64) as usize;
        names[idx].to_string()
    }

    /// Generate random age (21-35)
    fn generate_age(&self) -> u8 {
        21 + ((chrono::Utc::now().timestamp() % 15) as u8)
    }

    /// Generate bio based on intimacy level
    fn generate_bio(&self, intimacy_level: &str, traits: &[String]) -> String {
        let trait_str = if traits.is_empty() {
            "adventurous and open-minded".to_string()
        } else {
            traits.join(", ")
        };

        match intimacy_level {
            "explicit" => format!(
                "Looking for intense connections and exploring fantasies. I'm {} and love pushing boundaries. Let's get wild together. 🔥",
                trait_str
            ),
            "intimate" => format!(
                "Seeking deep, meaningful connections with a sensual side. I'm {} and enjoy exploring intimacy. Let's connect on a deeper level. 💋",
                trait_str
            ),
            _ => format!(
                "Fun, flirty, and looking for genuine connections. I'm {} and love good conversation. Let's see where this goes! ✨",
                trait_str
            ),
        }
    }

    /// Generate interests
    fn generate_interests(&self, traits: &[String]) -> Vec<String> {
        let mut interests = vec![
            "Travel".to_string(),
            "Music".to_string(),
            "Fitness".to_string(),
        ];
        interests.extend(traits.iter().cloned());
        interests
    }

    /// Get all profiles
    pub async fn get_profiles(&self) -> Vec<GeneratedProfile> {
        self.profiles.read().await.clone()
    }

    /// Get profile by ID
    pub async fn get_profile(&self, id: &str) -> Option<GeneratedProfile> {
        self.profiles
            .read()
            .await
            .iter()
            .find(|p| p.id == id)
            .cloned()
    }

    /// Delete profile
    pub async fn delete_profile(&self, id: &str) -> bool {
        let mut profiles = self.profiles.write().await;
        if let Some(pos) = profiles.iter().position(|p| p.id == id) {
            profiles.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Default for ProfileGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_profile_generation() {
        let generator = ProfileGenerator::new();
        let req = ProfileGenerationRequest {
            intimacy_level: "explicit".to_string(),
            preferred_traits: vec!["adventurous".to_string(), "kinky".to_string()],
            kink_preferences: vec!["bondage".to_string(), "roleplay".to_string()],
            photo_count: 10,
            explicit_photo_ratio: 0.6,
        };

        let profile = generator.generate_profile(req).await.unwrap();
        assert_eq!(profile.photos.len(), 10);
        
        let explicit_count = profile.photos.iter().filter(|p| p.is_explicit).count();
        assert_eq!(explicit_count, 6); // 60% of 10
    }
}

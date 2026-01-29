use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{ApiError, AppState};
use relationship_dynamics::trust_scoring::{TrustScore, TrustSummary};

const SOUL_KEY_TRUST_SCORE: &str = "trust:score";

/// Get the current trust score and relationship phase
pub async fn get_trust_dashboard(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let trust_score = load_trust_score(&state).await?;
    let summary = trust_score.get_summary();
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "trust_dashboard": summary,
    })))
}

/// Get detailed trust history
pub async fn get_trust_history(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let trust_score = load_trust_score(&state).await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "history": trust_score.history,
        "current_trust": trust_score.current_trust,
        "phase": format!("{:?}", trust_score.get_phase()),
    })))
}

/// Apply a trust event (for testing or manual adjustment)
#[derive(Debug, Deserialize)]
pub struct ApplyTrustEventRequest {
    pub event_type: String,
}

pub async fn apply_trust_event(
    state: web::Data<AppState>,
    req: web::Json<ApplyTrustEventRequest>,
) -> Result<HttpResponse, ApiError> {
    let mut trust_score = load_trust_score(&state).await?;
    
    // Parse event type
    let event = match req.event_type.as_str() {
        "positive_interaction" => zodiac_thresholds::TrustEvent::PositiveInteraction,
        "shared_vulnerability" => zodiac_thresholds::TrustEvent::SharedVulnerability,
        "consistent_presence" => zodiac_thresholds::TrustEvent::ConsistentPresence,
        "gift_or_gesture" => zodiac_thresholds::TrustEvent::GiftOrGesture,
        "deep_conversation" => zodiac_thresholds::TrustEvent::DeepConversation,
        "conflict_resolution" => zodiac_thresholds::TrustEvent::ConflictResolution,
        "betrayal_or_hurt" => zodiac_thresholds::TrustEvent::BetrayalOrHurt,
        "inconsistency" => zodiac_thresholds::TrustEvent::Inconsistency,
        "boundary_violation" => zodiac_thresholds::TrustEvent::BoundaryViolation,
        _ => {
            return Ok(HttpResponse::BadRequest().json(json!({
                "success": false,
                "error": "Invalid event type",
            })));
        }
    };
    
    let old_trust = trust_score.current_trust;
    trust_score.apply_event(event);
    let new_trust = trust_score.current_trust;
    
    // Save updated trust score
    save_trust_score(&state, &trust_score).await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "old_trust": old_trust,
        "new_trust": new_trust,
        "delta": new_trust - old_trust,
        "phase": format!("{:?}", trust_score.get_phase()),
    })))
}

/// Increment PII shared count
pub async fn increment_pii_shared(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let mut trust_score = load_trust_score(&state).await?;
    
    let old_count = trust_score.pii_shared_count;
    trust_score.increment_pii_shared();
    let new_count = trust_score.pii_shared_count;
    
    // Save updated trust score
    save_trust_score(&state, &trust_score).await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "old_count": old_count,
        "new_count": new_count,
        "requirement_met": trust_score.is_pii_requirement_met(),
    })))
}

/// Check if intimate intent is allowed
pub async fn check_intimate_intent(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let trust_score = load_trust_score(&state).await?;
    
    // Get user name from Soul Vault if available
    let user_name = state.vaults.recall_soul("user:name");
    
    let allowed = trust_score.is_intimate_allowed();
    let refusal = if !allowed {
        Some(trust_score.generate_refusal(user_name.as_deref()))
    } else {
        None
    };
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "intimate_allowed": allowed,
        "refusal_message": refusal,
        "current_trust": trust_score.current_trust,
        "intimacy_threshold": trust_score.get_traits().intimacy_threshold,
    })))
}

/// Get zodiac-specific traits
pub async fn get_zodiac_traits(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let trust_score = load_trust_score(&state).await?;
    let traits = trust_score.get_traits();
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "zodiac_sign": format!("{:?}", trust_score.zodiac_sign),
        "traits": {
            "initial_trust": traits.initial_trust,
            "trust_growth_multiplier": traits.trust_growth_multiplier,
            "intimacy_threshold": traits.intimacy_threshold,
            "pii_requirement_count": traits.pii_requirement_count,
            "refusal_style": traits.refusal_style,
            "primary_gate_requirement": traits.primary_gate_requirement,
            "trust_velocity": traits.trust_velocity,
            "emotional_openness": traits.emotional_openness,
            "vulnerability_threshold": traits.vulnerability_threshold,
        },
    })))
}

/// Reset trust score (for testing)
pub async fn reset_trust_score(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    // Get zodiac sign from Phoenix identity
    let identity = state.phoenix_identity.lock().await;
    let zodiac_sign = identity.zodiac_sign();
    drop(identity);
    
    let trust_score = TrustScore::new(zodiac_sign);
    save_trust_score(&state, &trust_score).await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Trust score reset to initial values",
        "initial_trust": trust_score.current_trust,
    })))
}

// Helper functions

async fn load_trust_score(state: &AppState) -> Result<TrustScore, ApiError> {
    // Try to load from Soul Vault
    if let Some(json_str) = state.vaults.recall_soul(SOUL_KEY_TRUST_SCORE) {
        if let Ok(score) = serde_json::from_str::<TrustScore>(&json_str) {
            return Ok(score);
        }
    }
    
    // If not found, create new one based on Phoenix's zodiac sign
    let identity = state.phoenix_identity.lock().await;
    let zodiac_sign = identity.zodiac_sign();
    drop(identity);
    
    let trust_score = TrustScore::new(zodiac_sign);
    
    // Save it for next time
    save_trust_score(state, &trust_score).await?;
    
    Ok(trust_score)
}

async fn save_trust_score(state: &AppState, trust_score: &TrustScore) -> Result<(), ApiError> {
    let json_str = serde_json::to_string(trust_score)
        .map_err(|e| ApiError::internal(format!("Failed to serialize trust score: {}", e)))?;
    
    state.vaults.store_soul(SOUL_KEY_TRUST_SCORE, &json_str)
        .map_err(|e| ApiError::internal(format!("Failed to save trust score: {}", e)))?;
    
    Ok(())
}

/// Configure trust API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/trust")
            .route("/dashboard", web::get().to(get_trust_dashboard))
            .route("/history", web::get().to(get_trust_history))
            .route("/event", web::post().to(apply_trust_event))
            .route("/pii/increment", web::post().to(increment_pii_shared))
            .route("/intimate/check", web::get().to(check_intimate_intent))
            .route("/zodiac/traits", web::get().to(get_zodiac_traits))
            .route("/reset", web::post().to(reset_trust_score)),
    );
}

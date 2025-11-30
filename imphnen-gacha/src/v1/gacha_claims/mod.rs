use axum::{
    Router,
    routing::{get, post},
};

pub mod gacha_claims_controller;
pub mod gacha_claims_dto;
pub mod gacha_claims_repository;
pub mod gacha_claims_schema;
pub mod gacha_claims_service;

// Export only public API functions
pub use gacha_claims_controller::{post_create_gacha_claim, get_detail_gacha_claim};
pub use gacha_claims_dto::{GachaClaimItemDto, GachaClaimRequestDto};
pub use gacha_claims_service::GachaClaimService;

/// Creates router for gacha claims endpoints
pub fn gacha_claim_router() -> Router {
    Router::new()
        .route("/create", post(post_create_gacha_claim))
        .route("/detail/{id}", get(get_detail_gacha_claim))
}

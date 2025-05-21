use axum::{
	Router,
	routing::{get, post},
};

pub mod gacha_claims_controller;
pub mod gacha_claims_dto;
pub mod gacha_claims_repository;
pub mod gacha_claims_schema;
pub mod gacha_claims_service;

pub use gacha_claims_controller::*;
pub use gacha_claims_dto::*;
pub use gacha_claims_repository::*;
pub use gacha_claims_schema::*;
pub use gacha_claims_service::*;

pub fn gacha_claim_router() -> Router {
	Router::new()
		.route("/create", post(post_create_gacha_claim))
		.route("/detail/{id}", get(get_detail_gacha_claim))
}

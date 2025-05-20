use axum::{
	Router,
	routing::{get, post},
};

pub mod gacha_claim_controller;
pub mod gacha_claim_dto;
pub mod gacha_claim_repository;
pub mod gacha_claim_schema;
pub mod gacha_claim_service;

pub use gacha_claim_controller::*;
pub use gacha_claim_dto::*;
pub use gacha_claim_repository::*;
pub use gacha_claim_schema::*;
pub use gacha_claim_service::*;

pub fn gacha_claim_router() -> Router {
	Router::new()
		.route("/create", post(post_create_gacha_claim))
		.route("/detail/{id}", get(get_detail_gacha_claim))
}

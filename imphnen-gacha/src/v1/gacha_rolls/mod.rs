pub mod gacha_rolls_controller;
pub mod gacha_rolls_dto;
pub mod gacha_rolls_repository;
pub mod gacha_rolls_schema;
pub mod gacha_rolls_service;

use axum::{
	Router,
	routing::{get, post},
};
pub use gacha_rolls_controller::*;
pub use gacha_rolls_dto::*;
pub use gacha_rolls_repository::*;
pub use gacha_rolls_schema::*;
pub use gacha_rolls_service::*;

pub fn gacha_roll_router() -> Router {
	Router::new()
		.route("/create", post(post_create_gacha_roll))
		.route("/execute", post(post_execute_gacha_roll))
		.route("/detail/{id}", get(get_detail_gacha_roll))
}

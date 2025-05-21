use axum::{
	Router,
	routing::{delete, get, post, put},
};

pub mod gacha_items_controller;
pub mod gacha_items_dto;
pub mod gacha_items_repository;
pub mod gacha_items_schema;
pub mod gacha_items_service;

pub use gacha_items_controller::*;
pub use gacha_items_dto::*;
pub use gacha_items_repository::*;
pub use gacha_items_schema::*;
pub use gacha_items_service::*;

pub fn gacha_item_router() -> Router {
	Router::new()
		.route("/", get(get_gacha_item_list))
		.route("/create", post(post_create_gacha_item))
		.route("/detail/{id}", get(get_gacha_item_by_id))
		.route("/update/{id}", put(put_update_gacha_item))
		.route("/delete/{id}", delete(delete_gacha_item))
}

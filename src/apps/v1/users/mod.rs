use axum::{
	routing::{delete, get, post, put},
	Router,
};

pub mod user_controller;
pub mod user_repository;
pub mod user_service;
pub mod users_dto;
pub mod users_schema;

pub use users_dto::*;

pub fn user_router() -> Router {
	Router::new()
		.route("/", get(user_controller::get_user))
		.route("/", post(user_controller::post_create_user))
		.route("/", put(user_controller::put_user))
		.route("/", delete(user_controller::delete_user))
		.route("/{mail}", get(user_controller::get_user_by_email))
}

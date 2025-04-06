use axum::{
	routing::{delete, get, post, put},
	Router,
};
pub mod permissions_controller;
pub mod permissions_dto;
pub mod permissions_enum;
pub mod permissions_guard;
pub mod permissions_repository;
pub mod permissions_schema;
pub mod permissions_service;

#[cfg(test)]
pub mod permissions_controller_test;
#[cfg(test)]
pub mod permissions_repository_test;

pub use permissions_controller::*;
pub use permissions_dto::*;
pub use permissions_enum::*;
pub use permissions_guard::*;
pub use permissions_repository::*;
pub use permissions_schema::*;

pub fn permissions_router() -> Router {
	Router::new()
		.route("/", get(get_permission_list))
		.route("/create", post(post_create_permission))
		.route("/detail/{id}", get(get_permission_by_id))
		.route("/update/{id}", put(put_update_permission))
		.route("/delete/{id}", delete(delete_permission))
}

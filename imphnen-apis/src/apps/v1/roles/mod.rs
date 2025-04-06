use axum::{
	routing::{delete, get, post, put},
	Router,
};

pub mod roles_controller;
pub mod roles_dto;
pub mod roles_enum;
pub mod roles_repository;
pub mod roles_schema;
pub mod roles_service;

#[cfg(test)]
pub mod roles_controller_test;
#[cfg(test)]
pub mod roles_repository_test;

pub use roles_controller::*;
pub use roles_dto::*;
pub use roles_enum::*;
pub use roles_repository::*;
pub use roles_schema::*;
pub use roles_service::*;

pub fn roles_router() -> Router {
	Router::new()
		.route("/", get(get_role_list))
		.route("/detail/{id}", get(get_role_by_id))
		.route("/create", post(post_create_role))
		.route("/update/{id}", put(put_update_role))
		.route("/delete/{id}", delete(delete_role))
}

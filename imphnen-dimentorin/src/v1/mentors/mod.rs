use axum::{
	Router,
	routing::{delete, get, post, put},
};

pub mod mentors_controller;
pub mod mentors_dto;
pub mod mentors_repository;
pub mod mentors_schema;
pub mod mentors_service;

pub use mentors_controller::*;
pub use mentors_dto::*;
pub use mentors_repository::*;
pub use mentors_schema::*;
pub use mentors_service::*;

pub fn mentors_router() -> Router {
	Router::new()
		.route("/", get(get_mentor_list))
		.route("/register", post(post_register_mentor))
		.route("/me", get(get_mentor_me))
		.route("/update/me", put(put_update_mentor_me))
		.route("/status", get(get_mentor_status))
		.route("/detail/{id}", get(get_mentor_by_id))
		.route("/update/{id}", put(put_update_mentor))
		.route("/update", put(put_update_mentor_no_id))
		.route("/delete/{id}", delete(delete_mentor))
		.route("/verify/{id}", put(put(put_verify_mentor)))
}

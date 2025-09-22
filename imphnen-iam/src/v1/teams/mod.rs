pub mod teams_controller;
pub mod teams_dto;
pub mod teams_repository;
pub mod teams_schema;
pub mod teams_service;

pub use teams_controller::*;
pub use teams_dto::*;
pub use teams_repository::*;
pub use teams_schema::*;
pub use teams_service::*;

use axum::{
	routing::{delete, get, post, put},
	Router,
};

pub fn teams_router() -> Router {
	Router::new()
		.route("/", get(get_team_list))
		.route("/create", post(post_create_team))
		.route("/detail/{id}", get(get_team_by_id))
		.route("/update/{id}", put(put_update_team))
		.route("/delete/{id}", delete(delete_team))
		.route("/{id}/invite", post(post_invite_team_members))
		.route("/accept/{token}", post(post_accept_invitation))
		.route("/search", get(get_public_team_search))
		.route("/{id}/members", get(get_team_members))
		.route("/{id}/leave", post(post_leave_team))
}
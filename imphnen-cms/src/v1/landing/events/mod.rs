pub mod events_dto;
pub mod events_schema;
pub mod events_repository;
pub mod events_service;
pub mod events_controller;

use axum::{routing::{delete, get, patch, post}, Router};
pub use events_controller::*;

pub fn events_public_routes() -> Router {
	Router::new()
		.route("/events", get(events_controller::get_event_list))
		.route("/events/detail/{id}", get(events_controller::get_event_by_id))
		.route("/events/create", post(events_controller::post_create_event))
		.route("/events/update/{id}", patch(events_controller::patch_update_event))
		.route("/events/delete/{id}", delete(events_controller::delete_event))
}

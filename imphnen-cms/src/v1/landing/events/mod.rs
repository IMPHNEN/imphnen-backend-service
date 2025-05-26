use axum::{
	Router,
	routing::{delete, get, patch, post},
};

pub mod events_controller;
pub mod events_dto;
pub mod events_repository;
pub mod events_schema;
pub mod events_service;

pub use events_controller::*;
pub use events_dto::*;
pub use events_repository::*;
pub use events_schema::*;
pub use events_service::*;

pub fn events_public_routes() -> Router {
	Router::new()
		.route(
			"/cms/landing/events",
			get(events_controller::get_event_list),
		)
		.route(
			"/cms/landing/events/detail/{id}",
			get(events_controller::get_event_by_id),
		)
}

pub fn events_protected_routes() -> Router {
	Router::new()
		.route(
			"/cms/landing/events/create",
			post(events_controller::post_create_event),
		)
		.route(
			"/cms/landing/events/update/{id}",
			patch(events_controller::patch_update_event),
		)
		.route(
			"/cms/landing/events/delete/{id}",
			delete(events_controller::delete_event),
		)
}

use axum::{
	Router,
	routing::{delete, get, patch, post},
};

pub mod events_controller;
pub mod events_dto;
pub mod events_repository;
pub mod events_schema;
pub mod events_service;

// Export only the necessary public items
pub use events_dto::{
	EventsCreateRequestDto,
	EventsUpdateRequestDto,
	EventsListItemDto,
	EventsDetailItemDto,
};
pub use events_controller::{
	get_event_list,
	get_event_by_id,
	post_create_event,
	patch_update_event,
	delete_event,
};

pub fn events_public_routes() -> Router {
	Router::new()
		.route("/cms/landing/events", get(get_event_list))
		.route("/cms/landing/events/detail/{id}", get(get_event_by_id))
}

pub fn events_protected_routes() -> Router {
	Router::new()
		.route("/cms/landing/events/create", post(post_create_event))
		.route("/cms/landing/events/update/{id}", patch(patch_update_event))
		.route("/cms/landing/events/delete/{id}", delete(delete_event))
}

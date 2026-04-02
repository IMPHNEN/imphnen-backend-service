use super::handlers::{
	delete_event, get_event_by_id, get_event_list, patch_update_event,
	post_create_event,
};
use crate::events::application::EventServiceImpl;
use crate::events::domain::EventService;
use crate::events::infrastructure::persistence::PostgresEventRepository;
use axum::{
	Extension, Router,
	routing::{delete, get, patch, post},
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

fn build_service(db: DatabaseConnection) -> Arc<dyn EventService> {
	let repo = Arc::new(PostgresEventRepository::new(db));
	Arc::new(EventServiceImpl::new(repo))
}

pub fn events_public_routes(db: DatabaseConnection) -> Router {
	let service = build_service(db);
	Router::new()
		.route("/events", get(get_event_list))
		.route("/events/detail/{id}", get(get_event_by_id))
		.layer(Extension(service))
}

pub fn events_protected_routes(db: DatabaseConnection) -> Router {
	let service = build_service(db);
	Router::new()
		.route("/events/create", post(post_create_event))
		.route("/events/update/{id}", patch(patch_update_event))
		.route("/events/delete/{id}", delete(delete_event))
		.layer(Extension(service))
}

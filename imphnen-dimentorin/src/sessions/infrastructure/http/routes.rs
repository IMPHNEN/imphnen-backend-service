use super::handlers::{
	get_mentor_availability, get_mentor_sessions, get_my_sessions, post_book_session,
	post_submit_feedback, put_update_session_status,
};
use crate::sessions::application::SessionServiceImpl;
use crate::sessions::domain::SessionService;
use crate::sessions::infrastructure::persistence::PostgresSessionRepository;
use axum::{
	Extension, Router,
	routing::{get, post, put},
};
use imphnen_libs::AppState;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

fn build_service(db: DatabaseConnection) -> Arc<dyn SessionService> {
	let repo = Arc::new(PostgresSessionRepository::new(db));
	Arc::new(SessionServiceImpl::new(repo))
}

pub fn sessions_public_routes(db: DatabaseConnection) -> Router {
	let service = build_service(db);
	Router::new()
		.route("/mentors/{id}/availability", get(get_mentor_availability))
		.layer(Extension(service))
}

pub fn sessions_protected_routes(
	db: DatabaseConnection,
	state: Arc<AppState>,
) -> Router {
	let service = build_service(db);
	Router::new()
		.route("/mentors/{id}/sessions/create", post(post_book_session))
		.route("/mentors/{id}/sessions", get(get_mentor_sessions))
		.route(
			"/sessions/update/{id}/status",
			put(put_update_session_status),
		)
		.route("/sessions/{id}/feedback/create", post(post_submit_feedback))
		.route("/sessions/me", get(get_my_sessions))
		.layer(Extension(service))
		.layer(Extension((*state).clone()))
}

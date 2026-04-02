use super::handlers::{
	delete_mentor, get_mentor_by_id, get_mentor_list, get_mentor_me,
	get_mentor_status, post_register_mentor, put_update_mentor, put_update_mentor_me,
	put_update_mentor_no_id, put_verify_mentor,
};
use crate::mentors::application::MentorServiceImpl;
use crate::mentors::domain::MentorService;
use crate::mentors::infrastructure::persistence::PostgresMentorRepository;
use axum::{
	Extension, Router,
	routing::{delete, get, post, put},
};
use imphnen_iam::roles::infrastructure::persistence::PostgresRoleRepository;
use imphnen_iam::users::infrastructure::persistence::PostgresUserRepository;
use imphnen_libs::AppState;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

fn build_service(
	db: DatabaseConnection,
	state: Arc<AppState>,
) -> Arc<dyn MentorService> {
	let user_repo = Arc::new(PostgresUserRepository::new(db.clone()));
	let role_repo = Arc::new(PostgresRoleRepository::new(db.clone()));
	let repo = Arc::new(PostgresMentorRepository::new(db));
	Arc::new(MentorServiceImpl::new(repo, state, user_repo, role_repo))
}

pub fn mentors_public_routes(
	db: DatabaseConnection,
	state: Arc<AppState>,
) -> Router {
	let service = build_service(db, state);
	Router::new()
		.route("/mentors/create", post(post_register_mentor))
		.layer(Extension(service))
}

pub fn mentors_protected_routes(
	db: DatabaseConnection,
	state: Arc<AppState>,
) -> Router {
	let svc = build_service(db, Arc::clone(&state));
	Router::new()
		.route("/mentors", get(get_mentor_list))
		.route("/mentors/me", get(get_mentor_me))
		.route("/mentors/me/update", put(put_update_mentor_me))
		.route("/mentors/me/status", get(get_mentor_status))
		.route("/mentors/detail/{id}", get(get_mentor_by_id))
		.route("/mentors/update/{id}", put(put_update_mentor))
		.route("/mentors/update", put(put_update_mentor_no_id))
		.route("/mentors/delete/{id}", delete(delete_mentor))
		.route("/mentors/verify/{id}", put(put_verify_mentor))
		.layer(Extension(svc))
		.layer(Extension((*state).clone()))
}

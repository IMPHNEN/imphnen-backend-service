use super::handlers::{
	delete_user, get_user_by_id, get_user_list, get_user_me, patch_user_active_status,
	post_create_user, put_update_user, put_update_user_me, upload_file,
};
use crate::users::application::UserServiceImpl;
use crate::users::domain::UserService;
use crate::users::infrastructure::persistence::PostgresUserRepository;
use axum::{
	Extension, Router,
	routing::{delete, get, post, put},
};
use imphnen_libs::AppState;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

fn build_service(db: DatabaseConnection) -> Arc<dyn UserService> {
	let repo = Arc::new(PostgresUserRepository::new(db));
	Arc::new(UserServiceImpl::new(repo))
}

pub fn users_public_routes(_db: DatabaseConnection) -> Router {
	Router::new()
}

pub fn users_protected_routes(
	db: DatabaseConnection,
	state: Arc<AppState>,
) -> Router {
	let service = build_service(db);
	Router::new()
		.route("/users", get(get_user_list))
		.route("/users/detail/{id}", get(get_user_by_id))
		.route("/users/me", get(get_user_me))
		.route("/users/create", post(post_create_user))
		.route("/users/update/{id}", put(put_update_user))
		.route("/users/update/me", put(put_update_user_me))
		.route("/users/activate/{id}", put(patch_user_active_status))
		.route("/users/delete/{id}", delete(delete_user))
		.route("/users/upload", post(upload_file))
		.layer(Extension(service))
		.layer(Extension((*state).clone()))
}

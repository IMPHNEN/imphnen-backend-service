use super::handlers::{
	delete_permission, get_permission_by_id, get_permission_list,
	post_create_permission, put_update_permission,
};
use crate::permissions::application::PermissionServiceImpl;
use crate::permissions::domain::PermissionService;
use crate::permissions::infrastructure::persistence::PostgresPermissionRepository;
use axum::{
	Extension, Router,
	routing::{delete, get, post, put},
};
use imphnen_libs::AppState;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

fn build_service(db: DatabaseConnection) -> Arc<dyn PermissionService> {
	let repo = Arc::new(PostgresPermissionRepository::new(db));
	Arc::new(PermissionServiceImpl::new(repo))
}

pub fn permissions_public_routes(_db: DatabaseConnection) -> Router {
	Router::new()
}

pub fn permissions_protected_routes(
	db: DatabaseConnection,
	state: Arc<AppState>,
) -> Router {
	let service = build_service(db);
	Router::new()
		.route("/permissions", get(get_permission_list))
		.route("/permissions/detail/{id}", get(get_permission_by_id))
		.route("/permissions/create", post(post_create_permission))
		.route("/permissions/update/{id}", put(put_update_permission))
		.route("/permissions/delete/{id}", delete(delete_permission))
		.layer(Extension(service))
		.layer(Extension((*state).clone()))
}

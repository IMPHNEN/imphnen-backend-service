use std::sync::Arc;
use axum::{Router, routing::{delete, get, post, put}, Extension};
use sea_orm::DatabaseConnection;
use imphnen_libs::AppState;
use crate::roles::application::RoleServiceImpl;
use crate::roles::domain::RoleService;
use crate::roles::infrastructure::persistence::PostgresRoleRepository;
use super::handlers::{
    get_role_list, get_role_by_id, post_create_role, put_update_role, delete_role,
};

fn build_service(db: DatabaseConnection) -> Arc<dyn RoleService> {
    let repo = Arc::new(PostgresRoleRepository::new(db));
    Arc::new(RoleServiceImpl::new(repo))
}

pub fn roles_public_routes(_db: DatabaseConnection) -> Router {
    Router::new()
}

pub fn roles_protected_routes(db: DatabaseConnection, state: Arc<AppState>) -> Router {
    let service = build_service(db);
    Router::new()
        .route("/roles", get(get_role_list))
        .route("/roles/detail/{id}", get(get_role_by_id))
        .route("/roles/create", post(post_create_role))
        .route("/roles/update/{id}", put(put_update_role))
        .route("/roles/delete/{id}", delete(delete_role))
        .layer(Extension(service))
        .layer(Extension((*state).clone()))
}

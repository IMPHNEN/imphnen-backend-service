use super::handlers::*;
use crate::admin::application::admin_service::AdminServiceImpl;
use crate::admin::domain::service::AdminService;
use crate::admin::infrastructure::persistence::PostgresAdminRepository;
use crate::middleware::{
	admin_only::admin_only, hackathon_auth::hackathon_auth_middleware,
};
use axum::{
	Extension, Router,
	middleware::from_fn,
	routing::{delete, get, post},
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn hackathon_admin_routes(pool: Arc<PgPool>) -> Router {
	let service: Arc<dyn AdminService> = Arc::new(AdminServiceImpl::new(Arc::new(
		PostgresAdminRepository::new(pool.clone()),
	)));
	Router::new()
		.route("/admin/users", get(admin_list_users))
		.route(
			"/admin/users/:user_id",
			get(admin_get_user).delete(admin_delete_user),
		)
		.route("/admin/users/:user_id/set-admin", post(admin_set_admin))
		.route("/admin/teams", get(admin_list_teams))
		.route("/admin/teams/:team_id", delete(admin_delete_team))
		.route("/admin/submissions", get(admin_list_submissions))
		.route(
			"/admin/winners",
			get(admin_list_winners).post(admin_set_winner),
		)
		.route("/admin/winners/:team_id", delete(admin_remove_winner))
		.layer(Extension(service))
		.layer(Extension(pool.clone()))
		.layer(from_fn(admin_only))
		.layer(Extension(pool))
		.layer(from_fn(hackathon_auth_middleware))
}

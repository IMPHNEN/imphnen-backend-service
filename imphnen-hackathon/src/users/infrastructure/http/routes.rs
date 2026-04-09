use super::handlers::*;
use crate::middleware::hackathon_auth::hackathon_auth_middleware;
use crate::users::application::user_service::HackathonUserServiceImpl;
use crate::users::domain::service::HackathonUserService;
use crate::users::infrastructure::persistence::PostgresHackathonUserRepository;
use axum::{Extension, Router, middleware::from_fn, routing::get};
use sqlx::PgPool;
use std::sync::Arc;

fn build_service(pool: Arc<PgPool>) -> Arc<dyn HackathonUserService> {
	let repo = Arc::new(PostgresHackathonUserRepository::new(pool));
	Arc::new(HackathonUserServiceImpl::new(repo))
}

pub fn hackathon_users_routes(pool: Arc<PgPool>) -> Router {
	let service = build_service(pool.clone());
	Router::new()
		.route("/users/me", get(get_me_handler).put(update_me_handler))
		.route("/users/{user_id}", get(get_user_handler))
		.route("/users/{user_id}/teams", get(get_user_teams_handler))
		.layer(Extension(service))
		.layer(from_fn(hackathon_auth_middleware))
		.layer(Extension(pool))
}

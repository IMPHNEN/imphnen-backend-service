use axum::{middleware::from_fn, routing::get, Extension, Router};
use sqlx::PgPool;
use std::sync::Arc;
use crate::users::application::user_service::HackathonUserServiceImpl;
use crate::users::domain::service::HackathonUserService;
use crate::users::infrastructure::persistence::PostgresHackathonUserRepository;
use crate::common::hackathon_jwt::HackathonJwtService;
use crate::middleware::hackathon_auth::hackathon_auth_middleware;
use super::handlers::*;

fn build_service(pool: Arc<PgPool>) -> Arc<dyn HackathonUserService> {
    let repo = Arc::new(PostgresHackathonUserRepository::new(pool));
    Arc::new(HackathonUserServiceImpl::new(repo))
}

pub fn hackathon_users_routes(pool: Arc<PgPool>, jwt: Arc<HackathonJwtService>) -> Router {
    let service = build_service(pool.clone());
    Router::new()
        .route("/users/me", get(get_me_handler).put(update_me_handler))
        .route("/users/:user_id", get(get_user_handler))
        .route("/users/:user_id/teams", get(get_user_teams_handler))
        .layer(Extension(service))
        .layer(Extension(jwt))
        .layer(Extension(pool))
        .layer(from_fn(hackathon_auth_middleware))
}

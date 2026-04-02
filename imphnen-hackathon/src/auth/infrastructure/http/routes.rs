use axum::{middleware::from_fn, routing::{get, post}, Extension, Router};
use sqlx::PgPool;
use std::sync::Arc;
use crate::auth::application::auth_service::HackathonAuthServiceImpl;
use crate::auth::domain::service::HackathonAuthService;
use crate::common::hackathon_jwt::HackathonJwtService;
use crate::common::supabase_client::SupabaseClient;
use crate::config::HackathonConfig;
use crate::middleware::hackathon_auth::hackathon_auth_middleware;
use super::handlers::*;

pub fn hackathon_auth_routes(pool: Arc<PgPool>, jwt: Arc<HackathonJwtService>, supabase: Arc<SupabaseClient>, config: Arc<HackathonConfig>) -> Router {
    let service: Arc<dyn HackathonAuthService> = Arc::new(
        HackathonAuthServiceImpl::new(pool.clone(), jwt.clone(), supabase, config)
    );

    let public = Router::new()
        .route("/auth/signup", post(signup_handler))
        .route("/auth/login", post(login_handler))
        .route("/auth/github", post(github_auth_handler))
        .route("/auth/forgot-password", post(forgot_password_handler))
        .route("/auth/reset-password", post(reset_password_handler))
        .layer(Extension(service.clone()));

    let protected = Router::new()
        .route("/auth/session", get(get_session_handler))
        .layer(Extension(service))
        .layer(Extension(jwt.clone()))
        .layer(Extension(pool))
        .layer(from_fn(hackathon_auth_middleware));

    public.merge(protected)
}

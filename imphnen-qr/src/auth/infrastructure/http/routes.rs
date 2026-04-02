use axum::{routing::{get, post}, Extension, Router};
use sqlx::PgPool;
use std::sync::Arc;
use crate::auth::application::auth_service::QrAuthServiceImpl;
use crate::auth::domain::service::QrAuthService;
use crate::common::qr_jwt::QrJwtService;
use crate::config::QrConfig;
use super::handlers::{
    register_handler,
    login_handler,
    google_redirect_handler,
    google_callback_handler,
    refresh_handler,
};

pub fn qr_auth_routes(pool: Arc<PgPool>, jwt: Arc<QrJwtService>, config: Arc<QrConfig>) -> Router {
    let service: Arc<dyn QrAuthService> = Arc::new(
        QrAuthServiceImpl::new(pool, jwt, config.clone())
    );

    Router::new()
        .route("/auth/register", post(register_handler))
        .route("/auth/login", post(login_handler))
        .route("/auth/google", get(google_redirect_handler))
        .route("/auth/google/callback", get(google_callback_handler))
        .route("/auth/refresh", post(refresh_handler))
        .layer(Extension(service))
        .layer(Extension(config))
}

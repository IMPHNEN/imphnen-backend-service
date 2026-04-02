pub mod config;
pub mod common;
pub mod middleware;
pub mod auth;
pub mod users;
pub mod campaigns;

pub use config::QrConfig;

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;
use common::qr_jwt::QrJwtService;

pub fn qr_router(pool: Arc<PgPool>, config: Arc<QrConfig>) -> Router {
    let jwt = Arc::new(QrJwtService::new(
        &config.jwt_secret,
        config.jwt_expiry_minutes,
        config.refresh_expiry_days,
    ));

    Router::new()
        .merge(auth::infrastructure::http::routes::qr_auth_routes(pool.clone(), jwt.clone(), config.clone()))
        .merge(users::infrastructure::http::routes::qr_users_routes(pool.clone(), jwt.clone()))
        .merge(campaigns::infrastructure::http::routes::qr_campaigns_routes(pool.clone(), jwt.clone()))
}

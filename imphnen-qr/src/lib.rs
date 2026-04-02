pub mod common;
pub mod middleware;
pub mod users;
pub mod campaigns;

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

pub fn qr_router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .merge(users::infrastructure::http::routes::qr_users_routes(pool.clone()))
        .merge(campaigns::infrastructure::http::routes::qr_campaigns_routes(pool))
}

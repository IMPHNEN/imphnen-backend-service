pub mod campaigns;
pub mod middleware;
pub mod users;

use axum::Router;
use sea_orm::DatabaseConnection;
use sqlx::PgPool;
use std::sync::Arc;

pub fn qr_router(db: DatabaseConnection) -> Router {
    let pool: Arc<PgPool> = Arc::new(db.get_postgres_connection_pool().clone());
    Router::new()
        .merge(users::infrastructure::http::routes::qr_users_routes(pool.clone()))
        .merge(campaigns::infrastructure::http::routes::qr_campaigns_routes(pool))
}

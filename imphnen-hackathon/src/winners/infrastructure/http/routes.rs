use super::handlers::list_winners_handler;
use crate::winners::application::winner_service::WinnerServiceImpl;
use crate::winners::domain::service::WinnerService;
use crate::winners::infrastructure::persistence::PostgresWinnerRepository;
use axum::{Extension, Router, routing::get};
use sqlx::PgPool;
use std::sync::Arc;

pub fn hackathon_winners_routes(pool: Arc<PgPool>) -> Router {
	let service: Arc<dyn WinnerService> = Arc::new(WinnerServiceImpl::new(Arc::new(
		PostgresWinnerRepository::new(pool.clone()),
	)));
	Router::new()
		.route("/winners", get(list_winners_handler))
		.layer(Extension(service))
		.layer(Extension(pool))
}

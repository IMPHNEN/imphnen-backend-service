use super::handlers::{get_user_credits, post_add_credits, post_consume_credit};
use crate::gacha_credits::application::GachaCreditServiceImpl;
use crate::gacha_credits::domain::GachaCreditService;
use crate::gacha_credits::infrastructure::persistence::PostgresGachaCreditRepository;
use axum::{
	Extension, Router,
	routing::{get, post},
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

fn build_service(db: DatabaseConnection) -> Arc<dyn GachaCreditService> {
	let repo = Arc::new(PostgresGachaCreditRepository::new(db));
	Arc::new(GachaCreditServiceImpl::new(repo))
}

pub fn gacha_credit_router(db: DatabaseConnection) -> Router {
	let service = build_service(db);
	Router::new()
		.route("/", get(get_user_credits))
		.route("/add", post(post_add_credits))
		.route("/consume", post(post_consume_credit))
		.layer(Extension(service))
}

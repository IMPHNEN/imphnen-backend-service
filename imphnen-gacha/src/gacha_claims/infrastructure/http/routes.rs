use std::sync::Arc;
use axum::{Router, routing::{get, post}, Extension};
use sea_orm::DatabaseConnection;
use crate::gacha_claims::application::GachaClaimServiceImpl;
use crate::gacha_claims::domain::GachaClaimService;
use crate::gacha_claims::infrastructure::persistence::PostgresGachaClaimRepository;
use super::handlers::{get_gacha_claim_by_id, post_create_gacha_claim};

fn build_service(db: DatabaseConnection, state: std::sync::Arc<imphnen_libs::AppState>) -> Arc<dyn GachaClaimService> {
    let repo = Arc::new(PostgresGachaClaimRepository::new(db, state));
    Arc::new(GachaClaimServiceImpl::new(repo))
}

pub fn gacha_claim_router(db: DatabaseConnection, state: std::sync::Arc<imphnen_libs::AppState>) -> Router {
    let service = build_service(db, state);
    Router::new()
        .route("/detail/{id}", get(get_gacha_claim_by_id))
        .route("/create", post(post_create_gacha_claim))
        .layer(Extension(service))
}

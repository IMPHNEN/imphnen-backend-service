use std::sync::Arc;
use axum::{Router, routing::{delete, get, post}, Extension};
use sea_orm::DatabaseConnection;
use crate::gacha_claims::infrastructure::persistence::PostgresGachaClaimRepository;
use crate::gacha_credits::infrastructure::persistence::PostgresGachaCreditRepository;
use crate::gacha_rolls::application::GachaRollServiceImpl;
use crate::gacha_rolls::domain::GachaRollService;
use crate::gacha_rolls::infrastructure::persistence::PostgresGachaRollRepository;
use super::handlers::{
    delete_gacha_roll, get_gacha_roll_by_id, post_create_gacha_roll, post_execute_gacha_roll,
};

fn build_service(
    db: DatabaseConnection,
    state: Arc<imphnen_libs::AppState>,
) -> Arc<dyn GachaRollService> {
    let roll_repo = Arc::new(PostgresGachaRollRepository::new(db.clone()));
    let credit_repo = Arc::new(PostgresGachaCreditRepository::new(db.clone()));
    let claim_repo = Arc::new(PostgresGachaClaimRepository::new(db, state));
    Arc::new(GachaRollServiceImpl::new(roll_repo, credit_repo, claim_repo))
}

pub fn gacha_roll_router(db: DatabaseConnection, state: Arc<imphnen_libs::AppState>) -> Router {
    let service = build_service(db, state);
    Router::new()
        .route("/detail/{id}", get(get_gacha_roll_by_id))
        .route("/create", post(post_create_gacha_roll))
        .route("/execute", post(post_execute_gacha_roll))
        .route("/delete/{id}", delete(delete_gacha_roll))
        .layer(Extension(service))
}

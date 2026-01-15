use axum::{Router, routing::get};
use axum::routing::post;

pub fn gacha_credit_router() -> Router {
    Router::new()
        .route("/", get(crate::v1::gacha_credits::GachaCreditController::get_user_credits))
        .route("/add", post(crate::v1::gacha_credits::GachaCreditController::add_user_credits))
        .route("/consume", post(crate::v1::gacha_credits::GachaCreditController::consume_user_credit))
}
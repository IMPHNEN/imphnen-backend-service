use axum::{
    extract::Json,
    http::HeaderMap,
    response::Response,
    Extension,
};
use crate::AppState;
use crate::v1::gacha_credits::gacha_credits_dto::GachaCreditRequestDto;
use crate::v1::gacha_credits::gacha_credits_service::GachaCreditService;

pub struct GachaCreditController;

impl GachaCreditController {
    pub async fn get_user_credits(
        headers: HeaderMap,
        Extension(state): Extension<AppState>,
    ) -> Response {
        GachaCreditService::get_user_credits(&headers, &state).await
    }

    pub async fn add_user_credits(
        headers: HeaderMap,
        Extension(state): Extension<AppState>,
        Json(payload): Json<GachaCreditRequestDto>,
    ) -> Response {
        GachaCreditService::add_user_credits(&headers, &state, payload).await
    }

    pub async fn consume_user_credit(
        headers: HeaderMap,
        Extension(state): Extension<AppState>,
    ) -> Response {
        GachaCreditService::consume_user_credit(&headers, &state).await
    }
}
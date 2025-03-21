use super::{GachaCreateClaimRequestDto, GachaCreateRollRequestDto, GachaService};
use crate::{v1::GachaCreateItemRequestDto, AppState, MessageResponseDto};
use axum::{http::HeaderMap, response::IntoResponse, Extension, Json};

#[utoipa::path(
    post,
    path = "/v1/gacha/claims/create",
    request_body = GachaCreateClaimRequestDto,
    responses(
        (status = 200, description = "Create gacha claim successful", body = MessageResponseDto),
        (status = 401, description = "Create gacha claim failed", body = MessageResponseDto)
    ),
    tag = "Gacha"
)]
pub async fn post_create_gacha_claim(
	header: HeaderMap,
	Extension(state): Extension<AppState>,
	Json(payload): Json<GachaCreateClaimRequestDto>,
) -> impl IntoResponse {
	GachaService::mutation_create_gacha_claim(payload, &state, header).await
}

#[utoipa::path(
    post,
    path = "/v1/gacha/items/create",
    request_body = GachaCreateItemRequestDto,
    responses(
        (status = 200, description = "Create gacha item successful", body = MessageResponseDto),
        (status = 401, description = "Create gacha item failed", body = MessageResponseDto)
    ),
    tag = "Gacha"
)]
pub async fn post_create_gacha_item(
	Extension(state): Extension<AppState>,
	Json(payload): Json<GachaCreateItemRequestDto>,
) -> impl IntoResponse {
	GachaService::mutation_create_gacha_item(payload, &state).await
}

#[utoipa::path(
    post,
    path = "/v1/gacha/rolls/create",
    request_body = GachaCreateRollRequestDto,
    responses(
        (status = 200, description = "Create gacha roll successful", body = MessageResponseDto),
        (status = 401, description = "Create gacha roll failed", body = MessageResponseDto)
    ),
    tag = "Gacha"
)]
pub async fn post_create_gacha_roll(
	Extension(state): Extension<AppState>,
	Json(payload): Json<GachaCreateRollRequestDto>,
) -> impl IntoResponse {
	GachaService::mutation_create_gacha_roll(payload, &state).await
}

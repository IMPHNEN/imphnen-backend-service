use crate::{
	AppState, GachaRollItemDto, GachaRollRequestDto, GachaRollService,
	MessageResponseDto, ResponseSuccessDto,
};
use axum::{
	Extension, Json, extract::Path, http::HeaderMap, response::IntoResponse,
};
use imphnen_iam::{PermissionsEnum, permissions_guard};

#[utoipa::path(
    get,
    path = "/v1/gacha/rolls/detail/{id}",
    security(
        ("Bearer" = [])
    ),
    params(("id" = String, Path, description = "Gacha Roll ID")),
    responses(
        (status = 200, description = "Get Gacha Roll by ID", body = ResponseSuccessDto<GachaRollItemDto>)
    ),
    tag = "Gacha"
)]
pub async fn get_detail_gacha_roll(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadDetailGachaRolls],
	)
	.await
	{
		Ok(_) => GachaRollService::get_gacha_roll_by_id(&state, id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    post,
    path = "/v1/gacha/rolls/create",
    security(
        ("Bearer" = [])
    ),
    request_body = GachaRollRequestDto,
    responses(
        (status = 201, description = "Create new gacha roll", body = MessageResponseDto)
    ),
    tag = "Gacha"
)]
pub async fn post_create_gacha_roll(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Json(payload): Json<GachaRollRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::CreateGachaRolls],
	)
	.await
	{
		Ok(_) => GachaRollService::create_gacha_roll(&state, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    post,
    path = "/v1/gacha/rolls/execute",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 200, description = "Execute and get 1 gacha result", body = ResponseSuccessDto<GachaRollItemDto>)
    ),
    tag = "Gacha"
)]
pub async fn post_execute_gacha_roll(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ExecuteGachaRolls],
	)
	.await
	{
		Ok(_) => GachaRollService::execute_roll_once(headers, &state).await,
		Err(response) => response,
	}
}

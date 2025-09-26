use axum::Extension;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{Json, extract::Path};
use imphnen_iam::{PermissionsEnum, permissions_guard};
use crate::AppState;
use imphnen_entities::{MessageResponseDto, ResponseSuccessDto};
use crate::v1::gacha_claims::{GachaClaimItemDto, GachaClaimRequestDto, GachaClaimService};

#[utoipa::path(
	get,
	path = "/v1/gacha/claims/detail/{id}",
	security(
        ("Bearer" = [])
    ),
	params(("id" = String, Path, description = "Gacha Claim ID")),
	responses(
		(status = 200, description = "Get Gacha Claim by ID", body = ResponseSuccessDto<GachaClaimItemDto>)
	),
	tag = "Gacha"
)]
pub async fn get_detail_gacha_claim(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		headers,
		Extension(state),
		vec![PermissionsEnum::ReadDetailGachaClaims],
	)
	.await
	{
		Ok((_user, state)) => GachaClaimService::get_gacha_claim_by_id(&state, id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	post,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/gacha/claims/create",
	request_body = GachaClaimRequestDto,
	responses(
		(status = 201, description = "Create new gacha claim", body = MessageResponseDto)
	),
	tag = "Gacha"
)]
pub async fn post_create_gacha_claim(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Json(payload): Json<GachaClaimRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		headers,
		Extension(state),
		vec![PermissionsEnum::CreateGachaClaims],
	)
	.await
	{
		Ok((_user, state)) => GachaClaimService::create_gacha_claim(&state, payload).await,
		Err(response) => response,
	}
}

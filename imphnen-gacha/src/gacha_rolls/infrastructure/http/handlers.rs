use super::dto::{GachaRollCreateRequestDto, GachaRollItemDto};
use crate::gacha_rolls::domain::{GachaRollEntity, GachaRollService};
use axum::{Extension, extract::Path, http::HeaderMap, response::IntoResponse};
use imphnen_entities::ResponseSuccessDto;
use imphnen_iam::{PermissionsEnum, require_permissions};
use imphnen_libs::{AppState, ValidatedJson};
use imphnen_utils::AppError;
use imphnen_utils::{ApiMessage, ApiSuccess, extract_email};
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    get,
    security(("Bearer" = [])),
    path = "/v1/gacha/rolls/detail/{id}",
    params(
        ("id" = String, Path, description = "Gacha Roll ID")
    ),
    responses(
        (status = 200, description = "[ADMIN] Get gacha roll by ID", body = ResponseSuccessDto<GachaRollItemDto>)
    ),
    tag = "Gacha"
)]
pub async fn get_gacha_roll_by_id(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn GachaRollService>>,
	Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(headers, state, [PermissionsEnum::ReadDetailGachaRolls], {
		let uuid = Uuid::parse_str(&id)
			.map_err(|e| AppError::BadRequestError(format!("Invalid UUID: {e}")))?;
		let roll = service.get_roll(uuid).await?;
		Ok(ApiSuccess(GachaRollItemDto::from(&roll)))
	})
}

#[utoipa::path(
    post,
    security(("Bearer" = [])),
    path = "/v1/gacha/rolls/create",
    request_body = GachaRollCreateRequestDto,
    responses(
        (status = 201, description = "[ADMIN] Create new gacha roll")
    ),
    tag = "Gacha"
)]
pub async fn post_create_gacha_roll(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn GachaRollService>>,
	ValidatedJson(payload): ValidatedJson<GachaRollCreateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(
		headers.clone(),
		state,
		[PermissionsEnum::CreateGachaRolls],
		{
			let email = extract_email(&headers)
				.ok_or_else(|| AppError::AuthenticationError("Unauthorized".to_string()))?;
			let user_info = state
				.user_lookup_service
				.get_user_by_email(&email, &state)
				.await
				.map_err(|_| AppError::NotFoundError("User not found".to_string()))?;
			let user_id = Uuid::parse_str(&user_info.basic_info.id)
				.map_err(|e| AppError::BadRequestError(e.to_string()))?;
			let item_id = Uuid::parse_str(&payload.item_id).map_err(|e| {
				AppError::BadRequestError(format!("Invalid item_id UUID: {e}"))
			})?;
			let entity = GachaRollEntity {
				id: Uuid::new_v4(),
				user_id,
				gacha_id: "default".to_string(),
				item_id,
				weight: payload.weight,
				quantity: payload.quantity,
				is_deleted: false,
				created_at: Some(chrono::Utc::now().naive_utc()),
				updated_at: Some(chrono::Utc::now().naive_utc()),
			};
			service.create_roll(entity).await?;
			Ok(ApiMessage::created("Gacha roll created"))
		}
	)
}

#[utoipa::path(
    post,
    security(("Bearer" = [])),
    path = "/v1/gacha/rolls/execute",
    responses(
        (status = 200, description = "[USER] Execute a gacha roll and receive a result", body = ResponseSuccessDto<GachaRollItemDto>)
    ),
    tag = "Gacha"
)]
pub async fn post_execute_gacha_roll(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn GachaRollService>>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(
		headers.clone(),
		state,
		[PermissionsEnum::ExecuteGachaRolls],
		{
			let email = extract_email(&headers)
				.ok_or_else(|| AppError::AuthenticationError("Unauthorized".to_string()))?;
			let user_info = state
				.user_lookup_service
				.get_user_by_email(&email, &state)
				.await
				.map_err(|_| AppError::NotFoundError("User not found".to_string()))?;
			let user_id = Uuid::parse_str(&user_info.basic_info.id)
				.map_err(|e| AppError::BadRequestError(e.to_string()))?;
			let roll = service.execute_roll(user_id).await?;
			Ok(ApiSuccess(GachaRollItemDto::from(&roll)))
		}
	)
}

#[utoipa::path(
    delete,
    security(("Bearer" = [])),
    path = "/v1/gacha/rolls/delete/{id}",
    params(
        ("id" = String, Path, description = "Gacha Roll ID")
    ),
    responses(
        (status = 200, description = "[ADMIN] Delete gacha roll (soft delete)")
    ),
    tag = "Gacha"
)]
pub async fn delete_gacha_roll(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn GachaRollService>>,
	Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(headers, state, [PermissionsEnum::DeleteGachaRolls], {
		let uuid = Uuid::parse_str(&id)
			.map_err(|e| AppError::BadRequestError(format!("Invalid UUID: {e}")))?;
		service.delete_roll(uuid).await?;
		Ok(ApiMessage::ok("Gacha roll deleted"))
	})
}

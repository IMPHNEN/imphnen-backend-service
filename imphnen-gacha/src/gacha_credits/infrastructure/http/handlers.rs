use std::sync::Arc;
use axum::{Extension, http::HeaderMap, response::IntoResponse};
use imphnen_libs::{AppState, ValidatedJson};
use imphnen_utils::{ApiSuccess, ApiMessage, extract_email};
use imphnen_entities::ResponseSuccessDto;
use imphnen_iam::{PermissionsEnum, require_permissions};
use imphnen_utils::AppError;
use uuid::Uuid;
use super::dto::{GachaCreditAddRequestDto, GachaCreditDto};
use crate::gacha_credits::domain::GachaCreditService;

#[utoipa::path(
    get,
    security(("Bearer" = [])),
    path = "/v1/gacha/credits",
    responses(
        (status = 200, description = "[ADMIN] Get current user credits", body = ResponseSuccessDto<GachaCreditDto>)
    ),
    tag = "Gacha"
)]
pub async fn get_user_credits(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn GachaCreditService>>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers.clone(), state, [PermissionsEnum::ReadDetailGachaItems], {
        let email = extract_email(&headers)
            .ok_or_else(|| AppError::AuthenticationError("Unauthorized".to_string()))?;
        let user_info = state.user_lookup_service.get_user_by_email(&email, &state).await
            .map_err(|_| AppError::NotFoundError("User not found".to_string()))?;
        let user = user_info.basic_info;
        let user_id = Uuid::parse_str(&user.id)
            .map_err(|e| AppError::BadRequestError(e.to_string()))?;
        match service.get_credits(user_id).await? {
            Some(credit) => Ok(ApiSuccess(GachaCreditDto::from(credit))),
            None => Ok(ApiSuccess(GachaCreditDto {
                id: "".to_string(),
                user_id: user.id,
                available_rolls: 0,
                is_deleted: false,
                created_at: None,
                updated_at: None,
            })),
        }
    })
}

#[utoipa::path(
    post,
    security(("Bearer" = [])),
    path = "/v1/gacha/credits/add",
    request_body = GachaCreditAddRequestDto,
    responses(
        (status = 200, description = "[ADMIN] Add credits to current user")
    ),
    tag = "Gacha"
)]
pub async fn post_add_credits(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn GachaCreditService>>,
    ValidatedJson(payload): ValidatedJson<GachaCreditAddRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers.clone(), state, [PermissionsEnum::CreateGachaItems], {
        let email = extract_email(&headers)
            .ok_or_else(|| AppError::AuthenticationError("Unauthorized".to_string()))?;
        let user_info = state.user_lookup_service.get_user_by_email(&email, &state).await
            .map_err(|_| AppError::NotFoundError("User not found".to_string()))?;
        let user_id = Uuid::parse_str(&user_info.basic_info.id)
            .map_err(|e| AppError::BadRequestError(e.to_string()))?;
        service.add_credits(user_id, payload.amount).await?;
        Ok(ApiMessage::ok(format!("Added {} credits successfully", payload.amount)))
    })
}

#[utoipa::path(
    post,
    security(("Bearer" = [])),
    path = "/v1/gacha/credits/consume",
    responses(
        (status = 200, description = "[USER] Consume 1 credit for the current user")
    ),
    tag = "Gacha"
)]
pub async fn post_consume_credit(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn GachaCreditService>>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers.clone(), state, [PermissionsEnum::UpdateGachaItems], {
        let email = extract_email(&headers)
            .ok_or_else(|| AppError::AuthenticationError("Unauthorized".to_string()))?;
        let user_info = state.user_lookup_service.get_user_by_email(&email, &state).await
            .map_err(|_| AppError::NotFoundError("User not found".to_string()))?;
        let user_id = Uuid::parse_str(&user_info.basic_info.id)
            .map_err(|e| AppError::BadRequestError(e.to_string()))?;
        service.consume_credit(user_id).await?;
        Ok(ApiMessage::ok("Consumed 1 credit successfully"))
    })
}

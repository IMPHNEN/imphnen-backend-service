use std::sync::Arc;
use axum::{Extension, extract::Path, http::HeaderMap, response::IntoResponse};
use imphnen_libs::{AppState, ValidatedJson};
use imphnen_utils::{ApiSuccess, ApiMessage};
use imphnen_entities::ResponseSuccessDto;
use imphnen_iam::{PermissionsEnum, require_permissions};
use imphnen_utils::AppError;
use uuid::Uuid;
use super::dto::{GachaClaimCreateRequestDto, GachaClaimDetailDto};
use crate::gacha_claims::domain::{GachaClaimEntity, GachaClaimService};

#[utoipa::path(
    get,
    security(("Bearer" = [])),
    path = "/v1/gacha/claims/detail/{id}",
    params(
        ("id" = String, Path, description = "Gacha Claim ID")
    ),
    responses(
        (status = 200, description = "[ADMIN] Get gacha claim by ID", body = ResponseSuccessDto<GachaClaimDetailDto>)
    ),
    tag = "Gacha"
)]
pub async fn get_gacha_claim_by_id(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn GachaClaimService>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::ReadDetailGachaClaims], {
        let uuid = Uuid::parse_str(&id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid UUID: {e}")))?;
        let detail = service.get_claim(uuid).await?;
        Ok(ApiSuccess(GachaClaimDetailDto::from(detail)))
    })
}

#[utoipa::path(
    post,
    security(("Bearer" = [])),
    path = "/v1/gacha/claims/create",
    request_body = GachaClaimCreateRequestDto,
    responses(
        (status = 201, description = "[ADMIN] Create new gacha claim")
    ),
    tag = "Gacha"
)]
pub async fn post_create_gacha_claim(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn GachaClaimService>>,
    ValidatedJson(payload): ValidatedJson<GachaClaimCreateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::CreateGachaClaims], {
        let user_id = Uuid::parse_str(&payload.user_id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid user_id UUID: {e}")))?;
        let item_id = Uuid::parse_str(&payload.item_id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid item_id UUID: {e}")))?;
        let entity = GachaClaimEntity {
            id: Uuid::new_v4(),
            user_id,
            gacha_item_id: item_id,
            claim_id: Uuid::new_v4(),
            claim_type: "standard".to_string(),
            status: "claimed".to_string(),
            quantity: 1,
            metadata: None,
            is_deleted: false,
            claimed_at: chrono::Utc::now(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        };
        service.create_claim(entity).await?;
        Ok(ApiMessage::created("Gacha claim created"))
    })
}

use std::sync::Arc;
use axum::{Extension, extract::Path, http::HeaderMap, response::IntoResponse};
use paginator_axum::PaginationQuery;
use paginator_utils::PaginatorResponse;
use uuid::Uuid;
use imphnen_libs::{AppState, ValidatedJson};
use imphnen_utils::{ApiSuccess, ApiPaginated, ApiMessage};
use imphnen_entities::ResponseSuccessDto;
use imphnen_iam::{PermissionsEnum, require_permissions};
use imphnen_utils::AppError;
use super::dto::{GachaItemCreateRequestDto, GachaItemDto, GachaItemUpdateRequestDto};
use crate::gacha_items::domain::{GachaItemEntity, GachaItemService};

#[utoipa::path(
    get,
    security(("Bearer" = [])),
    path = "/v1/gacha/items",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search keyword"),
        ("sort_by" = Option<String>, Query, description = "Sort by field"),
        ("order" = Option<String>, Query, description = "Order ASC or DESC"),
    ),
    responses(
        (status = 200, description = "[ADMIN] Get gacha item list")
    ),
    tag = "Gacha"
)]
pub async fn get_gacha_item_list(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn GachaItemService>>,
    PaginationQuery(params): PaginationQuery,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::ReadListGachaItems], {
        let result = service.list(params).await?;
        let mapped = PaginatorResponse {
            data: result.data.into_iter().map(GachaItemDto::from).collect::<Vec<_>>(),
            meta: result.meta,
        };
        Ok(ApiPaginated(mapped))
    })
}

#[utoipa::path(
    get,
    security(("Bearer" = [])),
    path = "/v1/gacha/items/detail/{id}",
    params(
        ("id" = String, Path, description = "Gacha Item ID")
    ),
    responses(
        (status = 200, description = "[ADMIN] Get gacha item by ID", body = ResponseSuccessDto<GachaItemDto>)
    ),
    tag = "Gacha"
)]
pub async fn get_gacha_item_by_id(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn GachaItemService>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::ReadDetailGachaItems], {
        let uuid = Uuid::parse_str(&id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid UUID: {e}")))?;
        let item = service.get(uuid).await?;
        Ok(ApiSuccess(GachaItemDto::from(item)))
    })
}

#[utoipa::path(
    post,
    security(("Bearer" = [])),
    path = "/v1/gacha/items/create",
    request_body = GachaItemCreateRequestDto,
    responses(
        (status = 201, description = "[ADMIN] Create gacha item")
    ),
    tag = "Gacha"
)]
pub async fn post_create_gacha_item(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn GachaItemService>>,
    ValidatedJson(payload): ValidatedJson<GachaItemCreateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::CreateGachaItems], {
        let entity: GachaItemEntity = payload.into();
        service.create(entity).await?;
        Ok(ApiMessage::created("Gacha item created"))
    })
}

#[utoipa::path(
    put,
    security(("Bearer" = [])),
    path = "/v1/gacha/items/update/{id}",
    params(
        ("id" = String, Path, description = "Gacha Item ID")
    ),
    request_body = GachaItemUpdateRequestDto,
    responses(
        (status = 200, description = "[ADMIN] Update gacha item")
    ),
    tag = "Gacha"
)]
pub async fn put_update_gacha_item(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn GachaItemService>>,
    Path(id): Path<String>,
    ValidatedJson(payload): ValidatedJson<GachaItemUpdateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::UpdateGachaItems], {
        let uuid = Uuid::parse_str(&id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid UUID: {e}")))?;
        let existing = service.get(uuid).await?;
        let entity = GachaItemEntity {
            id: existing.id,
            item_code: payload.item_code,
            name: payload.name,
            description: payload.description,
            rarity: payload.rarity,
            type_: payload.type_,
            category: payload.category,
            value: payload.value,
            weight: payload.weight,
            stock: payload.stock,
            is_limited: payload.is_limited,
            metadata: payload.metadata,
            is_deleted: existing.is_deleted,
            created_at: existing.created_at,
            updated_at: chrono::Utc::now(),
            deleted_at: existing.deleted_at,
        };
        service.update(entity).await?;
        Ok(ApiMessage::ok("Gacha item updated"))
    })
}

#[utoipa::path(
    delete,
    security(("Bearer" = [])),
    path = "/v1/gacha/items/delete/{id}",
    params(
        ("id" = String, Path, description = "Gacha Item ID")
    ),
    responses(
        (status = 200, description = "[ADMIN] Delete gacha item (soft delete)")
    ),
    tag = "Gacha"
)]
pub async fn delete_gacha_item(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn GachaItemService>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::DeleteGachaItems], {
        let uuid = Uuid::parse_str(&id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid UUID: {e}")))?;
        service.delete(uuid).await?;
        Ok(ApiMessage::ok("Gacha item deleted"))
    })
}

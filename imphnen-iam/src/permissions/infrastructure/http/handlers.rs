use crate::require_permissions;
use std::sync::Arc;
use axum::{
    Extension, Json,
    extract::Path,
    http::HeaderMap,
    response::IntoResponse,
};
use paginator_axum::PaginationQuery;
use paginator_utils::PaginatorResponse;
use imphnen_libs::AppState;
use imphnen_utils::{ApiSuccess, ApiPaginated, ApiMessage};
use imphnen_entities::{ResponseSuccessDto, ResponseListSuccessDto, PermissionsEnum};
use imphnen_utils::AppError;
use crate::permissions::domain::PermissionService;
use super::dto::{PermissionsCreateRequestDto, PermissionsItemDto, PermissionsUpdateRequestDto};

#[utoipa::path(
    get,
    path = "/v1/permissions",
    security(("Bearer" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search keyword"),
        ("sort_by" = Option<String>, Query, description = "Sort by field"),
        ("order" = Option<String>, Query, description = "Order ASC or DESC"),
        ("filter" = Option<String>, Query, description = "Filter value"),
        ("filter_by" = Option<String>, Query, description = "Field to filter by"),
    ),
    responses(
        (status = 200, description = "Get permission list", body = ResponseListSuccessDto<Vec<PermissionsItemDto>>)
    ),
    tag = "Permissions"
)]
pub async fn get_permission_list(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn PermissionService>>,
    PaginationQuery(params): PaginationQuery,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::ReadListPermissions], {
        let result = service.list(params).await?;
        let mapped = PaginatorResponse {
            data: result.data.into_iter().map(PermissionsItemDto::from).collect::<Vec<_>>(),
            meta: result.meta,
        };
        Ok(ApiPaginated(mapped))
    })
}

#[utoipa::path(
    get,
    path = "/v1/permissions/detail/{id}",
    security(("Bearer" = [])),
    params(("id" = String, Path, description = "Permission ID")),
    responses(
        (status = 200, description = "Get permission by ID", body = ResponseSuccessDto<PermissionsItemDto>)
    ),
    tag = "Permissions"
)]
pub async fn get_permission_by_id(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn PermissionService>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::ReadDetailPermissions], {
        let perm = service.get(id).await?;
        Ok(ApiSuccess(PermissionsItemDto::from(perm)))
    })
}

#[utoipa::path(
    post,
    path = "/v1/permissions/create",
    security(("Bearer" = [])),
    request_body = PermissionsCreateRequestDto,
    responses(
        (status = 201, description = "Create new permission")
    ),
    tag = "Permissions"
)]
pub async fn post_create_permission(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn PermissionService>>,
    Json(payload): Json<PermissionsCreateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::CreatePermissions], {
        let msg = service.create(payload.name).await?;
        Ok(ApiMessage::created(&msg))
    })
}

#[utoipa::path(
    put,
    path = "/v1/permissions/update/{id}",
    security(("Bearer" = [])),
    params(("id" = String, Path, description = "Permission ID")),
    request_body = PermissionsUpdateRequestDto,
    responses(
        (status = 200, description = "Update permission")
    ),
    tag = "Permissions"
)]
pub async fn put_update_permission(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn PermissionService>>,
    Path(id): Path<String>,
    Json(payload): Json<PermissionsUpdateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::UpdatePermissions], {
        let current = service.get(id.clone()).await
            .map_err(|_| AppError::NotFoundError("Permission not found".to_string()))?;
        let updated = payload.apply_to(current, id);
        let msg = service.update(updated).await?;
        Ok(ApiMessage::ok(&msg))
    })
}

#[utoipa::path(
    delete,
    path = "/v1/permissions/delete/{id}",
    security(("Bearer" = [])),
    params(("id" = String, Path, description = "Permission ID")),
    responses(
        (status = 200, description = "Delete permission")
    ),
    tag = "Permissions"
)]
pub async fn delete_permission(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn PermissionService>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::DeletePermissions], {
        let msg = service.delete(id).await?;
        Ok(ApiMessage::ok(&msg))
    })
}

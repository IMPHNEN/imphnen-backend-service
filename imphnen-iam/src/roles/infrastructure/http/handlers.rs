use super::dto::{
	RolesCreateRequestDto, RolesDetailItemDto, RolesListItemDto, RolesUpdateRequestDto,
};
use crate::require_permissions;
use crate::roles::domain::RoleService;
use axum::{
	Extension, Json, extract::Path, http::HeaderMap, response::IntoResponse,
};
use imphnen_entities::{
	PermissionsEnum, ResponseListSuccessDto, ResponseSuccessDto,
};
use imphnen_libs::AppState;
use imphnen_utils::AppError;
use imphnen_utils::{ApiCreated, ApiMessage, ApiPaginated, ApiSuccess};
use paginator_axum::PaginationQuery;
use paginator_utils::PaginatorResponse;
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/v1/iam/roles",
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
        (status = 200, description = "Get role list", body = ResponseListSuccessDto<Vec<RolesListItemDto>>)
    ),
    tag = "Roles"
)]
pub async fn get_role_list(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn RoleService>>,
	PaginationQuery(params): PaginationQuery,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(headers, state, [PermissionsEnum::ReadListRoles], {
		let result = service.list(params).await?;
		let mapped = PaginatorResponse {
			data: result
				.data
				.into_iter()
				.map(RolesListItemDto::from)
				.collect::<Vec<_>>(),
			meta: result.meta,
		};
		Ok(ApiPaginated(mapped))
	})
}

#[utoipa::path(
    get,
    path = "/v1/iam/roles/detail/{id}",
    security(("Bearer" = [])),
    params(("id" = String, Path, description = "Role ID")),
    responses(
        (status = 200, description = "Get role by ID", body = ResponseSuccessDto<RolesDetailItemDto>)
    ),
    tag = "Roles"
)]
pub async fn get_role_by_id(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn RoleService>>,
	Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(headers, state, [PermissionsEnum::ReadDetailRoles], {
		let role = service.get(id).await?;
		Ok(ApiSuccess(RolesDetailItemDto::from(role)))
	})
}

#[utoipa::path(
    post,
    path = "/v1/iam/roles/create",
    security(("Bearer" = [])),
    request_body = RolesCreateRequestDto,
    responses(
        (status = 201, description = "Create new role", body = ResponseSuccessDto<RolesDetailItemDto>)
    ),
    tag = "Roles"
)]
pub async fn post_create_role(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn RoleService>>,
	Json(payload): Json<RolesCreateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(headers, state, [PermissionsEnum::CreateRoles], {
		let role = service.create(payload.name, payload.permissions).await?;
		Ok(ApiCreated(RolesDetailItemDto::from(role)))
	})
}

#[utoipa::path(
    put,
    path = "/v1/iam/roles/update/{id}",
    security(("Bearer" = [])),
    params(("id" = String, Path, description = "Role ID")),
    request_body = RolesUpdateRequestDto,
    responses(
        (status = 200, description = "Update role")
    ),
    tag = "Roles"
)]
pub async fn put_update_role(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn RoleService>>,
	Path(id): Path<String>,
	Json(payload): Json<RolesUpdateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(headers, state, [PermissionsEnum::UpdateRoles], {
		let msg = service
			.update(id, payload.name, payload.permissions)
			.await?;
		Ok(ApiMessage::ok(&msg))
	})
}

#[utoipa::path(
    delete,
    path = "/v1/iam/roles/delete/{id}",
    security(("Bearer" = [])),
    params(("id" = String, Path, description = "Role ID")),
    responses(
        (status = 200, description = "Delete role")
    ),
    tag = "Roles"
)]
pub async fn delete_role(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn RoleService>>,
	Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(headers, state, [PermissionsEnum::DeleteRoles], {
		let msg = service.delete(id).await?;
		Ok(ApiMessage::ok(&msg))
	})
}

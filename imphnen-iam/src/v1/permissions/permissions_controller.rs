use axum::{
	Extension, Json,
	extract::{Path, Query},
	response::IntoResponse,
};

use crate::{
	AppState, MessageResponseDto, MetaRequestDto, ResponseListSuccessDto,
	ResponseSuccessDto,
	v1::{
		permissions_dto::{PermissionsItemDto, PermissionsRequestDto},
		permissions_service::PermissionsService,
	},
};

use super::{PermissionsEnum, permissions_guard};

#[utoipa::path(
	get,
	path = "/v1/permissions",
	 security(
        ("Bearer" = [])
    ),
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
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Query(meta): Query<MetaRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadListPermissions],
	)
	.await
	{
		Ok(_) => PermissionsService::get_permission_list(&state, meta).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	get,
	path = "/v1/permissions/detail/{id}",
	security(
        ("Bearer" = [])
    ),
	params(("id" = String, Path, description = "Permission ID")),
	responses(
		(status = 200, description = "Get permission by ID", body = ResponseSuccessDto<PermissionsItemDto>)
	),
	tag = "Permissions"
)]
pub async fn get_permission_by_id(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadDetailPermissions],
	)
	.await
	{
		Ok(_) => PermissionsService::get_permission_by_id(&state, id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	post,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/permissions/create",
	request_body = PermissionsRequestDto,
	responses(
		(status = 201, description = "Create new permission", body = MessageResponseDto)
	),
	tag = "Permissions"
)]
pub async fn post_create_permission(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Json(payload): Json<PermissionsRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::CreatePermissions],
	)
	.await
	{
		Ok(_) => PermissionsService::create_role(&state, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	put,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/permissions/update/{id}",
	request_body = PermissionsRequestDto,
	responses(
		(status = 200, description = "Update permission", body = MessageResponseDto)
	),
	tag = "Permissions"
)]
pub async fn put_update_permission(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
	Json(payload): Json<PermissionsRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::UpdatePermissions],
	)
	.await
	{
		Ok(_) => PermissionsService::update_permission(&state, payload, id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	delete,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/permissions/delete/{id}",
	responses(
		(status = 200, description = "Delete permission", body = MessageResponseDto)
	),
	tag = "Permissions"
)]
pub async fn delete_permission(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::DeletePermissions],
	)
	.await
	{
		Ok(_) => PermissionsService::delete_permission(&state, id).await,
		Err(response) => response,
	}
}

use crate::{AppState, MetaRequestDto};
use crate::{
	MessageResponseDto, PermissionsEnum, ResponseListSuccessDto, ResponseSuccessDto,
	UsersCreateRequestDto, UsersDetailItemDto, permissions_guard,
};
use axum::extract::{Path, Multipart};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

use super::{
	UsersActiveInactiveRequestDto, UsersListItemDto, UsersUpdateRequestDto,
};
use crate::v1::users::users_service::{UsersServiceTrait, UsersService};

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(description = "File upload form data for multipart/form-data")]
pub struct FileUploadSchema {
    /// Binary file data to upload
    #[schema(format = "binary")]
    pub file: String,
}

#[utoipa::path(
	get,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/users",
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
		(status = 200, description = "Get user list", body = ResponseListSuccessDto<Vec<UsersListItemDto>>)
	),
	tag = "Users"
)]
pub async fn get_user_list(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	axum::extract::Query(meta): axum::extract::Query<MetaRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		headers,
		Extension(state),
		vec![PermissionsEnum::ReadListUsers],
	)
	.await
	{
		Ok((_user, state)) => UsersService::get_user_list(&state, meta).await,
		Err(response) => response,
	}
}

#[utoipa::path(
get,
security(
       ("Bearer" = [])
   ),
path = "/v1/users/detail/{id}",
params(
	("id" = String, Path, description = "User ID")
),
responses(
	(status = 200, description = "Get user by ID", body = ResponseSuccessDto<UsersDetailItemDto>)
),
tag = "Users"
)]
pub async fn get_user_by_id(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		headers,
		Extension(state),
		vec![PermissionsEnum::ReadDetailUsers],
	)
	.await
	{
		Ok((_user, state)) => UsersService::get_user_by_id(&state, id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
get,
security(
       ("Bearer" = [])
   ),
path = "/v1/users/me",
responses(
	(status = 200, description = "Get user by ID", body = ResponseSuccessDto<UsersDetailItemDto>)
),
tag = "Users"
)]
pub async fn get_user_me(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
) -> impl IntoResponse {
	match permissions_guard(headers, Extension(state), vec![]).await {
		Ok((user, state)) => UsersService::get_user_me(user, &state).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	post,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/users/create",
	request_body = UsersCreateRequestDto,
	responses(
		(status = 200, description = "Create new user", body = ResponseSuccessDto<UsersDetailItemDto>)
	),
	tag = "Users"
)]
pub async fn post_create_user(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Json(payload): Json<UsersCreateRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		headers,
		Extension(state),
		vec![PermissionsEnum::CreateUsers],
	)
	.await
	{
		Ok((_user, state)) => UsersService::create_user(&state, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	put,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/users/update/{id}",
	params(
		("id" = String, Path, description = "User ID")
	),
	request_body = UsersUpdateRequestDto,
	responses(
		(status = 200, description = "Update user", body = ResponseSuccessDto<UsersDetailItemDto>)
	),
	tag = "Users"
)]
pub async fn put_update_user(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
	Json(payload): Json<UsersUpdateRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		headers,
		Extension(state),
		vec![PermissionsEnum::UpdateUsers],
	)
	.await
	{
		Ok((_user, state)) => UsersService::update_user(&state, id, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	put,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/users/update/me",
	request_body = UsersUpdateRequestDto,
	responses(
		(status = 200, description = "Update current user", body = ResponseSuccessDto<UsersDetailItemDto>)
	),
	tag = "Users"
)]
pub async fn put_update_user_me(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Json(payload): Json<UsersUpdateRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(headers.clone(), Extension(state), vec![]).await {
		Ok((_user, state)) => UsersService::update_user_me(headers, &state, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	put,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/users/activate/{id}",
	params(
		("id" = String, Path, description = "User ID")
	),
	request_body = UsersActiveInactiveRequestDto,
	responses(
		(status = 200, description = "Set user active status", body = MessageResponseDto)
	),
	tag = "Users"
)]
pub async fn patch_user_active_status(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
	Json(payload): Json<UsersActiveInactiveRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		headers,
		Extension(state),
		vec![PermissionsEnum::ActivateUsers],
	)
	.await
	{
		Ok((_user, state)) => UsersService::set_user_active_status(&state, id, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	delete,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/users/delete/{id}",
	responses(
		(status = 200, description = "Soft delete user", body = MessageResponseDto)
	),
	tag = "Users"
)]
pub async fn delete_user(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		headers,
		Extension(state),
		vec![PermissionsEnum::DeleteUsers],
	)
	.await
	{
		Ok((_user, state)) => UsersService::delete_user(&state, id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	post,
	security(
        ("Bearer" = [])
    ),
	path = "/v1/users/upload",
	request_body(
		content = FileUploadSchema,
		description = "Upload file with multipart form data. Only 'file' field is required - file type will be detected automatically from the uploaded file.",
		content_type = "multipart/form-data"
	),
	responses(
		(status = 200, description = "Upload file successfully", body = ResponseSuccessDto<serde_json::Value>),
		(status = 400, description = "Bad request"),
		(status = 401, description = "Unauthorized"),
		(status = 500, description = "Internal server error")
	),
	tag = "Users"
)]
pub async fn upload_file(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	multipart: Multipart,
) -> impl IntoResponse {
	// Check authentication first
	match permissions_guard(
		headers,
		Extension(state),
		vec![], // No specific permission needed, just authentication
	)
	.await
	{
		Ok((user, state)) => {
			// Extract user ID from user data
			let user_id = user.id.id.to_string(); // Use user.id.id to get the actual ID
			
			// Process upload - don't use match here since it returns Response directly
			UsersService::upload_file(&state, user_id, multipart).await
		},
		Err(response) => response,
	}
}

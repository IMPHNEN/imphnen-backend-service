use super::{ user_service::UserService, CreateUserRequestDto, UpdateRequestDto, DeleteRequestDto };
use crate::{v1::AuthLoginResponsetDto, AppState};
use crate::{MessageResponseDto, ResponseSuccessDto};
use axum::extract::Path;
use axum::{response::IntoResponse, Extension, Json};

#[utoipa::path(
    post,
    path = "/v1/user",
    request_body = CreateUserRequestDto,
    responses(
        (status = 200, description = "Create successful", body = ResponseSuccessDto<AuthLoginResponsetDto>),
        (status = 401, description = "Create failed", body = MessageResponseDto)
    ),
    tag = "UserManagement"
)]
pub async fn post_create_user(
	Extension(state): Extension<AppState>,
	Json(payload): Json<CreateUserRequestDto>,
) -> impl IntoResponse {
	UserService::mutation_create_user(payload, &state).await
}

#[utoipa::path(
    get,
    path = "/v1/user",
    responses(
        (status = 200, description = "Data exists", body = MessageResponseDto),
        (status = 401, description = "Bad request", body = MessageResponseDto)
    ),
    tag = "UserManagement"
)]
pub async fn get_user(
	Extension(state): Extension<AppState>,
) -> impl IntoResponse {
	UserService::read_all_user(&state).await
}

#[utoipa::path(
    put,
    path = "/v1/user",
    request_body = UpdateRequestDto,
    responses(
        (status = 200, description = "Update successful", body = MessageResponseDto),
        (status = 401, description = "Update failed", body = MessageResponseDto)
    ),
    tag = "UserManagement"
)]
pub async fn put_user(
	Extension(state): Extension<AppState>,
	Json(payload): Json<UpdateRequestDto>,
) -> impl IntoResponse {
	UserService::mutation_update_user(payload, &state).await
}

#[utoipa::path(
    delete,
    path = "/v1/user",
    request_body = DeleteRequestDto,
    responses(
        (status = 200, description = "Delete successful", body = MessageResponseDto),
        (status = 401, description = "Delete failed", body = MessageResponseDto)
    ),
    tag = "UserManagement"
)]
pub async fn delete_user(
	Extension(state): Extension<AppState>,
	Json(payload): Json<DeleteRequestDto>,
) -> impl IntoResponse {
	UserService::mutation_delete_user(payload, &state).await
}

#[utoipa::path(
    get,
    path = "/v1/user/:mail",
    responses(
        (status = 200, description = "Delete successful", body = MessageResponseDto),
        (status = 401, description = "Delete failed", body = MessageResponseDto)
    ),
    tag = "UserManagement"
)]
pub async fn get_user_by_email(
	Extension(state): Extension<AppState>,
	Path(mail): Path<String>
) -> impl IntoResponse {
	UserService::read_user_by_email(&mail, &state).await
}

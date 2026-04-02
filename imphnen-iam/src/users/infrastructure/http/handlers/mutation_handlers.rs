use super::super::dto::{
	UsersActiveInactiveRequestDto, UsersCreateRequestDto, UsersDetailItemDto,
	UsersUpdateRequestDto,
};
use crate::require_permissions;
use crate::users::domain::{UserEntity, UserService};
use axum::{
	Extension, Json, extract::Path, http::HeaderMap, response::IntoResponse,
};
use imphnen_entities::{PermissionsEnum, RolesDetailQueryDto};
use imphnen_libs::{AppState, hash_password};
use imphnen_utils::{ApiCreated, ApiMessage, AppError};
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/v1/iam/users/create",
    security(("Bearer" = [])),
    request_body = UsersCreateRequestDto,
    responses(
        (status = 201, description = "[ADMIN] Create new user", body = imphnen_entities::ResponseSuccessDto<UsersDetailItemDto>)
    ),
    tag = "Users"
)]
pub async fn post_create_user(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn UserService>>,
	Json(payload): Json<UsersCreateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(headers, state, [PermissionsEnum::CreateUsers], {
		let password_hash = hash_password(&payload.password).map_err(|_| {
			AppError::InternalServerError("Failed to hash password".to_string())
		})?;
		let role_id = payload.role_id.clone();
		let entity = UserEntity {
			id: Uuid::new_v4().to_string(),
			email: payload.email,
			fullname: payload.fullname,
			password: password_hash,
			is_active: payload.is_active,
			avatar: payload.avatar,
			role: RolesDetailQueryDto {
				id: role_id,
				..Default::default()
			},
			..Default::default()
		};
		let user = service.create(entity).await?;
		Ok(ApiCreated(UsersDetailItemDto::from(user)))
	})
}

#[utoipa::path(
    put,
    path = "/v1/iam/users/update/{id}",
    security(("Bearer" = [])),
    params(("id" = String, Path, description = "User ID")),
    request_body = UsersUpdateRequestDto,
    responses(
        (status = 200, description = "[ADMIN] Update user")
    ),
    tag = "Users"
)]
pub async fn put_update_user(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn UserService>>,
	Path(id): Path<String>,
	Json(payload): Json<UsersUpdateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(headers, state, [PermissionsEnum::UpdateUsers], {
		Uuid::parse_str(&id).map_err(|_| {
			AppError::BadRequestError("Invalid User ID format".to_string())
		})?;
		let current = service
			.get(id.clone())
			.await
			.map_err(|_| AppError::NotFoundError("User not found".to_string()))?;
		let password = match payload.password {
			Some(ref pw) => hash_password(pw).unwrap_or_else(|_| current.password.clone()),
			None => current.password.clone(),
		};
		let role_id = payload.role_id.clone().unwrap_or(current.role.id.clone());
		let entity = UserEntity {
			id: id.clone(),
			email: payload.email.unwrap_or(current.email),
			fullname: payload.fullname.unwrap_or(current.fullname),
			legal_name: payload.legal_name.or(current.legal_name),
			password,
			avatar: payload.avatar.or(current.avatar),
			is_active: payload.is_active.unwrap_or(current.is_active),
			is_deleted: current.is_deleted,
			role: RolesDetailQueryDto {
				id: role_id,
				..current.role
			},
			profile_extension: payload.profile_extension.or(current.profile_extension),
			created_at: current.created_at,
			updated_at: current.updated_at,
			mentor_id: current.mentor_id,
		};
		let msg = service.update(entity).await?;
		Ok(ApiMessage::ok(&msg))
	})
}

#[utoipa::path(
    put,
    path = "/v1/iam/users/activate/{id}",
    security(("Bearer" = [])),
    params(("id" = String, Path, description = "User ID")),
    request_body = UsersActiveInactiveRequestDto,
    responses(
        (status = 200, description = "[ADMIN] Set user active status")
    ),
    tag = "Users"
)]
pub async fn patch_user_active_status(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn UserService>>,
	Path(id): Path<String>,
	Json(payload): Json<UsersActiveInactiveRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(headers, state, [PermissionsEnum::ActivateUsers], {
		Uuid::parse_str(&id).map_err(|_| {
			AppError::BadRequestError("Invalid User ID format".to_string())
		})?;
		let msg = service.set_active_status(id, payload.is_active).await?;
		Ok(ApiMessage::ok(&msg))
	})
}

#[utoipa::path(
    delete,
    path = "/v1/iam/users/delete/{id}",
    security(("Bearer" = [])),
    params(("id" = String, Path, description = "User ID")),
    responses(
        (status = 200, description = "[ADMIN] Soft delete user")
    ),
    tag = "Users"
)]
pub async fn delete_user(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn UserService>>,
	Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(headers, state, [PermissionsEnum::DeleteUsers], {
		Uuid::parse_str(&id).map_err(|_| {
			AppError::BadRequestError("Invalid User ID format".to_string())
		})?;
		let msg = service.delete(id).await?;
		Ok(ApiMessage::ok(&msg))
	})
}

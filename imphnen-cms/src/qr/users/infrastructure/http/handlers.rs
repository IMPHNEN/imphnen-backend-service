use axum::{
	Extension, Json,
	extract::Path,
	response::{IntoResponse, Response},
};
use imphnen_utils::{errors::AppError, response_format::ApiSuccess};
use std::sync::Arc;
use uuid::Uuid;

use crate::qr::{
	middleware::qr_auth::QrAuthUser,
	users::{
		domain::{entity::UpdateUserInput, service::QrUserService},
		infrastructure::http::dto::{UpdateProfileRequest, UpdateRoleRequest},
	},
};

#[utoipa::path(
    get,
    path = "/v1/qr/users/me",
    responses(
        (status = 200, description = "Get my QR user profile",
         example = json!({
             "data": {
                 "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "email": "user@example.com",
                 "name": "Budi Santoso",
                 "role": "user",
                 "provider": "google",
                 "created_at": "2025-01-01T00:00:00Z",
                 "updated_at": "2025-01-01T00:00:00Z"
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized")
    ),
    tag = "QR - Users",
    security(("bearer_auth" = []))
)]
pub async fn get_me_handler(
	Extension(service): Extension<Arc<dyn QrUserService>>,
	Extension(auth_user): Extension<QrAuthUser>,
) -> Result<Response, AppError> {
	let user = service.get_profile(auth_user.user_id).await?;
	Ok(ApiSuccess(user).into_response())
}

#[utoipa::path(
    put,
    path = "/v1/qr/users/me",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Update my QR user profile",
         example = json!({
             "data": {
                 "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "email": "updated@example.com",
                 "name": "Budi Santoso Updated",
                 "role": "user",
                 "provider": "google",
                 "created_at": "2025-01-01T00:00:00Z",
                 "updated_at": "2025-01-15T00:00:00Z"
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized")
    ),
    tag = "QR - Users",
    security(("bearer_auth" = []))
)]
pub async fn update_me_handler(
	Extension(service): Extension<Arc<dyn QrUserService>>,
	Extension(auth_user): Extension<QrAuthUser>,
	Json(body): Json<UpdateProfileRequest>,
) -> Result<Response, AppError> {
	let input = UpdateUserInput {
		name: body.name,
		email: body.email,
	};
	let user = service.update_profile(auth_user.user_id, input).await?;
	Ok(ApiSuccess(user).into_response())
}

#[utoipa::path(
    get,
    path = "/v1/qr/users",
    responses(
        (status = 200, description = "Admin: list all QR users",
         example = json!({
             "data": [
                 {
                     "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                     "email": "user@example.com",
                     "name": "Budi Santoso",
                     "role": "user",
                     "provider": "google",
                     "created_at": "2025-01-01T00:00:00Z",
                     "updated_at": "2025-01-01T00:00:00Z"
                 },
                 {
                     "id": "4gb96g75-6828-5673-c4gd-3d074g77bgb7",
                     "email": "admin@example.com",
                     "name": "Admin User",
                     "role": "admin",
                     "provider": "google",
                     "created_at": "2024-12-01T00:00:00Z",
                     "updated_at": "2024-12-01T00:00:00Z"
                 }
             ],
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - admin only")
    ),
    tag = "QR - Users",
    security(("bearer_auth" = []))
)]
pub async fn list_users_handler(
	Extension(service): Extension<Arc<dyn QrUserService>>,
	Extension(auth_user): Extension<QrAuthUser>,
) -> Result<Response, AppError> {
	if auth_user.role != "admin" {
		return Err(AppError::ForbiddenError(
			"Admin access required".to_string(),
		));
	}
	let users = service.list_all().await?;
	Ok(ApiSuccess(users).into_response())
}

#[utoipa::path(
    put,
    path = "/v1/qr/users/{id}/role",
    params(("id" = Uuid, Path, description = "User ID")),
    request_body = UpdateRoleRequest,
    responses(
        (status = 200, description = "Admin: update user role",
         example = json!({
             "data": {
                 "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "email": "user@example.com",
                 "name": "Budi Santoso",
                 "role": "admin",
                 "provider": "google",
                 "created_at": "2025-01-01T00:00:00Z",
                 "updated_at": "2025-01-20T00:00:00Z"
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - admin only")
    ),
    tag = "QR - Users",
    security(("bearer_auth" = []))
)]
pub async fn update_role_handler(
	Extension(service): Extension<Arc<dyn QrUserService>>,
	Extension(auth_user): Extension<QrAuthUser>,
	Path(id): Path<Uuid>,
	Json(body): Json<UpdateRoleRequest>,
) -> Result<Response, AppError> {
	if auth_user.role != "admin" {
		return Err(AppError::ForbiddenError(
			"Admin access required".to_string(),
		));
	}
	let user = service.update_role(id, body.role).await?;
	Ok(ApiSuccess(user).into_response())
}

#[utoipa::path(
    delete,
    path = "/v1/qr/users/{id}",
    params(("id" = Uuid, Path, description = "User ID")),
    responses(
        (status = 200, description = "Admin: delete QR user",
         example = json!({"message": "User deleted successfully", "version": "0.3.0"})),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - admin only")
    ),
    tag = "QR - Users",
    security(("bearer_auth" = []))
)]
pub async fn delete_user_handler(
	Extension(service): Extension<Arc<dyn QrUserService>>,
	Extension(auth_user): Extension<QrAuthUser>,
	Path(id): Path<Uuid>,
) -> Result<Response, AppError> {
	if auth_user.role != "admin" {
		return Err(AppError::ForbiddenError(
			"Admin access required".to_string(),
		));
	}
	service.delete(id).await?;
	Ok(
		imphnen_utils::response_format::ApiMessage::ok("User deleted successfully")
			.into_response(),
	)
}

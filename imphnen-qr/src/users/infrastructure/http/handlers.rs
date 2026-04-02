use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Extension, Json,
};
use imphnen_utils::{errors::AppError, response_format::ApiSuccess};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    middleware::qr_auth::QrAuthUser,
    users::{
        domain::{entity::UpdateUserInput, service::QrUserService},
        infrastructure::http::dto::{UpdateProfileRequest, UpdateRoleRequest},
    },
};

pub async fn get_me_handler(
    Extension(service): Extension<Arc<dyn QrUserService>>,
    Extension(auth_user): Extension<QrAuthUser>,
) -> Result<Response, AppError> {
    let user = service.get_profile(auth_user.user_id).await?;
    Ok(ApiSuccess(user).into_response())
}

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

pub async fn list_users_handler(
    Extension(service): Extension<Arc<dyn QrUserService>>,
    Extension(auth_user): Extension<QrAuthUser>,
) -> Result<Response, AppError> {
    if auth_user.role != "admin" {
        return Err(AppError::ForbiddenError("Admin access required".to_string()));
    }
    let users = service.list_all().await?;
    Ok(ApiSuccess(users).into_response())
}

pub async fn update_role_handler(
    Extension(service): Extension<Arc<dyn QrUserService>>,
    Extension(auth_user): Extension<QrAuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateRoleRequest>,
) -> Result<Response, AppError> {
    if auth_user.role != "admin" {
        return Err(AppError::ForbiddenError("Admin access required".to_string()));
    }
    let user = service.update_role(id, body.role).await?;
    Ok(ApiSuccess(user).into_response())
}

pub async fn delete_user_handler(
    Extension(service): Extension<Arc<dyn QrUserService>>,
    Extension(auth_user): Extension<QrAuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Response, AppError> {
    if auth_user.role != "admin" {
        return Err(AppError::ForbiddenError("Admin access required".to_string()));
    }
    service.delete(id).await?;
    Ok(imphnen_utils::response_format::ApiMessage::ok("User deleted successfully").into_response())
}

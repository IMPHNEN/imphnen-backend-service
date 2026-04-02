use axum::{Extension, Json, extract::Path, response::IntoResponse};
use std::sync::Arc;
use uuid::Uuid;
use imphnen_utils::{errors::AppError, response_format::ApiSuccess};
use crate::middleware::hackathon_auth::HackathonAuthUser;
use crate::users::domain::service::HackathonUserService;
use super::dto::{UserResponse, UpdateUserRequest};

pub async fn get_me_handler(
    Extension(service): Extension<Arc<dyn HackathonUserService>>,
    Extension(auth): Extension<HackathonAuthUser>,
) -> Result<axum::response::Response, AppError> {
    let user = service.get_user(auth.user_id).await?;
    Ok(ApiSuccess(UserResponse::from(user)).into_response())
}

pub async fn update_me_handler(
    Extension(service): Extension<Arc<dyn HackathonUserService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Json(body): Json<UpdateUserRequest>,
) -> Result<axum::response::Response, AppError> {
    let user = service.update_user(auth.user_id, body.into()).await?;
    Ok(ApiSuccess(UserResponse::from(user)).into_response())
}

pub async fn get_user_handler(
    Extension(service): Extension<Arc<dyn HackathonUserService>>,
    Path(user_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
    let user = service.get_user(user_id).await?;
    Ok(ApiSuccess(UserResponse::from(user)).into_response())
}

pub async fn get_user_teams_handler(
    Extension(service): Extension<Arc<dyn HackathonUserService>>,
    Path(user_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
    let teams = service.get_user_teams(user_id).await?;
    Ok(ApiSuccess(teams).into_response())
}

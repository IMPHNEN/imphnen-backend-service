use super::dto::{UpdateUserRequest, UserResponse};
use crate::middleware::hackathon_auth::HackathonAuthUser;
use crate::users::domain::service::HackathonUserService;
use axum::{Extension, Json, extract::Path, response::IntoResponse};
use imphnen_utils::{errors::AppError, response_format::ApiSuccess};
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/v1/hackathon/users/me",
    responses(
        (status = 200, description = "Get my hackathon user profile"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Users",
    security(("bearer_auth" = []))
)]
pub async fn get_me_handler(
	Extension(service): Extension<Arc<dyn HackathonUserService>>,
	Extension(auth): Extension<HackathonAuthUser>,
) -> Result<axum::response::Response, AppError> {
	let user = service.get_user(auth.user_id).await?;
	Ok(ApiSuccess(UserResponse::from(user)).into_response())
}

#[utoipa::path(
    put,
    path = "/v1/hackathon/users/me",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "Update my hackathon user profile"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Users",
    security(("bearer_auth" = []))
)]
pub async fn update_me_handler(
	Extension(service): Extension<Arc<dyn HackathonUserService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Json(body): Json<UpdateUserRequest>,
) -> Result<axum::response::Response, AppError> {
	let user = service.update_user(auth.user_id, body.into()).await?;
	Ok(ApiSuccess(UserResponse::from(user)).into_response())
}

#[utoipa::path(
    get,
    path = "/v1/hackathon/users/{user_id}",
    params(("user_id" = Uuid, Path, description = "User ID")),
    responses(
        (status = 200, description = "Get hackathon user by ID"),
        (status = 404, description = "User not found")
    ),
    tag = "Hackathon - Users"
)]
pub async fn get_user_handler(
	Extension(service): Extension<Arc<dyn HackathonUserService>>,
	Path(user_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
	let user = service.get_user(user_id).await?;
	Ok(ApiSuccess(UserResponse::from(user)).into_response())
}

#[utoipa::path(
    get,
    path = "/v1/hackathon/users/{user_id}/teams",
    params(("user_id" = Uuid, Path, description = "User ID")),
    responses(
        (status = 200, description = "Get teams for a hackathon user"),
        (status = 404, description = "User not found")
    ),
    tag = "Hackathon - Users"
)]
pub async fn get_user_teams_handler(
	Extension(service): Extension<Arc<dyn HackathonUserService>>,
	Path(user_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
	let teams = service.get_user_teams(user_id).await?;
	Ok(ApiSuccess(teams).into_response())
}

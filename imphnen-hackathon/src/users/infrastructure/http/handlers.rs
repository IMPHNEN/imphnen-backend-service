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
        (status = 200, description = "Get my hackathon user profile",
         body = inline(UserResponse),
         example = json!({
             "data": {
                 "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "email": "user@example.com",
                 "fullname": "Budi Santoso",
                 "avatar": "https://cdn.example.com/avatar.png",
                 "phone_number": "+6281234567890",
                 "location": "Jakarta",
                 "bio": "Backend developer",
                 "skills": ["Rust", "Go", "PostgreSQL"],
                 "is_active": true,
                 "created_at": "2025-01-01T00:00:00Z",
                 "updated_at": "2025-01-10T00:00:00Z"
             },
             "version": "0.3.0"
         })),
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
        (status = 200, description = "Update my hackathon user profile",
         body = inline(UserResponse),
         example = json!({
             "data": {
                 "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "email": "user@example.com",
                 "fullname": "Budi Santoso",
                 "avatar": "https://cdn.example.com/avatar.png",
                 "phone_number": "+6281234567890",
                 "location": "Surabaya",
                 "bio": "Backend developer with 5 years of experience",
                 "skills": ["Rust", "Go", "PostgreSQL", "Redis"],
                 "is_active": true,
                 "created_at": "2025-01-01T00:00:00Z",
                 "updated_at": "2025-01-15T00:00:00Z"
             },
             "version": "0.3.0"
         })),
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
        (status = 200, description = "Get hackathon user by ID",
         body = inline(UserResponse),
         example = json!({
             "data": {
                 "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "email": "user@example.com",
                 "fullname": "Budi Santoso",
                 "avatar": null,
                 "phone_number": null,
                 "location": "Bandung",
                 "bio": null,
                 "skills": ["JavaScript", "React"],
                 "is_active": true,
                 "created_at": "2025-01-01T00:00:00Z",
                 "updated_at": "2025-01-01T00:00:00Z"
             },
             "version": "0.3.0"
         })),
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
        (status = 200, description = "Get teams for a hackathon user",
         example = json!({
             "data": [
                 {
                     "id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                     "name": "Rust Enjoyers",
                     "city": "Jakarta",
                     "visibility": "public",
                     "leader_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                     "member_count": 3
                 }
             ],
             "version": "0.3.0"
         })),
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

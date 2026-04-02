use axum::{middleware::from_fn, response::IntoResponse, routing::post, Extension, Json, Router};
use sqlx::PgPool;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use imphnen_utils::{errors::AppError, response_format::ApiSuccess};
use crate::common::hackathon_jwt::HackathonJwtService;
use crate::common::supabase_client::SupabaseClient;
use crate::middleware::hackathon_auth::{hackathon_auth_middleware, HackathonAuthUser};
use super::service::StorageService;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UploadRequest {
    pub filename: String,
    pub content_type: String,
    pub data: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UploadResponse {
    pub url: String,
}

async fn upload_file_handler(
    Extension(service): Extension<Arc<StorageService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Json(body): Json<UploadRequest>,
) -> Result<axum::response::Response, AppError> {
    let url = service.upload("uploads", auth.user_id, &body.filename, &body.content_type, &body.data).await?;
    Ok(ApiSuccess(UploadResponse { url }).into_response())
}

async fn upload_avatar_handler(
    Extension(service): Extension<Arc<StorageService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Json(body): Json<UploadRequest>,
) -> Result<axum::response::Response, AppError> {
    let url = service.upload("avatars", auth.user_id, &body.filename, &body.content_type, &body.data).await?;
    Ok(ApiSuccess(UploadResponse { url }).into_response())
}

async fn upload_team_handler(
    Extension(service): Extension<Arc<StorageService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Json(body): Json<UploadRequest>,
) -> Result<axum::response::Response, AppError> {
    let url = service.upload("teams", auth.user_id, &body.filename, &body.content_type, &body.data).await?;
    Ok(ApiSuccess(UploadResponse { url }).into_response())
}

async fn upload_submission_handler(
    Extension(service): Extension<Arc<StorageService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Json(body): Json<UploadRequest>,
) -> Result<axum::response::Response, AppError> {
    let url = service.upload("submissions", auth.user_id, &body.filename, &body.content_type, &body.data).await?;
    Ok(ApiSuccess(UploadResponse { url }).into_response())
}

pub fn hackathon_storage_routes(pool: Arc<PgPool>, jwt: Arc<HackathonJwtService>, supabase: Arc<SupabaseClient>) -> Router {
    let service = Arc::new(StorageService::new(supabase));
    Router::new()
        .route("/upload", post(upload_file_handler))
        .route("/upload/avatar", post(upload_avatar_handler))
        .route("/upload/team", post(upload_team_handler))
        .route("/upload/submission", post(upload_submission_handler))
        .layer(Extension(service))
        .layer(Extension(jwt.clone()))
        .layer(Extension(pool))
        .layer(from_fn(hackathon_auth_middleware))
}

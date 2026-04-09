use super::dto::{UploadRequest, UploadResponse};
use crate::middleware::hackathon_auth::HackathonAuthUser;
use crate::storage::domain::service::StorageService;
use axum::{Extension, Json, response::IntoResponse};
use imphnen_utils::{errors::AppError, response_format::ApiSuccess};
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/v1/hackathon/upload",
    request_body = UploadRequest,
    responses(
        (status = 200, description = "Upload a file (base64 encoded)",
         body = inline(UploadResponse),
         example = json!({
             "data": {
                 "url": "https://minio.example.com/uploads/3fa85f64-5717-4562-b3fc-2c963f66afa6/document.pdf"
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Storage",
    security(("Bearer" = []))
)]
pub async fn upload_file_handler(
	Extension(service): Extension<Arc<dyn StorageService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Json(body): Json<UploadRequest>,
) -> Result<axum::response::Response, AppError> {
	let url = service
		.upload(
			"uploads",
			auth.user_id,
			&body.filename,
			&body.content_type,
			&body.data,
		)
		.await?;
	Ok(ApiSuccess(UploadResponse { url }).into_response())
}

#[utoipa::path(
    post,
    path = "/v1/hackathon/upload/avatar",
    request_body = UploadRequest,
    responses(
        (status = 200, description = "Upload avatar image (base64 encoded)",
         body = inline(UploadResponse),
         example = json!({
             "data": {
                 "url": "https://minio.example.com/avatars/3fa85f64-5717-4562-b3fc-2c963f66afa6/avatar.png"
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Storage",
    security(("Bearer" = []))
)]
pub async fn upload_avatar_handler(
	Extension(service): Extension<Arc<dyn StorageService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Json(body): Json<UploadRequest>,
) -> Result<axum::response::Response, AppError> {
	let url = service
		.upload(
			"avatars",
			auth.user_id,
			&body.filename,
			&body.content_type,
			&body.data,
		)
		.await?;
	Ok(ApiSuccess(UploadResponse { url }).into_response())
}

#[utoipa::path(
    post,
    path = "/v1/hackathon/upload/team",
    request_body = UploadRequest,
    responses(
        (status = 200, description = "Upload team logo or banner (base64 encoded)",
         body = inline(UploadResponse),
         example = json!({
             "data": {
                 "url": "https://minio.example.com/teams/3fa85f64-5717-4562-b3fc-2c963f66afa6/logo.png"
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Storage",
    security(("Bearer" = []))
)]
pub async fn upload_team_handler(
	Extension(service): Extension<Arc<dyn StorageService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Json(body): Json<UploadRequest>,
) -> Result<axum::response::Response, AppError> {
	let url = service
		.upload(
			"teams",
			auth.user_id,
			&body.filename,
			&body.content_type,
			&body.data,
		)
		.await?;
	Ok(ApiSuccess(UploadResponse { url }).into_response())
}

#[utoipa::path(
    post,
    path = "/v1/hackathon/upload/submission",
    request_body = UploadRequest,
    responses(
        (status = 200, description = "Upload submission screenshot (base64 encoded)",
         body = inline(UploadResponse),
         example = json!({
             "data": {
                 "url": "https://minio.example.com/submissions/3fa85f64-5717-4562-b3fc-2c963f66afa6/screenshot.png"
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Storage",
    security(("Bearer" = []))
)]
pub async fn upload_submission_handler(
	Extension(service): Extension<Arc<dyn StorageService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Json(body): Json<UploadRequest>,
) -> Result<axum::response::Response, AppError> {
	let url = service
		.upload(
			"submissions",
			auth.user_id,
			&body.filename,
			&body.content_type,
			&body.data,
		)
		.await?;
	Ok(ApiSuccess(UploadResponse { url }).into_response())
}

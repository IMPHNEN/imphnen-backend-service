use super::dto::{UploadRequest, UploadResponse};
use crate::middleware::hackathon_auth::HackathonAuthUser;
use crate::storage::domain::service::StorageService;
use axum::{Extension, Json, response::IntoResponse};
use imphnen_utils::{errors::AppError, response_format::ApiSuccess};
use std::sync::Arc;

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

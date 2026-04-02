use axum::{Extension, Json, extract::Path, response::IntoResponse};
use std::sync::Arc;
use uuid::Uuid;
use imphnen_utils::{errors::AppError, response_format::ApiSuccess};
use crate::middleware::hackathon_auth::HackathonAuthUser;
use crate::submissions::domain::service::SubmissionService;
use super::dto::*;

pub async fn create_submission_handler(
    Extension(service): Extension<Arc<dyn SubmissionService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Path(team_id): Path<Uuid>,
    Json(body): Json<CreateSubmissionRequest>,
) -> Result<axum::response::Response, AppError> {
    let sub = service.create_submission(team_id, auth.user_id, body.into()).await?;
    Ok(ApiSuccess(SubmissionResponse::from(sub)).into_response())
}

pub async fn get_team_submission_handler(
    Extension(service): Extension<Arc<dyn SubmissionService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Path(team_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
    let sub = service.get_team_submission(team_id, auth.user_id).await?;
    Ok(ApiSuccess(SubmissionResponse::from(sub)).into_response())
}

pub async fn update_submission_handler(
    Extension(service): Extension<Arc<dyn SubmissionService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Path(submission_id): Path<Uuid>,
    Json(body): Json<UpdateSubmissionRequest>,
) -> Result<axum::response::Response, AppError> {
    let sub = service.update_submission(submission_id, auth.user_id, body.into()).await?;
    Ok(ApiSuccess(SubmissionResponse::from(sub)).into_response())
}

pub async fn submit_project_handler(
    Extension(service): Extension<Arc<dyn SubmissionService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Path(submission_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
    let sub = service.submit_project(submission_id, auth.user_id).await?;
    Ok(ApiSuccess(SubmissionResponse::from(sub)).into_response())
}

pub async fn confirm_submission_handler(
    Extension(service): Extension<Arc<dyn SubmissionService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Path(submission_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
    let sub = service.confirm_submission(submission_id, auth.user_id).await?;
    Ok(ApiSuccess(SubmissionResponse::from(sub)).into_response())
}

pub async fn cancel_submission_handler(
    Extension(service): Extension<Arc<dyn SubmissionService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Path(submission_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
    let sub = service.cancel_submission(submission_id, auth.user_id).await?;
    Ok(ApiSuccess(SubmissionResponse::from(sub)).into_response())
}

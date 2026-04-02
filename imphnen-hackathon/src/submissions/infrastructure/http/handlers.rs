use super::dto::*;
use crate::middleware::hackathon_auth::HackathonAuthUser;
use crate::submissions::domain::service::SubmissionService;
use axum::{Extension, Json, extract::Path, response::IntoResponse};
use imphnen_utils::{errors::AppError, response_format::ApiSuccess};
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/v1/hackathon/submissions/teams/{team_id}",
    params(("team_id" = Uuid, Path, description = "Team ID")),
    request_body = CreateSubmissionRequest,
    responses(
        (status = 200, description = "Create submission for team"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Submissions",
    security(("bearer_auth" = []))
)]
pub async fn create_submission_handler(
	Extension(service): Extension<Arc<dyn SubmissionService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path(team_id): Path<Uuid>,
	Json(body): Json<CreateSubmissionRequest>,
) -> Result<axum::response::Response, AppError> {
	let sub = service
		.create_submission(team_id, auth.user_id, body.into())
		.await?;
	Ok(ApiSuccess(SubmissionResponse::from(sub)).into_response())
}

#[utoipa::path(
    get,
    path = "/v1/hackathon/submissions/teams/{team_id}",
    params(("team_id" = Uuid, Path, description = "Team ID")),
    responses(
        (status = 200, description = "Get team submission"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not found")
    ),
    tag = "Hackathon - Submissions",
    security(("bearer_auth" = []))
)]
pub async fn get_team_submission_handler(
	Extension(service): Extension<Arc<dyn SubmissionService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path(team_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
	let sub = service.get_team_submission(team_id, auth.user_id).await?;
	Ok(ApiSuccess(SubmissionResponse::from(sub)).into_response())
}

#[utoipa::path(
    put,
    path = "/v1/hackathon/submissions/{submission_id}",
    params(("submission_id" = Uuid, Path, description = "Submission ID")),
    request_body = UpdateSubmissionRequest,
    responses(
        (status = 200, description = "Update submission"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Submissions",
    security(("bearer_auth" = []))
)]
pub async fn update_submission_handler(
	Extension(service): Extension<Arc<dyn SubmissionService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path(submission_id): Path<Uuid>,
	Json(body): Json<UpdateSubmissionRequest>,
) -> Result<axum::response::Response, AppError> {
	let sub = service
		.update_submission(submission_id, auth.user_id, body.into())
		.await?;
	Ok(ApiSuccess(SubmissionResponse::from(sub)).into_response())
}

#[utoipa::path(
    post,
    path = "/v1/hackathon/submissions/{submission_id}/submit",
    params(("submission_id" = Uuid, Path, description = "Submission ID")),
    responses(
        (status = 200, description = "Submit project"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Submissions",
    security(("bearer_auth" = []))
)]
pub async fn submit_project_handler(
	Extension(service): Extension<Arc<dyn SubmissionService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path(submission_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
	let sub = service.submit_project(submission_id, auth.user_id).await?;
	Ok(ApiSuccess(SubmissionResponse::from(sub)).into_response())
}

#[utoipa::path(
    post,
    path = "/v1/hackathon/submissions/{submission_id}/confirm",
    params(("submission_id" = Uuid, Path, description = "Submission ID")),
    responses(
        (status = 200, description = "Confirm submission"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Submissions",
    security(("bearer_auth" = []))
)]
pub async fn confirm_submission_handler(
	Extension(service): Extension<Arc<dyn SubmissionService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path(submission_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
	let sub = service
		.confirm_submission(submission_id, auth.user_id)
		.await?;
	Ok(ApiSuccess(SubmissionResponse::from(sub)).into_response())
}

#[utoipa::path(
    post,
    path = "/v1/hackathon/submissions/{submission_id}/cancel",
    params(("submission_id" = Uuid, Path, description = "Submission ID")),
    responses(
        (status = 200, description = "Cancel submission"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Submissions",
    security(("bearer_auth" = []))
)]
pub async fn cancel_submission_handler(
	Extension(service): Extension<Arc<dyn SubmissionService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path(submission_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
	let sub = service
		.cancel_submission(submission_id, auth.user_id)
		.await?;
	Ok(ApiSuccess(SubmissionResponse::from(sub)).into_response())
}

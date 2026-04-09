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
        (status = 200, description = "Create submission for team",
         body = inline(SubmissionResponse),
         example = json!({
             "data": {
                 "id": "c3d4e5f6-a7b8-9012-cdef-123456789012",
                 "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                 "project_name": "EcoTrack - Sustainability Monitor",
                 "description": "A real-time environmental monitoring platform",
                 "repository_url": "https://github.com/team/ecotrack",
                 "demo_url": "https://ecotrack.example.com",
                 "presentation_url": null,
                 "screenshots": [],
                 "status": "draft",
                 "submitted_at": null,
                 "submitted_by": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "created_at": "2025-01-05T00:00:00Z",
                 "updated_at": "2025-01-05T00:00:00Z"
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Submissions",
    security(("Bearer" = []))
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
        (status = 200, description = "Get team submission",
         body = inline(SubmissionResponse),
         example = json!({
             "data": {
                 "id": "c3d4e5f6-a7b8-9012-cdef-123456789012",
                 "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                 "project_name": "EcoTrack - Sustainability Monitor",
                 "description": "A real-time environmental monitoring platform",
                 "repository_url": "https://github.com/team/ecotrack",
                 "demo_url": "https://ecotrack.example.com",
                 "presentation_url": "https://slides.example.com/ecotrack",
                 "screenshots": ["https://cdn.example.com/ss1.png"],
                 "status": "submitted",
                 "submitted_at": "2025-01-20T12:00:00Z",
                 "submitted_by": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "created_at": "2025-01-05T00:00:00Z",
                 "updated_at": "2025-01-20T12:00:00Z"
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Submission not found")
    ),
    tag = "Hackathon - Submissions",
    security(("Bearer" = []))
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
        (status = 200, description = "Update submission",
         body = inline(SubmissionResponse),
         example = json!({
             "data": {
                 "id": "c3d4e5f6-a7b8-9012-cdef-123456789012",
                 "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                 "project_name": "EcoTrack v2 - Advanced Sustainability Monitor",
                 "description": "Updated description with more features",
                 "repository_url": "https://github.com/team/ecotrack",
                 "demo_url": "https://ecotrack-v2.example.com",
                 "presentation_url": "https://slides.example.com/ecotrack-v2",
                 "screenshots": ["https://cdn.example.com/ss1.png", "https://cdn.example.com/ss2.png"],
                 "status": "draft",
                 "submitted_at": null,
                 "submitted_by": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "created_at": "2025-01-05T00:00:00Z",
                 "updated_at": "2025-01-15T00:00:00Z"
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Submissions",
    security(("Bearer" = []))
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
        (status = 200, description = "Submit project for review",
         body = inline(SubmissionResponse),
         example = json!({
             "data": {
                 "id": "c3d4e5f6-a7b8-9012-cdef-123456789012",
                 "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                 "project_name": "EcoTrack - Sustainability Monitor",
                 "description": "A real-time environmental monitoring platform",
                 "repository_url": "https://github.com/team/ecotrack",
                 "demo_url": "https://ecotrack.example.com",
                 "presentation_url": "https://slides.example.com/ecotrack",
                 "screenshots": ["https://cdn.example.com/ss1.png"],
                 "status": "submitted",
                 "submitted_at": "2025-01-20T12:00:00Z",
                 "submitted_by": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "created_at": "2025-01-05T00:00:00Z",
                 "updated_at": "2025-01-20T12:00:00Z"
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Submissions",
    security(("Bearer" = []))
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
        (status = 200, description = "Confirm submission (admin)",
         body = inline(SubmissionResponse),
         example = json!({
             "data": {
                 "id": "c3d4e5f6-a7b8-9012-cdef-123456789012",
                 "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                 "project_name": "EcoTrack - Sustainability Monitor",
                 "description": "A real-time environmental monitoring platform",
                 "repository_url": "https://github.com/team/ecotrack",
                 "demo_url": "https://ecotrack.example.com",
                 "presentation_url": "https://slides.example.com/ecotrack",
                 "screenshots": ["https://cdn.example.com/ss1.png"],
                 "status": "confirmed",
                 "submitted_at": "2025-01-20T12:00:00Z",
                 "submitted_by": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "created_at": "2025-01-05T00:00:00Z",
                 "updated_at": "2025-01-21T08:00:00Z"
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Submissions",
    security(("Bearer" = []))
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
        (status = 200, description = "Cancel submission",
         body = inline(SubmissionResponse),
         example = json!({
             "data": {
                 "id": "c3d4e5f6-a7b8-9012-cdef-123456789012",
                 "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                 "project_name": "EcoTrack - Sustainability Monitor",
                 "description": "A real-time environmental monitoring platform",
                 "repository_url": "https://github.com/team/ecotrack",
                 "demo_url": "https://ecotrack.example.com",
                 "presentation_url": null,
                 "screenshots": [],
                 "status": "cancelled",
                 "submitted_at": null,
                 "submitted_by": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "created_at": "2025-01-05T00:00:00Z",
                 "updated_at": "2025-01-22T10:00:00Z"
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Submissions",
    security(("Bearer" = []))
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

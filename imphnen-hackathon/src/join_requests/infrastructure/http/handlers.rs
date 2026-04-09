use super::dto::*;
use crate::join_requests::domain::service::JoinRequestService;
use crate::middleware::hackathon_auth::HackathonAuthUser;
use axum::{Extension, Json, extract::Path, response::IntoResponse};
use imphnen_utils::{
	errors::AppError,
	response_format::{ApiMessage, ApiSuccess},
};
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/v1/hackathon/join-requests/teams/{team_id}",
    params(("team_id" = Uuid, Path, description = "Team ID")),
    request_body = CreateJoinRequestRequest,
    responses(
        (status = 200, description = "Create a join request",
         body = inline(JoinRequestResponse),
         example = json!({
             "data": {
                 "id": "b2c3d4e5-f6a7-8901-bcde-f12345678901",
                 "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                 "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "user_fullname": "Dewi Rahayu",
                 "user_email": "dewi@example.com",
                 "user_avatar": null,
                 "message": "I would love to join your team, I have experience in backend development",
                 "status": "pending",
                 "created_at": "2025-01-10T00:00:00Z"
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Join Requests",
    security(("Bearer" = []))
)]
pub async fn create_join_request_handler(
	Extension(service): Extension<Arc<dyn JoinRequestService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path(team_id): Path<Uuid>,
	Json(body): Json<CreateJoinRequestRequest>,
) -> Result<axum::response::Response, AppError> {
	let request = service
		.create_join_request(team_id, auth.user_id, body.into())
		.await?;
	Ok(ApiSuccess(JoinRequestResponse::from(request)).into_response())
}

#[utoipa::path(
    get,
    path = "/v1/hackathon/join-requests/my",
    responses(
        (status = 200, description = "Get my join requests",
         example = json!({
             "data": [
                 {
                     "id": "b2c3d4e5-f6a7-8901-bcde-f12345678901",
                     "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                     "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                     "user_fullname": "Dewi Rahayu",
                     "user_email": "dewi@example.com",
                     "user_avatar": null,
                     "message": "I would love to join your team",
                     "status": "pending",
                     "created_at": "2025-01-10T00:00:00Z"
                 }
             ],
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Join Requests",
    security(("Bearer" = []))
)]
pub async fn get_my_join_requests_handler(
	Extension(service): Extension<Arc<dyn JoinRequestService>>,
	Extension(auth): Extension<HackathonAuthUser>,
) -> Result<axum::response::Response, AppError> {
	let list = service.get_my_join_requests(auth.user_id).await?;
	let response: Vec<JoinRequestResponse> =
		list.into_iter().map(JoinRequestResponse::from).collect();
	Ok(ApiSuccess(response).into_response())
}

#[utoipa::path(
    get,
    path = "/v1/hackathon/join-requests/teams/{team_id}/pending",
    params(("team_id" = Uuid, Path, description = "Team ID")),
    responses(
        (status = 200, description = "Get pending join requests for team",
         example = json!({
             "data": [
                 {
                     "id": "b2c3d4e5-f6a7-8901-bcde-f12345678901",
                     "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                     "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                     "user_fullname": "Dewi Rahayu",
                     "user_email": "dewi@example.com",
                     "user_avatar": null,
                     "message": "I would love to join your team",
                     "status": "pending",
                     "created_at": "2025-01-10T00:00:00Z"
                 }
             ],
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - not team leader")
    ),
    tag = "Hackathon - Join Requests",
    security(("Bearer" = []))
)]
pub async fn get_team_join_requests_handler(
	Extension(service): Extension<Arc<dyn JoinRequestService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path(team_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
	let list = service
		.get_team_join_requests(team_id, auth.user_id)
		.await?;
	let response: Vec<JoinRequestResponse> =
		list.into_iter().map(JoinRequestResponse::from).collect();
	Ok(ApiSuccess(response).into_response())
}

#[utoipa::path(
    post,
    path = "/v1/hackathon/join-requests/{request_id}/respond",
    params(("request_id" = Uuid, Path, description = "Join request ID")),
    request_body = RespondToJoinRequestRequest,
    responses(
        (status = 200, description = "Respond to join request",
         example = json!({"message": "Join request accepted", "version": "0.3.0"})),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - not team leader")
    ),
    tag = "Hackathon - Join Requests",
    security(("Bearer" = []))
)]
pub async fn respond_to_join_request_handler(
	Extension(service): Extension<Arc<dyn JoinRequestService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path(request_id): Path<Uuid>,
	Json(body): Json<RespondToJoinRequestRequest>,
) -> Result<axum::response::Response, AppError> {
	service
		.respond_to_join_request(request_id, auth.user_id, body.accept)
		.await?;
	let msg = if body.accept {
		"Join request accepted"
	} else {
		"Join request rejected"
	};
	Ok(ApiMessage::ok(msg).into_response())
}

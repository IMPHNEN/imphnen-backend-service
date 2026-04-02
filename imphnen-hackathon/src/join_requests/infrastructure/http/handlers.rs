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
        (status = 200, description = "Create a join request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Join Requests",
    security(("bearer_auth" = []))
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
        (status = 200, description = "Get my join requests"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Join Requests",
    security(("bearer_auth" = []))
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
        (status = 200, description = "Get pending join requests for team"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    tag = "Hackathon - Join Requests",
    security(("bearer_auth" = []))
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
        (status = 200, description = "Respond to join request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    tag = "Hackathon - Join Requests",
    security(("bearer_auth" = []))
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

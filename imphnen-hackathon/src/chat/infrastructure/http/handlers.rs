use super::dto::*;
use crate::chat::domain::service::ChatService;
use crate::middleware::hackathon_auth::HackathonAuthUser;
use axum::{Extension, Json, extract::Path, response::IntoResponse};
use imphnen_utils::{
	errors::AppError,
	response_format::{ApiMessage, ApiSuccess},
};
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/v1/hackathon/chat/teams/{team_id}",
    params(("team_id" = Uuid, Path, description = "Team ID")),
    responses(
        (status = 200, description = "Get team chat messages"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Chat",
    security(("bearer_auth" = []))
)]
pub async fn get_team_messages_handler(
	Extension(service): Extension<Arc<dyn ChatService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path(team_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
	let messages = service.get_team_messages(team_id, auth.user_id).await?;
	let response: Vec<MessageResponse> =
		messages.into_iter().map(MessageResponse::from).collect();
	Ok(ApiSuccess(response).into_response())
}

#[utoipa::path(
    post,
    path = "/v1/hackathon/chat/teams/{team_id}",
    params(("team_id" = Uuid, Path, description = "Team ID")),
    request_body = SendMessageRequest,
    responses(
        (status = 200, description = "Send a message to team chat"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Chat",
    security(("bearer_auth" = []))
)]
pub async fn send_message_handler(
	Extension(service): Extension<Arc<dyn ChatService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path(team_id): Path<Uuid>,
	Json(body): Json<SendMessageRequest>,
) -> Result<axum::response::Response, AppError> {
	let message = service
		.send_message(team_id, auth.user_id, body.into())
		.await?;
	Ok(ApiSuccess(MessageResponse::from(message)).into_response())
}

#[utoipa::path(
    delete,
    path = "/v1/hackathon/chat/messages/{message_id}",
    params(("message_id" = Uuid, Path, description = "Message ID")),
    responses(
        (status = 200, description = "Delete a chat message"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    tag = "Hackathon - Chat",
    security(("bearer_auth" = []))
)]
pub async fn delete_message_handler(
	Extension(service): Extension<Arc<dyn ChatService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path(message_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
	service.delete_message(message_id, auth.user_id).await?;
	Ok(ApiMessage::ok("Message deleted").into_response())
}

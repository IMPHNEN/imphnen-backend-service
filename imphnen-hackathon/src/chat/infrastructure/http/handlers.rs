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
        (status = 200, description = "Get team chat messages",
         example = json!({
             "data": [
                 {
                     "id": "f6a7b8c9-d0e1-2345-fab0-456789012345",
                     "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                     "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                     "user_fullname": "Budi Santoso",
                     "user_avatar": null,
                     "message": "Hey team, let's finalize the project name!",
                     "created_at": "2025-01-15T09:00:00Z",
                     "updated_at": "2025-01-15T09:00:00Z"
                 },
                 {
                     "id": "a7b8c9d0-e1f2-3456-abc0-567890123456",
                     "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                     "user_id": "4gb96g75-6828-5673-c4gd-3d074g77bgb7",
                     "user_fullname": "Dewi Rahayu",
                     "user_avatar": "https://cdn.example.com/dewi.png",
                     "message": "I suggest EcoTrack!",
                     "created_at": "2025-01-15T09:05:00Z",
                     "updated_at": "2025-01-15T09:05:00Z"
                 }
             ],
             "version": "0.3.0"
         })),
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
        (status = 200, description = "Send a message to team chat",
         body = inline(MessageResponse),
         example = json!({
             "data": {
                 "id": "b8c9d0e1-f2a3-4567-bcd1-678901234567",
                 "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                 "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "user_fullname": "Budi Santoso",
                 "user_avatar": null,
                 "message": "Great, let's go with EcoTrack!",
                 "created_at": "2025-01-15T09:10:00Z",
                 "updated_at": "2025-01-15T09:10:00Z"
             },
             "version": "0.3.0"
         })),
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
        (status = 200, description = "Delete a chat message",
         example = json!({"message": "Message deleted", "version": "0.3.0"})),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - not message owner")
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

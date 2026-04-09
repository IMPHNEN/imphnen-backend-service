use super::dto::*;
use crate::invitations::domain::service::InvitationService;
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
    path = "/v1/hackathon/invitations/my",
    responses(
        (status = 200, description = "Get my invitations",
         example = json!({
             "data": [
                 {
                     "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
                     "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                     "team_name": "Rust Enjoyers",
                     "inviter_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                     "inviter_fullname": "Budi Santoso",
                     "invitee_email": "newmember@example.com",
                     "status": "pending",
                     "created_at": "2025-01-10T00:00:00Z"
                 }
             ],
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Invitations",
    security(("Bearer" = []))
)]
pub async fn get_my_invitations_handler(
	Extension(service): Extension<Arc<dyn InvitationService>>,
	Extension(auth): Extension<HackathonAuthUser>,
) -> Result<axum::response::Response, AppError> {
	let list = service.get_my_invitations(auth.user_id).await?;
	let response: Vec<InvitationResponse> =
		list.into_iter().map(InvitationResponse::from).collect();
	Ok(ApiSuccess(response).into_response())
}

#[utoipa::path(
    post,
    path = "/v1/hackathon/invitations/{invitation_id}/respond",
    params(("invitation_id" = Uuid, Path, description = "Invitation ID")),
    request_body = RespondToInvitationRequest,
    responses(
        (status = 200, description = "Respond to invitation",
         example = json!({"message": "Invitation accepted", "version": "0.3.0"})),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Invitation not found")
    ),
    tag = "Hackathon - Invitations",
    security(("Bearer" = []))
)]
pub async fn respond_to_invitation_handler(
	Extension(service): Extension<Arc<dyn InvitationService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path(invitation_id): Path<Uuid>,
	Json(body): Json<RespondToInvitationRequest>,
) -> Result<axum::response::Response, AppError> {
	service
		.respond_to_invitation(invitation_id, auth.user_id, body.accept)
		.await?;
	let msg = if body.accept {
		"Invitation accepted"
	} else {
		"Invitation declined"
	};
	Ok(ApiMessage::ok(msg).into_response())
}

#[utoipa::path(
    post,
    path = "/v1/hackathon/invitations/teams/{team_id}/invite",
    params(("team_id" = Uuid, Path, description = "Team ID")),
    request_body = CreateInvitationRequest,
    responses(
        (status = 200, description = "Invite a member to team",
         body = inline(InvitationResponse),
         example = json!({
             "data": {
                 "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
                 "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                 "team_name": "Rust Enjoyers",
                 "inviter_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "inviter_fullname": "Budi Santoso",
                 "invitee_email": "newmember@example.com",
                 "status": "pending",
                 "created_at": "2025-01-10T00:00:00Z"
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - not team leader")
    ),
    tag = "Hackathon - Invitations",
    security(("Bearer" = []))
)]
pub async fn invite_team_member_handler(
	Extension(service): Extension<Arc<dyn InvitationService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path(team_id): Path<Uuid>,
	Json(body): Json<CreateInvitationRequest>,
) -> Result<axum::response::Response, AppError> {
	let invitation = service
		.invite_member(team_id, auth.user_id, body.into())
		.await?;
	Ok(ApiSuccess(InvitationResponse::from(invitation)).into_response())
}

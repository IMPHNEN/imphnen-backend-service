use axum::{Extension, Json, extract::Path, response::IntoResponse};
use std::sync::Arc;
use uuid::Uuid;
use imphnen_utils::{errors::AppError, response_format::{ApiSuccess, ApiMessage}};
use crate::middleware::hackathon_auth::HackathonAuthUser;
use crate::invitations::domain::service::InvitationService;
use super::dto::*;

pub async fn get_my_invitations_handler(
    Extension(service): Extension<Arc<dyn InvitationService>>,
    Extension(auth): Extension<HackathonAuthUser>,
) -> Result<axum::response::Response, AppError> {
    let list = service.get_my_invitations(auth.user_id).await?;
    let response: Vec<InvitationResponse> = list.into_iter().map(InvitationResponse::from).collect();
    Ok(ApiSuccess(response).into_response())
}

pub async fn respond_to_invitation_handler(
    Extension(service): Extension<Arc<dyn InvitationService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Path(invitation_id): Path<Uuid>,
    Json(body): Json<RespondToInvitationRequest>,
) -> Result<axum::response::Response, AppError> {
    service.respond_to_invitation(invitation_id, auth.user_id, body.accept).await?;
    let msg = if body.accept { "Invitation accepted" } else { "Invitation declined" };
    Ok(ApiMessage::ok(msg).into_response())
}

pub async fn invite_team_member_handler(
    Extension(service): Extension<Arc<dyn InvitationService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Path(team_id): Path<Uuid>,
    Json(body): Json<CreateInvitationRequest>,
) -> Result<axum::response::Response, AppError> {
    let invitation = service.invite_member(team_id, auth.user_id, body.into()).await?;
    Ok(ApiSuccess(InvitationResponse::from(invitation)).into_response())
}

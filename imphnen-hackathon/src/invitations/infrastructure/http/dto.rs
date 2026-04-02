use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::invitations::domain::entity::*;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InvitationResponse {
    pub id: Uuid,
    pub team_id: Uuid,
    pub team_name: String,
    pub inviter_id: Uuid,
    pub inviter_fullname: String,
    pub invitee_email: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl From<InvitationWithDetails> for InvitationResponse {
    fn from(e: InvitationWithDetails) -> Self {
        Self {
            id: e.id,
            team_id: e.team_id,
            team_name: e.team_name,
            inviter_id: e.inviter_id,
            inviter_fullname: e.inviter_fullname,
            invitee_email: e.invitee_email,
            status: e.status,
            created_at: e.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RespondToInvitationRequest {
    pub accept: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateInvitationRequest {
    pub invitee_email: String,
}

impl From<CreateInvitationRequest> for CreateInvitationInput {
    fn from(r: CreateInvitationRequest) -> Self {
        Self {
            invitee_email: r.invitee_email,
        }
    }
}

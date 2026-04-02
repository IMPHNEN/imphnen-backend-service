use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::join_requests::domain::entity::*;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct JoinRequestResponse {
    pub id: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub user_fullname: String,
    pub user_email: String,
    pub user_avatar: Option<String>,
    pub message: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl From<JoinRequestWithDetails> for JoinRequestResponse {
    fn from(e: JoinRequestWithDetails) -> Self {
        Self {
            id: e.id,
            team_id: e.team_id,
            user_id: e.user_id,
            user_fullname: e.user_fullname,
            user_email: e.user_email,
            user_avatar: e.user_avatar,
            message: e.message,
            status: e.status,
            created_at: e.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateJoinRequestRequest {
    pub message: String,
}

impl From<CreateJoinRequestRequest> for CreateJoinRequestInput {
    fn from(r: CreateJoinRequestRequest) -> Self {
        Self { message: r.message }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RespondToJoinRequestRequest {
    pub accept: bool,
}

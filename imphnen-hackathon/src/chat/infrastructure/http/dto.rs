use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::chat::domain::entity::*;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub id: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub user_fullname: String,
    pub user_avatar: Option<String>,
    pub message: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<MessageWithUser> for MessageResponse {
    fn from(e: MessageWithUser) -> Self {
        Self {
            id: e.id,
            team_id: e.team_id,
            user_id: e.user_id,
            user_fullname: e.user_fullname,
            user_avatar: e.user_avatar,
            message: e.message,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SendMessageRequest {
    pub message: String,
}

impl From<SendMessageRequest> for SendMessageInput {
    fn from(r: SendMessageRequest) -> Self {
        Self { message: r.message }
    }
}

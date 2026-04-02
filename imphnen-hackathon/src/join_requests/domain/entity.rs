use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct JoinRequestEntity {
    pub id: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub message: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct JoinRequestWithDetails {
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

#[derive(Debug, Default)]
pub struct CreateJoinRequestInput {
    pub message: String,
}

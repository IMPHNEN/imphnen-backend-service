use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct HackathonUserEntity {
    pub id: Uuid,
    pub email: String,
    pub fullname: String,
    pub avatar: Option<String>,
    pub phone_number: Option<String>,
    pub location: Option<String>,
    pub bio: Option<String>,
    pub skills: Option<Vec<String>>,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateUserInput {
    pub fullname: Option<String>,
    pub phone_number: Option<String>,
    pub avatar: Option<String>,
    pub location: Option<String>,
    pub bio: Option<String>,
    pub skills: Option<Vec<String>>,
}

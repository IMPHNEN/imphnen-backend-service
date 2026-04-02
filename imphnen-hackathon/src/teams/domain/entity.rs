use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct TeamEntity {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub city: String,
    pub visibility: String,
    pub logo: Option<String>,
    pub banner: Option<String>,
    pub leader_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct TeamUserInfo {
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

#[derive(Debug, Clone)]
pub struct TeamMemberEntity {
    pub id: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub user: TeamUserInfo,
    pub role: String,
    pub status: String,
    pub joined_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct TeamWithDetails {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub city: String,
    pub visibility: String,
    pub logo: Option<String>,
    pub banner: Option<String>,
    pub leader_id: Uuid,
    pub leader: Option<TeamUserInfo>,
    pub members: Option<Vec<TeamMemberEntity>>,
    pub member_count: Option<i64>,
    pub has_submission: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default)]
pub struct CreateTeamInput {
    pub name: String,
    pub description: Option<String>,
    pub city: String,
    pub visibility: String,
    pub logo: Option<String>,
    pub banner: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateTeamInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub city: Option<String>,
    pub visibility: Option<String>,
    pub logo: Option<String>,
    pub banner: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct BrowseTeamsInput {
    pub search: Option<String>,
    pub city: Option<String>,
    pub min_members: Option<i64>,
    pub max_members: Option<i64>,
    pub has_submission: Option<bool>,
    pub page: i64,
    pub per_page: i64,
}

pub struct BrowseTeamsResult {
    pub teams: Vec<TeamWithDetails>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    #[serde(rename = "registration_approved")]
    RegistrationApproved,
    #[serde(rename = "registration_rejected")]
    RegistrationRejected,
    #[serde(rename = "registration_waitlisted")]
    RegistrationWaitlisted,
    #[serde(rename = "hackathon_reminder")]
    HackathonReminder,
    #[serde(rename = "team_invite")]
    TeamInvite,
    #[serde(rename = "team_update")]
    TeamUpdate,
    #[serde(rename = "hackathon_update")]
    HackathonUpdate,
    #[serde(rename = "check_in_reminder")]
    CheckInReminder,
    #[serde(rename = "announcement")]
    Announcement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSchema {
    pub id: Thing,
    pub user_id: Thing,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
    pub read_at: Option<DateTime<Utc>>,
    pub related_id: Option<Thing>, // Could be hackathon_id, registration_id, team_id, etc.
    pub action_url: Option<String>,
    pub metadata: Option<serde_json::Value>, // For additional flexible data
}

impl NotificationSchema {
    pub fn mark_as_read(&mut self) {
        self.is_read = true;
        self.read_at = Some(Utc::now());
    }
}

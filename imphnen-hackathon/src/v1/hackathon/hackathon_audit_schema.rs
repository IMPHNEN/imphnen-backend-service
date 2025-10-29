use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use imphnen_utils::{get_iso_date, make_thing};

/// Audit log schema for tracking all hackathon-related changes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HackathonAuditLogSchema {
    pub id: Thing,
    pub hackathon_id: Option<Thing>,  // None for system-wide events
    pub action: AuditAction,
    pub actor_id: String,              // User who performed the action
    pub actor_email: Option<String>,   // For better traceability
    pub resource_type: String,         // hackathon, timeline, event, submission
    pub resource_id: Option<String>,   // ID of the affected resource
    pub changes: Option<serde_json::Value>,  // JSON of what changed
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AuditAction {
    // Hackathon actions
    HackathonCreated,
    HackathonUpdated,
    HackathonDeleted,
    HackathonStatusChanged,
    
    // Timeline actions
    TimelineCreated,
    TimelineUpdated,
    TimelineDeleted,
    TimelineActivated,
    
    // Event actions
    EventCreated,
    EventUpdated,
    EventDeleted,
    
    // Submission actions
    SubmissionCreated,
    SubmissionUpdated,
    SubmissionDeleted,
    SubmissionStatusChanged,
    
    // Participant actions
    ParticipantRegistered,
    ParticipantRemoved,
    
    // Organizer actions
    OrganizerAdded,
    OrganizerRemoved,
}

impl std::fmt::Display for AuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditAction::HackathonCreated => write!(f, "hackathon_created"),
            AuditAction::HackathonUpdated => write!(f, "hackathon_updated"),
            AuditAction::HackathonDeleted => write!(f, "hackathon_deleted"),
            AuditAction::HackathonStatusChanged => write!(f, "hackathon_status_changed"),
            AuditAction::TimelineCreated => write!(f, "timeline_created"),
            AuditAction::TimelineUpdated => write!(f, "timeline_updated"),
            AuditAction::TimelineDeleted => write!(f, "timeline_deleted"),
            AuditAction::TimelineActivated => write!(f, "timeline_activated"),
            AuditAction::EventCreated => write!(f, "event_created"),
            AuditAction::EventUpdated => write!(f, "event_updated"),
            AuditAction::EventDeleted => write!(f, "event_deleted"),
            AuditAction::SubmissionCreated => write!(f, "submission_created"),
            AuditAction::SubmissionUpdated => write!(f, "submission_updated"),
            AuditAction::SubmissionDeleted => write!(f, "submission_deleted"),
            AuditAction::SubmissionStatusChanged => write!(f, "submission_status_changed"),
            AuditAction::ParticipantRegistered => write!(f, "participant_registered"),
            AuditAction::ParticipantRemoved => write!(f, "participant_removed"),
            AuditAction::OrganizerAdded => write!(f, "organizer_added"),
            AuditAction::OrganizerRemoved => write!(f, "organizer_removed"),
        }
    }
}

impl Default for HackathonAuditLogSchema {
    fn default() -> Self {
        Self {
            id: make_thing(
                "app_hackathon_audit_logs",
                &surrealdb::Uuid::new_v4().to_string(),
            ),
            hackathon_id: None,
            action: AuditAction::HackathonCreated,
            actor_id: String::new(),
            actor_email: None,
            resource_type: String::new(),
            resource_id: None,
            changes: None,
            old_value: None,
            new_value: None,
            ip_address: None,
            user_agent: None,
            timestamp: Utc::now(),
            created_at: get_iso_date(),
        }
    }
}

impl HackathonAuditLogSchema {
    pub fn new(
        hackathon_id: Option<Thing>,
        action: AuditAction,
        actor_id: String,
        resource_type: String,
        resource_id: Option<String>,
    ) -> Self {
        Self {
            id: make_thing(
                "app_hackathon_audit_logs",
                &surrealdb::Uuid::new_v4().to_string(),
            ),
            hackathon_id,
            action,
            actor_id,
            actor_email: None,
            resource_type,
            resource_id,
            changes: None,
            old_value: None,
            new_value: None,
            ip_address: None,
            user_agent: None,
            timestamp: Utc::now(),
            created_at: get_iso_date(),
        }
    }

    pub fn with_changes(mut self, changes: serde_json::Value) -> Self {
        self.changes = Some(changes);
        self
    }

    pub fn with_old_new_values(
        mut self,
        old_value: serde_json::Value,
        new_value: serde_json::Value,
    ) -> Self {
        self.old_value = Some(old_value);
        self.new_value = Some(new_value);
        self
    }

    pub fn with_request_info(
        mut self,
        ip_address: Option<String>,
        user_agent: Option<String>,
        actor_email: Option<String>,
    ) -> Self {
        self.ip_address = ip_address;
        self.user_agent = user_agent;
        self.actor_email = actor_email;
        self
    }
}

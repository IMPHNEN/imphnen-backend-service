use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use imphnen_utils::make_thing;
use imphnen_utils::get_iso_date;
use imphnen_libs::ResourceEnum;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HackathonSchema {
    pub id: Thing,
    pub name: String,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub registration_deadline: DateTime<Utc>,
    pub max_participants: Option<u32>,
    pub status: HackathonStatus,
    pub theme: Option<String>,
    pub rules: Option<String>,
    pub prizes: Option<Vec<Prize>>,
    pub previous_winners: Option<Vec<Winner>>,
    pub organizers: Vec<String>, // User IDs
    pub is_deleted: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HackathonEventsSchema {
    pub id: Thing,
    pub hackathon_id: Thing,
    pub title: String,
    pub description: Option<String>,
    pub event_type: HackathonEventType,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub location: Option<String>,
    pub virtual_link: Option<String>,
    pub max_attendees: Option<u32>,
    pub is_mandatory: bool,
    pub is_deleted: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HackathonTimelineSchema {
    pub id: Thing,
    pub hackathon_id: Thing,
    pub phase: HackathonPhase,
    pub title: String,
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub is_active: bool,
    pub order: u32,
    pub is_deleted: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HackathonSubmissionsSchema {
    pub id: Thing,
    pub hackathon_id: Thing,
    pub team_id: Thing,
    pub project_name: String,
    pub description: String,
    pub repository_url: Option<String>,
    pub demo_url: Option<String>,
    pub slides_url: Option<String>,
    pub technologies: Vec<String>,
    pub submission_status: SubmissionStatus,
    pub submitted_at: DateTime<Utc>,
    pub is_deleted: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Prize {
    pub position: u32,
    pub title: String,
    pub description: Option<String>,
    pub value: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Winner {
    pub position: u32,
    pub team_id: String,
    pub project_name: String,
    pub team_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, utoipa::ToSchema)]
pub enum HackathonStatus {
    Draft,
    RegistrationOpen,
    RegistrationClosed,
    InProgress,
    Judging,
    Completed,
    Cancelled,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, utoipa::ToSchema)]
pub enum HackathonEventType {
    Workshop,
    Keynote,
    Networking,
    Judging,
    Ceremony,
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, utoipa::ToSchema)]
pub enum HackathonPhase {
    Registration,
    Ideation,
    Development,
    Submission,
    Judging,
    Awards,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, utoipa::ToSchema)]
pub enum SubmissionStatus {
    Draft,
    Submitted,
    UnderReview,
    Shortlisted,
    Winner,
    Rejected,
}

impl Default for HackathonSchema {
    fn default() -> Self {
        HackathonSchema {
            id: make_thing(
                &ResourceEnum::Hackathons.to_string(),
                &surrealdb::Uuid::new_v4().to_string(),
            ),
            name: String::new(),
            description: String::new(),
            start_date: Utc::now(),
            end_date: Utc::now(),
            registration_deadline: Utc::now(),
            max_participants: None,
            status: HackathonStatus::Draft,
            theme: None,
            rules: None,
            prizes: None,
            previous_winners: None,
            organizers: vec![],
            is_deleted: false,
            created_at: Some(get_iso_date()),
            updated_at: Some(get_iso_date()),
        }
    }
}

impl Default for HackathonEventsSchema {
    fn default() -> Self {
        HackathonEventsSchema {
            id: make_thing(
                &ResourceEnum::HackathonEvents.to_string(),
                &surrealdb::Uuid::new_v4().to_string(),
            ),
            hackathon_id: Thing::from(("app_hackathons".to_string(), surrealdb::sql::Id::rand())),
            title: String::new(),
            description: None,
            event_type: HackathonEventType::Other,
            start_time: Utc::now(),
            end_time: Utc::now(),
            location: None,
            virtual_link: None,
            max_attendees: None,
            is_mandatory: false,
            is_deleted: false,
            created_at: Some(get_iso_date()),
            updated_at: Some(get_iso_date()),
        }
    }
}

impl Default for HackathonTimelineSchema {
    fn default() -> Self {
        HackathonTimelineSchema {
            id: make_thing(
                &ResourceEnum::HackathonTimeline.to_string(),
                &surrealdb::Uuid::new_v4().to_string(),
            ),
            hackathon_id: Thing::from(("app_hackathons".to_string(), surrealdb::sql::Id::rand())),
            phase: HackathonPhase::Registration,
            title: String::new(),
            description: None,
            start_date: Utc::now(),
            end_date: Utc::now(),
            is_active: false,
            order: 0,
            is_deleted: false,
            created_at: Some(get_iso_date()),
            updated_at: Some(get_iso_date()),
        }
    }
}

impl Default for HackathonSubmissionsSchema {
    fn default() -> Self {
        HackathonSubmissionsSchema {
            id: make_thing(
                &ResourceEnum::HackathonSubmissions.to_string(),
                &surrealdb::Uuid::new_v4().to_string(),
            ),
            hackathon_id: Thing::from(("app_hackathons".to_string(), surrealdb::sql::Id::rand())),
            team_id: Thing::from(("app_teams".to_string(), surrealdb::sql::Id::rand())),
            project_name: String::new(),
            description: String::new(),
            repository_url: None,
            demo_url: None,
            slides_url: None,
            technologies: vec![],
            submission_status: SubmissionStatus::Draft,
            submitted_at: Utc::now(),
            is_deleted: false,
            created_at: Some(get_iso_date()),
            updated_at: Some(get_iso_date()),
        }
    }
}
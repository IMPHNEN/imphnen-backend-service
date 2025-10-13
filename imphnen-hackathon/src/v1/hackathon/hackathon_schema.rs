use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Deserializer};
use std::str::FromStr;
use serde::de;
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
    pub team_id: Option<Thing>,
    pub project_name: Option<String>,
    pub description: Option<String>,
    pub repository_url: Option<String>,
    pub demo_url: Option<String>,
    pub slides_url: Option<String>,
    pub technologies: Option<Vec<String>>,
    pub submission_status: Option<SubmissionStatus>,
    pub judge_feedback: Option<String>,
    pub submitted_at: Option<DateTime<Utc>>,
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, utoipa::ToSchema, strum::Display)]
pub enum HackathonStatus {
    Draft,
    RegistrationOpen,
    RegistrationClosed,
    InProgress,
    Judging,
    Completed,
    Cancelled,
}

#[derive(Clone, Debug, Serialize, PartialEq, utoipa::ToSchema, strum::Display)]
pub enum HackathonPhase {
    Registration,
    Ideation,
    Development,
    Submission,
    Judging,
    Awards,
}

// Add as_str method for HackathonPhase
impl HackathonPhase {
    pub fn as_str(&self) -> &str {
        match self {
            HackathonPhase::Registration => "registration",
            HackathonPhase::Ideation => "ideation",
            HackathonPhase::Development => "development",
            HackathonPhase::Submission => "submission",
            HackathonPhase::Judging => "judging",
            HackathonPhase::Awards => "awards",
        }
    }
}

// Manual Deserialize implementation for case-insensitive support
impl<'de> Deserialize<'de> for HackathonPhase {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let normalized = s.to_lowercase();
        
        match normalized.as_str() {
            "registration" => Ok(HackathonPhase::Registration),
            "ideation" => Ok(HackathonPhase::Ideation),
            "development" => Ok(HackathonPhase::Development),
            "submission" => Ok(HackathonPhase::Submission),
            "judging" => Ok(HackathonPhase::Judging),
            "awards" => Ok(HackathonPhase::Awards),
            _ => Err(serde::de::Error::custom(format!("Invalid HackathonPhase: {}", s)))
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, utoipa::ToSchema, strum::Display)]
pub enum HackathonEventType {
    Workshop,
    Keynote,
    Networking,
    Judging,
    Ceremony,
    Other,
}

// Implement case-insensitive string parsing for HackathonEventType
impl FromStr for HackathonEventType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "workshop" => Ok(Self::Workshop),
            "keynote" => Ok(Self::Keynote),
            "networking" => Ok(Self::Networking),
            "judging" => Ok(Self::Judging),
            "ceremony" => Ok(Self::Ceremony),
            "other" => Ok(Self::Other),
            _ => Err(format!("Invalid HackathonEventType: {}", s)),
        }
    }
}

// Manual Deserialize implementation for case-insensitive support
impl<'de> Deserialize<'de> for HackathonEventType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(de::Error::custom)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, utoipa::ToSchema, strum::Display)]
pub enum SubmissionStatus {
    Draft,
    Submitted,
    Accepted,
    UnderReview,
    Shortlisted,
    Winner,
    Rejected,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HackathonParticipantSchema {
    pub id: Thing,
    pub hackathon_id: Thing,
    pub user_id: String,
    pub is_deleted: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Default for HackathonParticipantSchema {
    fn default() -> Self {
        HackathonParticipantSchema {
            id: make_thing(
                "app_hackathon_participants",
                &surrealdb::Uuid::new_v4().to_string(),
            ),
            hackathon_id: Thing::from(("app_hackathons".to_string(), surrealdb::sql::Id::rand())),
            user_id: String::new(),
            is_deleted: false,
            created_at: Some(get_iso_date()),
            updated_at: Some(get_iso_date()),
        }
    }
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
            team_id: Some(Thing::from(("app_teams".to_string(), surrealdb::sql::Id::rand()))),
            project_name: Some(String::new()),
            description: Some(String::new()),
            repository_url: None,
            demo_url: None,
            slides_url: None,
            technologies: Some(vec![]),
            submission_status: Some(SubmissionStatus::Draft),
            judge_feedback: None,
            submitted_at: Some(Utc::now()),
            is_deleted: false,
            created_at: Some(get_iso_date()),
            updated_at: Some(get_iso_date()),
        }
    }
}
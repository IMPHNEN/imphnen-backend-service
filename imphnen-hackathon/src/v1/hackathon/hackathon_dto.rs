use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, schema};
use validator::Validate;

use crate::v1::hackathon::hackathon_schema::{
    HackathonEventType, HackathonEventsSchema, HackathonPhase, HackathonSchema,
    HackathonStatus, HackathonSubmissionsSchema, HackathonTimelineSchema,
    SubmissionStatus,
};

// Hackathon DTOs
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct HackathonCreateRequestDto {
    #[validate(length(min = 1, max = 100, message = "Hackathon name must be between 1 and 100 characters"))]
    pub name: String,
    #[validate(length(min = 1, max = 1000, message = "Description must be between 1 and 1000 characters"))]
    pub description: String,
    #[schema(value_type = String, format = DateTime)]
    pub start_date: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub end_date: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub registration_deadline: DateTime<Utc>,
    #[validate(range(min = 1, max = 10000, message = "Max participants must be between 1 and 10000"))]
    pub max_participants: Option<u32>,
    pub theme: Option<String>,
    pub rules: Option<String>,
    pub prizes: Option<Vec<PrizeDto>>,
    pub previous_winners: Option<Vec<WinnerDto>>,
    pub organizers: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct HackathonUpdateRequestDto {
    #[validate(length(min = 1, max = 100, message = "Hackathon name must be between 1 and 100 characters"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[validate(length(min = 1, max = 1000, message = "Description must be between 1 and 1000 characters"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = DateTime)]
    pub start_date: Option<DateTime<Utc>>,
    #[schema(value_type = String, format = DateTime)]
    pub end_date: Option<DateTime<Utc>>,
    #[schema(value_type = String, format = DateTime)]
    pub registration_deadline: Option<DateTime<Utc>>,
    #[validate(range(min = 1, max = 10000, message = "Max participants must be between 1 and 10000"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_participants: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prizes: Option<Vec<PrizeDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_winners: Option<Vec<WinnerDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizers: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct HackathonDto {
    pub id: String,
    pub name: String,
    pub description: String,
    #[schema(value_type = String, format = DateTime)]
    pub start_date: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub end_date: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub registration_deadline: DateTime<Utc>,
    pub max_participants: Option<u32>,
    pub status: HackathonStatus,
    pub theme: Option<String>,
    pub rules: Option<String>,
    pub prizes: Option<Vec<PrizeDto>>,
    pub previous_winners: Option<Vec<WinnerDto>>,
    pub organizers: Vec<String>,
    pub is_deleted: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct PrizeDto {
    #[validate(range(min = 1, message = "Position must be at least 1"))]
    pub position: u32,
    #[validate(length(min = 1, message = "Prize title cannot be empty"))]
    pub title: String,
    pub description: Option<String>,
    pub value: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct WinnerDto {
    #[validate(range(min = 1, message = "Position must be at least 1"))]
    pub position: u32,
    pub team_id: String,
    #[validate(length(min = 1, message = "Project name cannot be empty"))]
    pub project_name: String,
    pub team_name: Option<String>,
}

// Hackathon Events DTOs
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct HackathonEventCreateRequestDto {
    #[validate(length(min = 1, message = "Event title cannot be empty"))]
    pub title: String,
    pub description: Option<String>,
    pub event_type: HackathonEventType,
    #[schema(value_type = String, format = DateTime)]
    pub start_time: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub end_time: DateTime<Utc>,
    pub location: Option<String>,
    pub virtual_link: Option<String>,
    #[validate(range(min = 1, message = "Max attendees must be at least 1"))]
    pub max_attendees: Option<u32>,
    pub is_mandatory: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct HackathonEventUpdateRequestDto {
    #[validate(length(min = 1, message = "Event title cannot be empty"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<HackathonEventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = DateTime)]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = DateTime)]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_link: Option<String>,
    #[validate(range(min = 1, message = "Max attendees must be at least 1"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attendees: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_mandatory: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct HackathonEventDto {
    pub id: String,
    pub hackathon_id: String,
    pub title: String,
    pub description: Option<String>,
    pub event_type: HackathonEventType,
    #[schema(value_type = String, format = DateTime)]
    pub start_time: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub end_time: DateTime<Utc>,
    pub location: Option<String>,
    pub virtual_link: Option<String>,
    pub max_attendees: Option<u32>,
    pub is_mandatory: bool,
    pub is_deleted: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

// Hackathon Timeline DTOs
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct HackathonTimelineCreateRequestDto {
    pub phase: HackathonPhase,
    #[validate(length(min = 1, message = "Timeline title cannot be empty"))]
    pub title: String,
    pub description: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub start_date: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub end_date: DateTime<Utc>,
    pub is_active: bool,
    #[validate(range(min = 0, message = "Order must be non-negative"))]
    pub order: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct HackathonTimelineUpdateRequestDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<HackathonPhase>,
    #[validate(length(min = 1, message = "Timeline title cannot be empty"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = DateTime)]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = DateTime)]
    pub end_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[validate(range(min = 0, message = "Order must be non-negative"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct HackathonTimelineDto {
    pub id: String,
    pub hackathon_id: String,
    pub phase: HackathonPhase,
    pub title: String,
    pub description: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub start_date: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub end_date: DateTime<Utc>,
    pub is_active: bool,
    pub order: u32,
    pub is_deleted: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

// Hackathon Submissions DTOs
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct HackathonSubmissionCreateRequestDto {
    #[validate(length(min = 1, message = "Project name cannot be empty"))]
    pub project_name: String,
    #[validate(length(min = 1, message = "Description cannot be empty"))]
    pub description: String,
    pub repository_url: Option<String>,
    pub demo_url: Option<String>,
    pub slides_url: Option<String>,
    pub technologies: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct HackathonSubmissionUpdateRequestDto {
    #[validate(length(min = 1, message = "Project name cannot be empty"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[validate(length(min = 1, message = "Description cannot be empty"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slides_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technologies: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct HackathonSubmissionDto {
    pub id: String,
    pub hackathon_id: String,
    pub team_id: String,
    pub project_name: String,
    pub description: String,
    pub repository_url: Option<String>,
    pub demo_url: Option<String>,
    pub slides_url: Option<String>,
    pub technologies: Vec<String>,
    pub submission_status: SubmissionStatus,
    #[schema(value_type = String, format = DateTime)]
    pub submitted_at: DateTime<Utc>,
    pub is_deleted: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

// Query DTOs
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct HackathonQueryDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<HackathonStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct HackathonEventQueryDto {
    pub hackathon_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<HackathonEventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct HackathonTimelineQueryDto {
    pub hackathon_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<HackathonPhase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct HackathonSubmissionQueryDto {
    pub hackathon_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_status: Option<SubmissionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

// Conversion implementations
impl From<HackathonSchema> for HackathonDto {
    fn from(schema: HackathonSchema) -> Self {
        Self {
            id: schema.id.id.to_raw(),
            name: schema.name,
            description: schema.description,
            start_date: schema.start_date,
            end_date: schema.end_date,
            registration_deadline: schema.registration_deadline,
            max_participants: schema.max_participants,
            status: schema.status,
            theme: schema.theme,
            rules: schema.rules,
            prizes: schema.prizes.map(|prizes| {
                prizes
                    .into_iter()
                    .map(|p| PrizeDto {
                        position: p.position,
                        title: p.title,
                        description: p.description,
                        value: p.value,
                    })
                    .collect()
            }),
            previous_winners: schema.previous_winners.map(|winners| {
                winners
                    .into_iter()
                    .map(|w| WinnerDto {
                        position: w.position,
                        team_id: w.team_id,
                        project_name: w.project_name,
                        team_name: w.team_name,
                    })
                    .collect()
            }),
            organizers: schema.organizers,
            is_deleted: schema.is_deleted,
            created_at: schema.created_at,
            updated_at: schema.updated_at,
        }
    }
}

impl From<HackathonEventsSchema> for HackathonEventDto {
    fn from(schema: HackathonEventsSchema) -> Self {
        Self {
            id: schema.id.id.to_raw(),
            hackathon_id: schema.hackathon_id.id.to_raw(),
            title: schema.title,
            description: schema.description,
            event_type: schema.event_type,
            start_time: schema.start_time,
            end_time: schema.end_time,
            location: schema.location,
            virtual_link: schema.virtual_link,
            max_attendees: schema.max_attendees,
            is_mandatory: schema.is_mandatory,
            is_deleted: schema.is_deleted,
            created_at: schema.created_at,
            updated_at: schema.updated_at,
        }
    }
}

impl From<HackathonTimelineSchema> for HackathonTimelineDto {
    fn from(schema: HackathonTimelineSchema) -> Self {
        Self {
            id: schema.id.id.to_raw(),
            hackathon_id: schema.hackathon_id.id.to_raw(),
            phase: schema.phase,
            title: schema.title,
            description: schema.description,
            start_date: schema.start_date,
            end_date: schema.end_date,
            is_active: schema.is_active,
            order: schema.order,
            is_deleted: schema.is_deleted,
            created_at: schema.created_at,
            updated_at: schema.updated_at,
        }
    }
}

impl From<HackathonSubmissionsSchema> for HackathonSubmissionDto {
    fn from(schema: HackathonSubmissionsSchema) -> Self {
        Self {
            id: schema.id.id.to_raw(),
            hackathon_id: schema.hackathon_id.id.to_raw(),
            team_id: schema.team_id.id.to_raw(),
            project_name: schema.project_name,
            description: schema.description,
            repository_url: schema.repository_url,
            demo_url: schema.demo_url,
            slides_url: schema.slides_url,
            technologies: schema.technologies,
            submission_status: schema.submission_status,
            submitted_at: schema.submitted_at,
            is_deleted: schema.is_deleted,
            created_at: schema.created_at,
            updated_at: schema.updated_at,
        }
    }
}
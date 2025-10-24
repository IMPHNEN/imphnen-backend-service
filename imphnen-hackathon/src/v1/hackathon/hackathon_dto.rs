use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, schema};
use validator::{Validate, ValidationError};

// Custom validators
pub fn validate_url_format(url: &str) -> Result<(), ValidationError> {
	lazy_static! {
		static ref URL_REGEX: Regex = Regex::new(r"^https?://[^\s$.?#].[^\s]*$").unwrap();
	}
	if URL_REGEX.is_match(url) {
		Ok(())
	} else {
		Err(ValidationError::new("invalid_url"))
	}
}

pub fn validate_github_url(url: &str) -> Result<(), ValidationError> {
	lazy_static! {
		static ref GITHUB_REGEX: Regex = Regex::new(r"^https?://github\.com/[a-zA-Z0-9_-]+(/[a-zA-Z0-9_-]+)?$").unwrap();
	}
	if GITHUB_REGEX.is_match(url) {
		Ok(())
	} else {
		Err(ValidationError::new("invalid_github_url"))
	}
}

pub fn validate_demo_url(url: &str) -> Result<(), ValidationError> {
	lazy_static! {
		static ref DEMO_URL_REGEX: Regex = Regex::new(r"^https?://(?:www\.)?[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+(/[^\s]*)?$").unwrap();
	}
	if DEMO_URL_REGEX.is_match(url) {
		Ok(())
	} else {
		Err(ValidationError::new("invalid_demo_url"))
	}
}

use crate::v1::hackathon::hackathon_schema::{
    HackathonEventType, HackathonEventsSchema, HackathonPhase, HackathonSchema,
    HackathonStatus, HackathonSubmissionsSchema, HackathonTimelineSchema,
    SubmissionStatus,
    HackathonParticipantSchema,
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
    
    #[validate(length(max = 200, message = "Theme cannot exceed 200 characters"))]
    pub theme: Option<String>,
    
    #[validate(length(max = 2000, message = "Rules cannot exceed 2000 characters"))]
    pub rules: Option<String>,
    
    pub prizes: Option<Vec<PrizeDto>>,
    pub previous_winners: Option<Vec<WinnerDto>>,
    
    #[validate(length(min = 1, message = "Organizers list cannot be empty"))]
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
    // Accept either `title` or `name` in incoming JSON (tests may send `name`).
    // Make it optional so missing title doesn't cause a 422; service/repo will
    // fallback to an empty title or a sensible default.
    #[serde(alias = "name")]
    #[serde(default)]
    pub title: Option<String>,
    pub description: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub start_date: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub end_date: DateTime<Utc>,
    #[serde(default)]
    pub is_active: Option<bool>,
    #[serde(default)]
    #[validate(range(min = 0, message = "Order must be non-negative"))]
    pub order: Option<u32>,
}

// Custom validator for HackathonPhase (case-insensitive)
pub fn validate_hackathon_phase(phase: &str) -> Result<(), ValidationError> {
    let normalized = phase.to_lowercase();
    match normalized.as_str() {
        "registration" | "ideation" | "development" | "submission" | "judging" | "awards" => Ok(()),
        _ => Err(ValidationError::new("invalid_hackathon_phase")),
    }
}

// Custom validator to ensure start_date is in the future
pub fn validate_future_date(date: &DateTime<Utc>) -> Result<(), ValidationError> {
    let now = Utc::now();
    if date <= &now {
        Err(ValidationError::new("start_date_must_be_in_future"))
    } else {
        Ok(())
    }
}

// Custom validator to ensure end_date is in the future or current
pub fn validate_future_or_current_date(date: &DateTime<Utc>) -> Result<(), ValidationError> {
    let now = Utc::now();
    if date < &now {
        Err(ValidationError::new("end_date_must_be_in_future_or_current"))
    } else {
        Ok(())
    }
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
    pub upload_file_url: Option<String>, // URL to uploaded zip/pdf file
    pub demo_url: Option<String>,
    pub slides_url: Option<String>,
    pub technologies: Vec<String>,
    // Social media contacts for demo (at least one required)
    pub contact_instagram: Option<String>,
    pub contact_twitter: Option<String>,
    pub contact_linkedin: Option<String>,
    pub contact_facebook: Option<String>,
    pub contact_youtube: Option<String>,
    pub contact_tiktok: Option<String>,
    pub contact_other: Option<String>,
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
    pub upload_file_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slides_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technologies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_instagram: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_linkedin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_facebook: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_youtube: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_tiktok: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_other: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct HackathonSubmissionDto {
    pub id: String,
    pub hackathon_id: String,
    pub team_id: String,
    pub project_name: String,
    pub description: String,
    pub repository_url: Option<String>,
    pub upload_file_url: Option<String>,
    pub demo_url: Option<String>,
    pub slides_url: Option<String>,
    pub technologies: Vec<String>,
    pub contact_instagram: Option<String>,
    pub contact_twitter: Option<String>,
    pub contact_linkedin: Option<String>,
    pub contact_facebook: Option<String>,
    pub contact_youtube: Option<String>,
    pub contact_tiktok: Option<String>,
    pub contact_other: Option<String>,
    #[serde(rename = "status")]
    pub submission_status: SubmissionStatus,
    pub judge_feedback: Option<String>,
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
            team_id: schema.team_id.map(|t| t.id.to_raw()).unwrap_or_default(),
            project_name: schema.project_name.unwrap_or_default(),
            description: schema.description.unwrap_or_default(),
            repository_url: schema.repository_url,
            upload_file_url: schema.upload_file_url,
            demo_url: schema.demo_url,
            slides_url: schema.slides_url,
            technologies: schema.technologies.unwrap_or_default(),
            contact_instagram: schema.contact_instagram,
            contact_twitter: schema.contact_twitter,
            contact_linkedin: schema.contact_linkedin,
            contact_facebook: schema.contact_facebook,
            contact_youtube: schema.contact_youtube,
            contact_tiktok: schema.contact_tiktok,
            contact_other: schema.contact_other,
            submission_status: schema.submission_status.unwrap_or(super::hackathon_schema::SubmissionStatus::Draft),
            judge_feedback: schema.judge_feedback,
            submitted_at: schema.submitted_at.unwrap_or(chrono::Utc::now()),
            is_deleted: schema.is_deleted,
            created_at: schema.created_at,
            updated_at: schema.updated_at,
        }
    }
}

// Hackathon Participant DTOs
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct RegisterParticipantRequestDto {
    #[validate(length(min = 1, message = "user_id cannot be empty"))]
    pub user_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct HackathonParticipantDto {
    pub id: String,
    pub hackathon_id: String,
    pub user_id: String,
    pub is_deleted: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<HackathonParticipantSchema> for HackathonParticipantDto {
    fn from(schema: HackathonParticipantSchema) -> Self {
        Self {
            id: schema.id.id.to_raw(),
            hackathon_id: schema.hackathon_id.id.to_raw(),
            user_id: schema.user_id,
            is_deleted: schema.is_deleted,
            created_at: schema.created_at,
            updated_at: schema.updated_at,
        }
    }
}

// Admin Sensitive Data Management DTOs
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct AdminManageSensitiveDataRequestDto {
    #[validate(length(min = 1, message = "At least one user ID is required"))]
    pub user_ids: Vec<String>,
    #[validate(length(min = 1, message = "At least one raw score is required"))]
    pub raw_scores: Vec<i32>,
    pub personal_info: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminSensitiveDataMemberDto {
    pub user_id: String,
    pub masked_email: String,
    pub masked_phone: String,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminSensitiveDataDto {
    pub submission_id: String,
    pub team_id: String,
    pub project_name: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub score: Option<i32>,
    pub members: Vec<AdminSensitiveDataMemberDto>,
    pub raw_scores: Option<Vec<i32>>,
    pub submission_date: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminSensitiveDataResponseDto {
    pub data: Vec<AdminSensitiveDataDto>,
    pub message: String,
}
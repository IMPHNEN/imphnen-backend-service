use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use super::{ParticipantRole, RegistrationStatus};

// ============================================
// Registration Request/Response DTOs
// ============================================

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct RegistrationRequestDto {
    pub team_id: Option<String>,
    pub role: Option<ParticipantRole>,
    
    #[validate(length(max = 1000, message = "Motivation must not exceed 1000 characters"))]
    pub motivation: Option<String>,
    
    pub skills: Option<Vec<String>>,
    
    #[validate(custom(function = "validate_experience_level"))]
    pub experience_level: Option<String>,
    
    #[validate(length(max = 100))]
    pub github_username: Option<String>,
    
    #[validate(url(message = "Invalid portfolio URL"))]
    pub portfolio_url: Option<String>,
    
    pub dietary_requirements: Option<String>,
    
    #[validate(custom(function = "validate_tshirt_size"))]
    pub tshirt_size: Option<String>,
    
    #[validate(length(max = 100))]
    pub emergency_contact_name: Option<String>,
    
    #[validate(length(max = 20))]
    pub emergency_contact_phone: Option<String>,
}

fn validate_experience_level(level: &str) -> Result<(), validator::ValidationError> {
    let valid_levels = ["beginner", "intermediate", "advanced"];
    if valid_levels.contains(&level) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("Invalid experience level"))
    }
}

fn validate_tshirt_size(size: &str) -> Result<(), validator::ValidationError> {
    let valid_sizes = ["XS", "S", "M", "L", "XL", "XXL"];
    if valid_sizes.contains(&size) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("Invalid t-shirt size"))
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegistrationResponseDto {
    pub id: String,
    pub hackathon_id: String,
    pub user_id: String,
    pub team_id: Option<String>,
    pub status: RegistrationStatus,
    pub role: ParticipantRole,
    pub registration_date: String,
    pub checked_in: bool,
    pub message: String,
}

// ============================================
// List Registrations DTOs
// ============================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegistrationListItemDto {
    pub id: String,
    pub hackathon_id: String,
    pub hackathon_name: Option<String>,
    pub user_id: String,
    pub user_fullname: Option<String>,
    pub user_email: Option<String>,
    pub team_id: Option<String>,
    pub team_name: Option<String>,
    pub status: RegistrationStatus,
    pub role: ParticipantRole,
    pub registration_date: String,
    pub checked_in: bool,
    pub check_in_time: Option<String>,
    pub experience_level: Option<String>,
    pub skills: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegistrationListResponseDto {
    pub registrations: Vec<RegistrationListItemDto>,
    pub total: usize,
    pub status_filter: Option<String>,
}

// Internal query DTO (fields already as String from DB)
#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationListQueryDto {
    pub id: String,
    pub hackathon_id: String,
    pub hackathon_name: Option<String>,
    pub user_id: String,
    pub user_fullname: Option<String>,
    pub user_email: Option<String>,
    pub team_id: Option<String>,
    pub team_name: Option<String>,
    pub status: RegistrationStatus,
    pub role: ParticipantRole,
    pub registration_date: String,
    pub checked_in: bool,
    pub check_in_time: Option<String>,
    pub experience_level: Option<String>,
    pub skills: Option<Vec<String>>,
}

// ============================================
// Update Status DTOs
// ============================================

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateRegistrationStatusRequestDto {
    pub status: RegistrationStatus,
    
    #[validate(length(max = 500, message = "Reason must not exceed 500 characters"))]
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateRegistrationStatusResponseDto {
    pub id: String,
    pub status: RegistrationStatus,
    pub updated_at: String,
    pub message: String,
}

// ============================================
// Check-in DTOs
// ============================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CheckInResponseDto {
    pub id: String,
    pub user_fullname: Option<String>,
    pub checked_in: bool,
    pub check_in_time: String,
    pub message: String,
}

// ============================================
// Statistics DTOs
// ============================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegistrationStatsDto {
    pub hackathon_id: String,
    pub hackathon_name: Option<String>,
    pub total_registrations: usize,
    pub pending: usize,
    pub approved: usize,
    pub rejected: usize,
    pub waitlisted: usize,
    pub cancelled: usize,
    pub checked_in: usize,
    pub team_registrations: usize,
    pub individual_registrations: usize,
}

// ============================================
// User's Hackathons DTOs
// ============================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserHackathonDto {
    pub registration_id: String,
    pub hackathon_id: String,
    pub hackathon_name: String,
    pub hackathon_description: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub status: RegistrationStatus,
    pub role: ParticipantRole,
    pub registration_date: String,
    pub checked_in: bool,
    pub team_id: Option<String>,
    pub team_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserHackathonsResponseDto {
    pub hackathons: Vec<UserHackathonDto>,
    pub total: usize,
}

// Internal query DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct UserHackathonQueryDto {
    pub registration_id: String,
    pub hackathon_id: String,
    pub hackathon_name: String,
    pub hackathon_description: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub status: RegistrationStatus,
    pub role: ParticipantRole,
    pub registration_date: String,
    pub checked_in: bool,
    pub team_id: Option<String>,
    pub team_name: Option<String>,
}

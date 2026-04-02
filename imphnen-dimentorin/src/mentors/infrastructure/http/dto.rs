use imphnen_libs::ZodValidate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use zod_rs::prelude::*;

// ============================================================
// Response DTOs
// ============================================================

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MentorListResponseDto {
    pub id: String,
    pub user_id: String,
    pub fullname: Option<String>,
    pub email: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MentorDetailResponseDto {
    pub id: String,
    pub user_id: String,
    pub fullname: Option<String>,
    pub email: Option<String>,
    pub legal_name: Option<String>,
    pub gender: Option<String>,
    pub domicile: Option<String>,
    pub phone_for_verification: Option<String>,
    pub bio: Option<String>,
    pub last_education: Option<String>,
    pub linkedin_url: Option<String>,
    pub github_url: Option<String>,
    pub cv_url: Option<String>,
    pub portfolio_url: Option<String>,
    pub industries: Vec<String>,
    pub expertise: Vec<String>,
    pub languages: Vec<String>,
    pub current_company: String,
    pub current_role: String,
    pub years_of_experience: i32,
    pub topics_of_interest: Vec<String>,
    pub preferred_mentee_level: Vec<String>,
    pub preferred_mentoring_formats: Vec<String>,
    pub availability_commitment: String,
    pub mentoring_rate: f64,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MentorRegisterResponseDto {
    pub id: String,
    pub user_id: String,
    pub email: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

// ============================================================
// Request DTOs
// ============================================================

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct MentorUserRegisterRequestDto {
    #[zod(email, min_length(1))]
    pub email: String,
    #[zod(min_length(8), regex(pattern = "^[A-Za-z\\d@$!%*?&]{8,}$"))]
    pub password: String,
    #[zod(min_length(2))]
    pub fullname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    pub identity_and_verification: IdentityAndVerification,
    pub professional_profile: ProfessionalProfile,
    pub mentoring_logistics: MentoringLogistics,
}

impl ZodValidate for MentorUserRegisterRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct IdentityAndVerification {
    #[zod(min_length(3))]
    pub legal_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domicile: Option<String>,
    #[zod(url)]
    pub identity_document_url: String,
    #[zod(min_length(10), max_length(15))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_for_verification: Option<String>,
}

impl ZodValidate for IdentityAndVerification {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct ProfessionalProfile {
    #[zod(min_length(50))]
    pub bio: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_education: Option<String>,
    #[zod(url)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin_url: Option<String>,
    #[zod(url)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_url: Option<String>,
    #[zod(url)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cv_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_url: Option<String>,
    pub industries: Vec<String>,
    pub expertise: Vec<String>,
    pub languages: Vec<String>,
    #[zod(min_length(1))]
    pub current_company: String,
    #[zod(min_length(1))]
    pub current_role: String,
    #[zod(min(2.0), int)]
    pub years_of_experience: i32,
}

impl ZodValidate for ProfessionalProfile {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct MentoringLogistics {
    pub topics_of_interest: Vec<String>,
    pub preferred_mentee_level: Vec<String>,
    pub preferred_mentoring_formats: Vec<String>,
    #[zod(min_length(5))]
    pub availability_commitment: String,
    #[zod(min(1.0))]
    pub mentoring_rate_amount: u64,
}

impl ZodValidate for MentoringLogistics {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct MentorUpdateRequestDto {
    #[zod(min_length(3))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domicile: Option<String>,
    #[zod(min_length(10), max_length(15))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_for_verification: Option<String>,
    #[zod(min_length(50))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_education: Option<String>,
    #[zod(url)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin_url: Option<String>,
    #[zod(url)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_url: Option<String>,
    #[zod(url)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cv_url: Option<String>,
    #[zod(url)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industries: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expertise: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_role: Option<String>,
    #[zod(min(2.0), int)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years_of_experience: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics_of_interest: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_mentee_level: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_mentoring_formats: Option<Vec<String>>,
    #[zod(min_length(5))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_commitment: Option<String>,
    #[zod(min(1.0))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentoring_rate_amount: Option<u64>,
}

impl ZodValidate for MentorUpdateRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct MentorVerifyRequestDto {
    #[zod(min_length(1))]
    pub status: String,
}

impl ZodValidate for MentorVerifyRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema, Default)]
pub struct MentoringRate {
    #[zod(min(1.0))]
    pub amount: u64,
    #[zod(min_length(1))]
    pub currency: String,
    #[zod(min_length(1))]
    pub per_duration: String,
}

impl ZodValidate for MentoringRate {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct MentorRegisterFromTokenRequestDto {
    pub identity_and_verification: IdentityAndVerification,
    pub professional_profile: ProfessionalProfile,
    pub mentoring_logistics: MentoringLogistics,
}

impl ZodValidate for MentorRegisterFromTokenRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

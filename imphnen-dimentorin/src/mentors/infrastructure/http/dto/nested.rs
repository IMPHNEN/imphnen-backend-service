use imphnen_libs::ZodValidate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use zod_rs::prelude::*;

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

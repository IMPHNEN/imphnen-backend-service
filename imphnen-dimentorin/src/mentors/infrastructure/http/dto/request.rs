use super::nested::{
	IdentityAndVerification, MentoringLogistics, ProfessionalProfile,
};
use crate::mentors::domain::{
	MentorRegisterCommand, MentorUpdateCommand, MentorVerifyCommand,
};
use imphnen_libs::ZodValidate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use zod_rs::prelude::*;

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

impl From<MentorUserRegisterRequestDto> for MentorRegisterCommand {
	fn from(dto: MentorUserRegisterRequestDto) -> Self {
		Self {
			email: dto.email,
			password: dto.password,
			fullname: dto.fullname,
			phone_number: dto.phone_number,
			legal_name: dto.identity_and_verification.legal_name,
			gender: dto.identity_and_verification.gender,
			domicile: dto.identity_and_verification.domicile,
			identity_document_url: dto.identity_and_verification.identity_document_url,
			phone_for_verification: dto.identity_and_verification.phone_for_verification,
			bio: dto.professional_profile.bio,
			last_education: dto.professional_profile.last_education,
			linkedin_url: dto.professional_profile.linkedin_url,
			github_url: dto.professional_profile.github_url,
			cv_url: dto.professional_profile.cv_url,
			portfolio_url: dto.professional_profile.portfolio_url,
			industries: dto.professional_profile.industries,
			expertise: dto.professional_profile.expertise,
			languages: dto.professional_profile.languages,
			current_company: dto.professional_profile.current_company,
			current_role: dto.professional_profile.current_role,
			years_of_experience: dto.professional_profile.years_of_experience,
			topics_of_interest: dto.mentoring_logistics.topics_of_interest,
			preferred_mentee_level: dto.mentoring_logistics.preferred_mentee_level,
			preferred_mentoring_formats: dto
				.mentoring_logistics
				.preferred_mentoring_formats,
			availability_commitment: dto.mentoring_logistics.availability_commitment,
			mentoring_rate_amount: dto.mentoring_logistics.mentoring_rate_amount,
		}
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

impl From<MentorUpdateRequestDto> for MentorUpdateCommand {
	fn from(dto: MentorUpdateRequestDto) -> Self {
		Self {
			legal_name: dto.legal_name,
			gender: dto.gender,
			domicile: dto.domicile,
			phone_for_verification: dto.phone_for_verification,
			bio: dto.bio,
			last_education: dto.last_education,
			linkedin_url: dto.linkedin_url,
			github_url: dto.github_url,
			cv_url: dto.cv_url,
			portfolio_url: dto.portfolio_url,
			industries: dto.industries,
			expertise: dto.expertise,
			languages: dto.languages,
			current_company: dto.current_company,
			current_role: dto.current_role,
			years_of_experience: dto.years_of_experience,
			topics_of_interest: dto.topics_of_interest,
			preferred_mentee_level: dto.preferred_mentee_level,
			preferred_mentoring_formats: dto.preferred_mentoring_formats,
			availability_commitment: dto.availability_commitment,
			mentoring_rate_amount: dto.mentoring_rate_amount,
		}
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

impl From<MentorVerifyRequestDto> for MentorVerifyCommand {
	fn from(dto: MentorVerifyRequestDto) -> Self {
		Self { status: dto.status }
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

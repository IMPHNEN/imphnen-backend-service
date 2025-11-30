use crate::v1::mentors::MentorSchema;
use crate::v1::sessions::sessions_schema::Thing;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MentorListResponseDto {
	pub id: String,
	pub fullname: Option<String>,
	pub email: Option<String>,
	pub status: String,
	pub created_at: String,
	pub updated_at: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MentorDetailWithUserDto {
	pub id: String,
	pub user_id: String,
	// Personal data is now in UsersSchema, access via user_id
	// Removed: fullname, email, legal_name, identity_document_url, 
	// phone_for_verification, bio, linkedin_url, github_url, cv_url
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
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MentorDetailResponseDto {
	pub id: String,
	pub user_id: String,
	// Personal data fields from UsersSchema
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
	// Professional data from MentorSchema
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

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct MentorRegisterResponseDto {
	pub id: String,
	pub user_id: String,
	pub email: Option<String>,
	pub status: String,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct MentorUpdateRequestDto {
	#[validate(length(
		min = 3,
		message = "Legal name must be at least 3 characters"
	))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub legal_name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gender: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub domicile: Option<String>,
	#[validate(length(
		min = 10,
		max = 15,
		message = "Phone must be 10-15 characters"
	))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone_for_verification: Option<String>,
	#[validate(length(min = 50, message = "Bio must be at least 50 characters"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bio: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_education: Option<String>,
	#[validate(url(message = "Invalid LinkedIn URL"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub linkedin_url: Option<String>,
	#[validate(url(message = "Invalid GitHub URL"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub github_url: Option<String>,
	#[validate(url(message = "Invalid CV URL"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cv_url: Option<String>,
	#[validate(url(message = "Invalid portfolio URL"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub portfolio_url: Option<String>,
	#[validate(length(min = 1, message = "At least 1 industry required"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub industries: Option<Vec<String>>,
	#[validate(length(min = 1, message = "At least 1 expertise required"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expertise: Option<Vec<String>>,
	#[validate(length(min = 1, message = "At least 1 language required"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub languages: Option<Vec<String>>,
	#[validate(length(min = 1, message = "Current company required"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub current_company: Option<String>,
	#[validate(length(min = 1, message = "Current role required"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub current_role: Option<String>,
	#[validate(range(min = 2, message = "At least 2 years of experience required"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub years_of_experience: Option<i32>,
	#[validate(length(min = 1, message = "At least 1 topic required"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub topics_of_interest: Option<Vec<String>>,
	#[validate(length(min = 1, message = "At least 1 mentee level required"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred_mentee_level: Option<Vec<String>>,
	#[validate(length(min = 1, message = "At least 1 mentoring format required"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred_mentoring_formats: Option<Vec<String>>,
	#[validate(length(
		min = 5,
		message = "Availability commitment must be at least 5 characters"
	))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub availability_commitment: Option<String>,
	#[validate(range(min = 1, message = "Amount must be at least 1"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mentoring_rate_amount: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct MentorUserRegisterRequestDto {
	#[validate(
		length(min = 1, message = "Email cannot be empty"),
		email(message = "Email not valid")
	)]
	pub email: String,
	#[validate(length(
		min = 8,
		message = "Password must have at least 8 characters"
	))]
	#[validate(custom(
		function = "imphnen_iam::v1::auth::auth_dto::validate_password_complexity",
		message = "Password must include uppercase, lowercase, number, and special character"
	))]
	pub password: String,
	#[validate(length(min = 2, message = "Fullname at least have 2 character"))]
	pub fullname: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone_number: Option<String>,
	#[validate(nested)]
	pub identity_and_verification: IdentityAndVerification,
	#[validate(nested)]
	pub professional_profile: ProfessionalProfile,
	#[validate(nested)]
	pub mentoring_logistics: MentoringLogistics,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct MentorRegisterFromTokenRequestDto {
	#[validate(nested)]
	pub identity_and_verification: IdentityAndVerification,
	#[validate(nested)]
	pub professional_profile: ProfessionalProfile,
	#[validate(nested)]
	pub mentoring_logistics: MentoringLogistics,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct IdentityAndVerification {
	#[validate(length(
		min = 3,
		message = "Legal name must be at least 3 characters"
	))]
	pub legal_name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gender: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub domicile: Option<String>,
	#[validate(url(message = "Invalid identity document URL"))]
	pub identity_document_url: String,
	#[validate(length(
		min = 10,
		max = 15,
		message = "Phone must be 10-15 characters"
	))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone_for_verification: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct ProfessionalProfile {
	#[validate(length(min = 50, message = "Bio must be at least 50 characters"))]
	pub bio: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_education: Option<String>,
	#[validate(url(message = "Invalid LinkedIn URL"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub linkedin_url: Option<String>,
	#[validate(url(message = "Invalid GitHub URL"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub github_url: Option<String>,
	#[validate(url(message = "Invalid CV URL"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cv_url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub portfolio_url: Option<String>,
	#[validate(length(min = 1, message = "At least 1 industry required"))]
	pub industries: Vec<String>,
	#[validate(length(min = 1, message = "At least 1 expertise required"))]
	pub expertise: Vec<String>,
	#[validate(length(min = 1, message = "At least 1 language required"))]
	pub languages: Vec<String>,
	#[validate(length(min = 1, message = "Current company required"))]
	pub current_company: String,
	#[validate(length(min = 1, message = "Current role required"))]
	pub current_role: String,
	#[validate(range(min = 2, message = "At least 2 years of experience required"))]
	pub years_of_experience: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct MentoringLogistics {
	#[validate(length(min = 1, message = "At least 1 topic required"))]
	pub topics_of_interest: Vec<String>,
	#[validate(length(min = 1, message = "At least 1 mentee level required"))]
	pub preferred_mentee_level: Vec<String>,
	#[validate(length(min = 1, message = "At least 1 mentoring format required"))]
	pub preferred_mentoring_formats: Vec<String>,
	#[validate(length(
		min = 5,
		message = "Availability commitment must be at least 5 characters"
	))]
	pub availability_commitment: String,
	#[validate(range(min = 1, message = "Amount must be at least 1"))]
	pub mentoring_rate_amount: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate, Default)]
pub struct MentoringRate {
	#[validate(range(min = 1, message = "Amount must be at least 1"))]
	pub amount: u64,
	#[validate(length(min = 1, message = "Currency is required"))]
	pub currency: String,
	#[validate(length(min = 1, message = "Per duration is required"))]
	pub per_duration: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MentorInsertDto {
	pub id: Thing,
	pub user_id: Option<Thing>,
	// Personal data removed - now stored in UsersSchema
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
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

impl From<MentorSchema> for MentorInsertDto {
	fn from(schema: MentorSchema) -> Self {
		MentorInsertDto {
			id: schema.id,
			user_id: schema.user_id,
			// Personal data removed from MentorSchema
			industries: schema.industries,
			expertise: schema.expertise,
			languages: schema.languages,
			current_company: schema.current_company,
			current_role: schema.current_role,
			years_of_experience: schema.years_of_experience,
			topics_of_interest: schema.topics_of_interest,
			preferred_mentee_level: schema.preferred_mentee_level,
			preferred_mentoring_formats: schema.preferred_mentoring_formats,
			availability_commitment: schema.availability_commitment,
			mentoring_rate: schema.mentoring_rate,
			status: schema.status,
			is_deleted: schema.is_deleted,
			created_at: schema.created_at,
			updated_at: schema.updated_at,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct MentorVerifyRequestDto {
	#[validate(length(min = 1, message = "Status is required"))]
	pub status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MentorDetailQueryDto {
	pub id: Thing,
	pub user_id: Thing,
	// Personal data has been moved to UsersSchema
	// Use user_id to get: fullname, email, legal_name, gender, domicile, 
	// identity_document_url, phone_for_verification, bio, last_education, 
	// linkedin_url, github_url, cv_url, portfolio_url
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
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

impl From<MentorDetailQueryDto> for MentorListResponseDto {
	fn from(dto: MentorDetailQueryDto) -> Self {
		Self {
			id: dto.id.clone(),
			fullname: None, // now in user table, must be populated from service layer
			email: None, // now in user table, must be populated from service layer
			status: dto.status,
			created_at: dto.created_at,
			updated_at: dto.updated_at,
		}
	}
}

impl From<MentorDetailQueryDto> for MentorDetailResponseDto {
	fn from(dto: MentorDetailQueryDto) -> Self {
		Self {
			id: dto.id.clone(),
			user_id: dto.user_id.clone(),
			// Personal data fields are populated in service layer from UsersSchema
			fullname: None, // populated from user table in service layer
			email: None, // populated from user table in service layer
			legal_name: None, // populated from user table in service layer
			gender: None, // populated from user table in service layer
			domicile: None, // populated from user table in service layer
			phone_for_verification: None, // populated from user table in service layer
			bio: None, // populated from user table in service layer
			last_education: None, // populated from user table in service layer
			linkedin_url: None, // populated from user table in service layer
			github_url: None, // populated from user table in service layer
			cv_url: None, // populated from user table in service layer
			portfolio_url: None, // populated from user table in service layer
			// Professional data from mentor
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
			mentoring_rate: dto.mentoring_rate,
			status: dto.status,
			created_at: dto.created_at,
			updated_at: dto.updated_at,
		}
	}
}
impl From<MentorSchema> for MentorRegisterResponseDto {
	fn from(schema: MentorSchema) -> Self {
		Self {
			id: schema.id.to_string(),
			user_id: schema.user_id.unwrap_or_default(),
			email: None, // schema.email - now in user table
			status: schema.status,
			created_at: schema.created_at,
			updated_at: schema.updated_at,
		}
	}
}

impl From<MentorDetailWithUserDto> for MentorDetailQueryDto {
	fn from(dto: MentorDetailWithUserDto) -> Self {
		MentorDetailQueryDto {
			id: dto.id,
			user_id: dto.user_id,
			// Personal data removed from MentorDetailWithUserDto - now in UsersSchema
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
			mentoring_rate: dto.mentoring_rate,
			status: dto.status,
			is_deleted: dto.is_deleted,
			created_at: dto.created_at,
			updated_at: dto.updated_at,
		}
	}
}

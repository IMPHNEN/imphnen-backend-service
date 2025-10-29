use imphnen_entities::{ExperienceDto, EducationDto, UsersDetailQueryDto, RolesDetailQueryDto, RolesDetailItemDto};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;
use validator::Validate;
use crate::UsersSchema; // Import UsersSchema

lazy_static! {
	static ref PASSWORD_REGEX: regex::Regex =
		regex::Regex::new(r"^[A-Za-z\d@$!%*?&]{8,}$").unwrap();
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersActiveInactiveRequestDto {
	pub is_active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersSetNewPasswordRequestDto {
	pub password: String,
	pub old_password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UsersCreateRequestDto {
	#[validate(
		length(min = 1, message = "Email cannot be empty"),
		email(message = "Email not valid")
	)]
	pub email: String,

	#[validate(length(
		min = 8,
		message = "Password must have at least 8 characters"
	))]
	#[validate(regex(
		path = "*PASSWORD_REGEX",
		message = "Password must include uppercase, lowercase, number, and special character"
	))]
	pub password: String,

	#[validate(length(min = 2, message = "Fullname at least have 2 character"))]
	pub fullname: String,

	#[validate(length(
		min = 10,
		message = "Phone number at least have 10 character"
	))]
	pub phone_number: String,
	pub is_active: bool,
	pub role_id: String,
	pub avatar: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UsersUpdateRequestDto {
	#[validate(
		length(min = 1, message = "Email cannot be empty"),
		email(message = "Email not valid")
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub email: Option<String>,
	#[validate(length(
		min = 8,
		message = "Password must have at least 8 characters"
	))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub password: Option<String>,
	#[validate(length(min = 2, message = "Fullname at least have 2 character"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub fullname: Option<String>,
	#[validate(length(min = 2, message = "Legal name at least have 2 character"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub legal_name: Option<String>,
	#[validate(length(
		min = 10,
		message = "Phone number at least have 10 character"
	))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone_number: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone_for_verification: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_active: Option<bool>,
	#[validate(length(min = 1, message = "Gender is required"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gender: Option<String>,
	#[validate(length(min = 1, message = "Birthdate is required"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub birthdate: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub domicile: Option<String>,
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
	#[validate(url(message = "Invalid website URL"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub website_url: Option<String>,
	#[validate(url(message = "Invalid Twitter URL"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub twitter_url: Option<String>,
	#[validate(length(min = 1, message = "Avatar is required"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub location: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub skills: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub experience: Option<Vec<ExperienceDto>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub education: Option<Vec<EducationDto>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub career_status: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Default)]
pub struct UsersDetailItemDto {
	pub id: String,
	pub role: RolesDetailItemDto,
	pub fullname: String,
	pub legal_name: Option<String>,
	pub email: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub phone_for_verification: Option<String>,
	pub is_active: bool,
	pub gender: Option<String>,
	pub birthdate: Option<String>,
	pub domicile: Option<String>,
	pub bio: Option<String>,
	pub last_education: Option<String>,
	pub linkedin_url: Option<String>,
	pub github_url: Option<String>,
	pub cv_url: Option<String>,
	pub portfolio_url: Option<String>,
	pub website_url: Option<String>,
	pub twitter_url: Option<String>,
	pub location: Option<String>,
	pub skills: Option<Vec<String>>,
	pub experience: Option<Vec<ExperienceDto>>,
	pub education: Option<Vec<EducationDto>>,
	pub career_status: Option<String>,
	pub created_at: String,
	pub updated_at: String,
}

impl UsersDetailItemDto {
	pub fn from(dto: &UsersDetailQueryDto) -> Self {
		Self {
			id: dto.id.id.to_raw(),
			role: RolesDetailItemDto::from(&dto.role),
			fullname: dto.fullname.clone(),
			legal_name: dto.legal_name.clone(),
			email: dto.email.clone(),
			avatar: dto.avatar.clone(),
			phone_number: dto.phone_number.clone(),
			phone_for_verification: dto.phone_for_verification.clone(),
			is_active: dto.is_active,
			gender: dto.gender.clone(),
			birthdate: dto.birthdate.clone(),
			domicile: dto.domicile.clone(),
			bio: dto.bio.clone(),
			last_education: dto.last_education.clone(),
			linkedin_url: dto.linkedin_url.clone(),
			github_url: dto.github_url.clone(),
			cv_url: dto.cv_url.clone(),
			portfolio_url: dto.portfolio_url.clone(),
			website_url: dto.website_url.clone(),
			twitter_url: dto.twitter_url.clone(),
			location: dto.location.clone(),
			skills: dto.skills.clone(),
			experience: dto.experience.clone(),
			education: dto.education.clone(),
			career_status: dto.career_status.clone(),
			created_at: dto.created_at.clone(),
			updated_at: dto.updated_at.clone(),
		}
	}

    pub fn from_schema(schema: &UsersSchema) -> Self {
        Self {
            id: schema.id.id.to_raw(),
            role: RolesDetailItemDto::default(), // Placeholder, role needs to be fetched
            fullname: schema.fullname.clone(),
            legal_name: schema.legal_name.clone(),
            email: schema.email.clone(),
            avatar: schema.avatar.clone(),
            phone_number: schema.phone_number.clone(),
            phone_for_verification: schema.phone_for_verification.clone(),
            is_active: schema.is_active,
            gender: schema.gender.clone(),
            birthdate: schema.birthdate.clone(),
            domicile: schema.domicile.clone(),
            bio: schema.bio.clone(),
            last_education: schema.last_education.clone(),
            linkedin_url: schema.linkedin_url.clone(),
            github_url: schema.github_url.clone(),
            cv_url: schema.cv_url.clone(),
            portfolio_url: schema.portfolio_url.clone(),
            website_url: schema.website_url.clone(),
            twitter_url: schema.twitter_url.clone(),
            location: schema.location.clone(),
            skills: schema.skills.clone(),
            experience: schema.experience.clone(),
            education: schema.education.clone(),
            career_status: schema.career_status.clone(),
            created_at: schema.created_at.clone(),
            updated_at: schema.updated_at.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersListItemDto {
	pub id: String,
	pub role: String,
	pub fullname: String,
	pub email: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub is_active: bool,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersListQueryDto {
	pub id: Thing,
	pub role: RolesDetailQueryDto,
	pub fullname: String,
	pub email: String, // Corrected from pub pub email: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub is_active: bool,
	pub created_at: String,
	pub updated_at: String,
}

impl UsersListQueryDto {
	pub fn from(self) -> UsersListItemDto {
		UsersListItemDto {
			id: self.id.id.to_raw(),
			role: self.role.name,
			fullname: self.fullname,
			email: self.email,
			avatar: self.avatar,
			phone_number: self.phone_number,
			is_active: self.is_active,
			created_at: self.created_at,
			updated_at: self.updated_at,
		}
	}
}


impl From<&UsersDetailItemDto> for UsersDetailQueryDto {
	fn from(dto: &UsersDetailItemDto) -> Self {
		Self {
			id: crate::make_thing(&imphnen_libs::ResourceEnum::Users.to_string(), &dto.id),
			fullname: dto.fullname.clone(),
			legal_name: dto.legal_name.clone(),
			email: dto.email.clone(),
			avatar: dto.avatar.clone(),
			phone_number: dto.phone_number.clone(),
			phone_for_verification: dto.phone_for_verification.clone(),
			is_active: dto.is_active,
			is_deleted: false,
			gender: dto.gender.clone(),
			birthdate: dto.birthdate.clone(),
			domicile: dto.domicile.clone(),
			bio: dto.bio.clone(),
			last_education: dto.last_education.clone(),
			linkedin_url: dto.linkedin_url.clone(),
			github_url: dto.github_url.clone(),
			cv_url: dto.cv_url.clone(),
			portfolio_url: dto.portfolio_url.clone(),
			website_url: dto.website_url.clone(),
			twitter_url: dto.twitter_url.clone(),
			location: dto.location.clone(),
			skills: dto.skills.clone(),
			experience: dto.experience.clone(),
			education: dto.education.clone(),
			career_status: dto.career_status.clone(),
			password: String::new(),
			role: RolesDetailQueryDto::default(),
			created_at: dto.created_at.clone(),
			updated_at: dto.updated_at.clone(),
			mentor_id: None,
		}
		}
}

impl UsersDetailItemDto {
	   pub fn extract_permissions_from_user_role(&self) -> Vec<String> {
	       self.role.permissions.iter().map(|p| p.name.clone()).collect()
	   }
}
use crate::permissions::{PermissionsItemDto, PermissionsQueryDto};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ExperienceDto {
	pub id: String,
	pub company: String,
	pub position: String,
	pub duration: String,
	pub period: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct EducationDto {
	pub id: String,
	pub institution: String,
	pub degree: String,
	pub field: String,
	pub period: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Default)]
pub struct UserProfileExtensionDto {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone_number: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone_for_verification: Option<String>,
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
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RolesDetailQueryDto {
	pub id: String,
	pub name: String,
	pub permissions: Option<Vec<Option<PermissionsQueryDto>>>,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Default)]
pub struct RolesDetailItemDto {
	pub id: String,
	pub name: String,
	pub is_deleted: bool,
	pub permissions: Vec<PermissionsItemDto>,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl RolesDetailItemDto {
	pub fn from(dto: &RolesDetailQueryDto) -> Self {
		Self {
			id: dto.id.clone(),
			name: dto.name.clone(),
			is_deleted: dto.is_deleted,
			permissions: dto
				.permissions
				.as_ref()
				.unwrap_or(&vec![])
				.iter()
				.filter_map(|p| p.as_ref())
				.map(PermissionsItemDto::from)
				.collect(),
			created_at: dto.created_at.clone(),
			updated_at: dto.updated_at.clone(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct UsersDetailQueryDto {
	pub id: String,
	pub fullname: String,
	pub legal_name: Option<String>,
	pub email: String,
	pub avatar: Option<String>,
	pub is_active: bool,
	pub is_deleted: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub profile_extension: Option<UserProfileExtensionDto>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone_number: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone_for_verification: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gender: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub domicile: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bio: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_education: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub linkedin_url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub github_url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cv_url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub portfolio_url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub website_url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub twitter_url: Option<String>,
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
	pub password: String,
	pub role: RolesDetailQueryDto,
	pub created_at: String,
	pub updated_at: String,
	pub mentor_id: Option<String>,
}

impl UsersDetailQueryDto {
	pub fn from(self) -> Self {
		self
	}
}

impl UsersDetailQueryDto {
	pub fn from_profile_extension(mut self) -> Self {
		if let Some(ext) = &self.profile_extension {
			self.phone_number = ext.phone_number.clone();
			self.phone_for_verification = ext.phone_for_verification.clone();
			self.gender = ext.gender.clone();
			self.domicile = ext.domicile.clone();
			self.bio = ext.bio.clone();
			self.last_education = ext.last_education.clone();
			self.linkedin_url = ext.linkedin_url.clone();
			self.github_url = ext.github_url.clone();
			self.cv_url = ext.cv_url.clone();
			self.portfolio_url = ext.portfolio_url.clone();
		}
		self
	}
}

impl std::fmt::Display for UsersDetailQueryDto {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.id)
	}
}

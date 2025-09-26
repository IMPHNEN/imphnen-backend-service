use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;
use crate::permissions::{PermissionsQueryDto, PermissionsItemDto};

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


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolesDetailQueryDto {
	pub id: Thing,
	pub name: String,
	pub permissions: Option<Vec<Option<PermissionsQueryDto>>>,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl Default for RolesDetailQueryDto {
	fn default() -> Self {
		Self {
			id: Thing::from(("".to_string(), surrealdb::sql::Id::Number(0))),
			name: String::new(),
			permissions: None,
			is_deleted: false,
			created_at: None,
			updated_at: None,
		}
	}
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
			id: dto.id.id.to_raw(),
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersDetailQueryDto {
	pub id: Thing,
	pub fullname: String,
	pub legal_name: Option<String>,
	pub email: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub phone_for_verification: Option<String>,
	pub is_active: bool,
	pub is_deleted: bool,
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
	pub password: String,
	pub role: RolesDetailQueryDto,
	pub created_at: String,
	pub updated_at: String,
	pub mentor_id: Option<Thing>,
}

impl UsersDetailQueryDto {
	pub fn from(self) -> Self {
		self
	}
}
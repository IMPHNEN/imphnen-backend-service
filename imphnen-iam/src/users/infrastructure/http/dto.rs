use crate::users::domain::{UserEntity, UserListItem};
use imphnen_entities::{
	RolesDetailItemDto, UsersDetailQueryDto, users::UserProfileExtensionDto,
};
use imphnen_libs::ZodValidate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use zod_rs::prelude::*;

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(description = "File upload form data for multipart/form-data")]
pub struct FileUploadSchema {
	#[schema(format = "binary")]
	pub file: String,
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

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct UsersCreateRequestDto {
	#[zod(email, min_length(1))]
	pub email: String,
	#[zod(min_length(8), regex(pattern = "^[A-Za-z\\d@$!%*?&]{8,}$"))]
	pub password: String,
	#[zod(min_length(2))]
	pub fullname: String,
	pub is_active: bool,
	pub role_id: String,
	pub avatar: Option<String>,
}

impl ZodValidate for UsersCreateRequestDto {
	fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
		Self::validate_and_parse(value).map_err(|e| e.to_string())
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersUpdateRequestDto {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub email: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub password: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub fullname: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub legal_name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_active: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub profile_extension: Option<UserProfileExtensionDto>,
}

impl ZodValidate for UsersUpdateRequestDto {
	fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
		serde_json::from_value(value.clone()).map_err(|e| e.to_string())
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Default)]
pub struct UsersDetailItemDto {
	pub id: String,
	pub role: RolesDetailItemDto,
	pub fullname: String,
	pub legal_name: Option<String>,
	pub email: String,
	pub avatar: Option<String>,
	pub is_active: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub profile_extension: Option<UserProfileExtensionDto>,
	pub created_at: String,
	pub updated_at: String,
}

impl From<UserEntity> for UsersDetailItemDto {
	fn from(e: UserEntity) -> Self {
		Self {
			id: e.id,
			role: RolesDetailItemDto::from(&e.role),
			fullname: e.fullname,
			legal_name: e.legal_name,
			email: e.email,
			avatar: e.avatar,
			is_active: e.is_active,
			profile_extension: e.profile_extension,
			created_at: e.created_at,
			updated_at: e.updated_at,
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
	pub is_active: bool,
	pub created_at: String,
	pub updated_at: String,
}

impl From<&UsersDetailQueryDto> for UsersDetailItemDto {
	fn from(dto: &UsersDetailQueryDto) -> Self {
		Self {
			id: dto.id.clone(),
			role: RolesDetailItemDto::from(&dto.role),
			fullname: dto.fullname.clone(),
			legal_name: dto.legal_name.clone(),
			email: dto.email.clone(),
			avatar: dto.avatar.clone(),
			is_active: dto.is_active,
			profile_extension: dto.profile_extension.clone(),
			created_at: dto.created_at.clone(),
			updated_at: dto.updated_at.clone(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersMeResponseDto {
	#[serde(flatten)]
	pub user: UsersDetailItemDto,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hackathon: Option<HackathonProfileDto>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub qr: Option<QrProfileDto>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mentor: Option<MentorProfileDto>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sessions: Option<Vec<SessionProfileDto>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionProfileDto {
	pub id: String,
	pub topic: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	pub scheduled_at: String,
	pub duration_minutes: i32,
	pub session_type: String,
	pub status: String,
	pub role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct HackathonProfileDto {
	pub is_admin: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone_number: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub location: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bio: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub skills: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QrProfileDto {
	pub role: String,
	pub provider: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MentorProfileDto {
	pub mentor_id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub current_company: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub current_role: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub years_of_experience: Option<i32>,
}

impl From<UserListItem> for UsersListItemDto {
	fn from(item: UserListItem) -> Self {
		Self {
			id: item.id,
			role: item.role,
			fullname: item.fullname,
			email: item.email,
			avatar: item.avatar,
			is_active: item.is_active,
			created_at: item.created_at,
			updated_at: item.updated_at,
		}
	}
}

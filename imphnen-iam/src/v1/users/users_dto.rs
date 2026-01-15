use imphnen_entities::{UsersDetailQueryDto, RolesDetailQueryDto, RolesDetailItemDto, users::UserProfileExtensionDto};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
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
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_active: Option<bool>,
	#[validate(length(min = 1, message = "Avatar is required"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_extension: Option<UserProfileExtensionDto>,
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

impl UsersDetailItemDto {
	pub fn from(dto: &UsersDetailQueryDto) -> Self {
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

    pub fn from_schema(schema: &UsersSchema) -> Self {
        Self {
            id: schema.id.clone(),
            role: RolesDetailItemDto::default(), // Placeholder, role needs to be fetched
            fullname: schema.fullname.clone().unwrap_or_default(),
            legal_name: schema.legal_name.clone(),
            email: schema.email.clone().unwrap_or_default(),
            avatar: schema.avatar.clone(),
            is_active: schema.is_active,
            profile_extension: schema.profile_extension.clone(),
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
	pub is_active: bool,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersListQueryDto {
	pub id: String,
	pub role: RolesDetailQueryDto,
	pub fullname: String,
	pub email: String,
	pub avatar: Option<String>,
	pub is_active: bool,
	pub created_at: String,
	pub updated_at: String,
}

impl UsersListQueryDto {
	pub fn from(self) -> UsersListItemDto {
		UsersListItemDto {
			id: self.id,
			role: self.role.name,
			fullname: self.fullname,
			email: self.email,
			avatar: self.avatar,
			is_active: self.is_active,
			created_at: self.created_at,
			updated_at: self.updated_at,
		}
	}
}


impl From<&UsersDetailItemDto> for UsersDetailQueryDto {
	fn from(dto: &UsersDetailItemDto) -> Self {
		let mut s = UsersDetailQueryDto::default();
		s.id = dto.id.clone();
		s.fullname = dto.fullname.clone();
		s.legal_name = dto.legal_name.clone();
		s.email = dto.email.clone();
		s.avatar = dto.avatar.clone();
		s.is_active = dto.is_active;
		s.is_deleted = false;
		s.profile_extension = dto.profile_extension.clone();
		s.password = String::new();
		s.role = RolesDetailQueryDto::default();
		s.created_at = dto.created_at.clone();
		s.updated_at = dto.updated_at.clone();
		s.mentor_id = None;
		s.from_profile_extension()
        
		}
}

impl UsersDetailItemDto {
	   pub fn extract_permissions_from_user_role(&self) -> Vec<String> {
	       self.role.permissions.iter().map(|p| p.name.clone()).collect()
	   }
}
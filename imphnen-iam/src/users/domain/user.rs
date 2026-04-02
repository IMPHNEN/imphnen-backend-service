use imphnen_entities::{RolesDetailQueryDto, users::UserProfileExtensionDto};

#[derive(Clone, Debug, Default)]
pub struct UserEntity {
	pub id: String,
	pub email: String,
	pub fullname: String,
	pub legal_name: Option<String>,
	pub password: String,
	pub avatar: Option<String>,
	pub is_active: bool,
	pub is_deleted: bool,
	pub role: RolesDetailQueryDto,
	pub profile_extension: Option<UserProfileExtensionDto>,
	pub created_at: String,
	pub updated_at: String,
	pub mentor_id: Option<String>,
}

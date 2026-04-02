use chrono::{DateTime, Utc};
use imphnen_entities::seaorm::auth::users::Model as UserModel;
use imphnen_entities::{PermissionsQueryDto, UsersDetailQueryDto};
use sea_orm::prelude::Json;
use uuid::Uuid;

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum UserReference {
	Id(Uuid),
	Email(String),
	Username(String),
	Model(UserModel),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExtendedUserInfo {
	pub basic_info: UsersDetailQueryDto,
	pub last_login_at: Option<DateTime<Utc>>,
	pub login_count: u64,
	pub account_age_days: i64,
	pub is_recently_active: bool,
}

#[derive(Debug, Clone)]
pub struct UserRegistrationData {
	pub id: Option<Uuid>,
	pub email: String,
	pub password_hash: String,
	pub username: String,
	pub first_name: Option<String>,
	pub last_name: Option<String>,
	pub avatar_url: Option<String>,
	pub metadata: Option<Json>,
	pub role_id: Option<Uuid>,
}

pub fn model_to_dto(
	model: &UserModel,
	role_model: Option<&imphnen_entities::seaorm::auth::roles::Model>,
) -> UsersDetailQueryDto {
	let mut dto = UsersDetailQueryDto::default();
	dto.id = model.id.to_string();
	dto.fullname = format!(
		"{} {}",
		model.first_name.as_deref().unwrap_or(""),
		model.last_name.as_deref().unwrap_or("")
	)
	.trim()
	.to_string();
	dto.legal_name = None;
	dto.email = model.email.clone();
	dto.avatar = model.avatar_url.clone();
	dto.is_active = model.is_active;
	dto.is_deleted = model.deleted_at.is_some();
	dto.profile_extension = model
		.metadata
		.clone()
		.and_then(|m| serde_json::from_value(m).ok());
	dto.password = String::new();

	if let Some(role) = role_model {
		let mut role_dto = imphnen_entities::RolesDetailQueryDto::default();
		role_dto.id = role.id.to_string();
		role_dto.name = role.name.clone();
		role_dto.is_deleted = false;

		if let Some(perms_json) = &role.permissions
			&& let Ok(perms_list) =
				serde_json::from_value::<Vec<String>>(perms_json.clone())
		{
			let dtos = perms_list
				.into_iter()
				.map(|p| {
					Some(PermissionsQueryDto {
						id: Some(p.clone()),
						name: Some(p),
						created_at: None,
						updated_at: None,
					})
				})
				.collect();
			role_dto.permissions = Some(dtos);
		}

		dto.role = role_dto;
	} else {
		dto.role = imphnen_entities::RolesDetailQueryDto::default();
	}

	dto.created_at = model.created_at.to_rfc3339();
	dto.updated_at = model.updated_at.to_rfc3339();
	dto.mentor_id = None;
	dto.from_profile_extension()
}

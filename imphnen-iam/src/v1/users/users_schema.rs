use imphnen_entities::{UsersDetailQueryDto, users::UserProfileExtensionDto};
use super::{UsersCreateRequestDto, UsersUpdateRequestDto};
use imphnen_libs::hash_password; // Keep hash_password
use imphnen_utils::generate_date::get_iso_date; // Keep get_iso_date
use serde::{Deserialize, Serialize};
use uuid::Uuid; // Keep Uuid
use anyhow::Result;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersSchema {
	pub id: String,
	pub fullname: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub legal_name: Option<String>,
	pub email: Option<String>,
	pub password: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar: Option<String>,
	pub is_active: bool,
	pub is_deleted: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mentor_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_extension: Option<UserProfileExtensionDto>,
	pub role_id: Option<Uuid>,
	pub created_at: String,
	pub updated_at: String,
}

impl Default for UsersSchema {
	fn default() -> Self {
		Self {
			id: Uuid::new_v4().to_string(),
			fullname: None,
			legal_name: None,
			email: None,
			password: None,
			avatar: None,
			is_active: false,
			is_deleted: false,
			mentor_id: None,
			profile_extension: None,
			role_id: None,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}
}

impl UsersSchema {
	pub fn from(dto: UsersDetailQueryDto) -> Self {
		Self {
			id: dto.id,
			fullname: Some(dto.fullname),
			legal_name: dto.legal_name,
			email: Some(dto.email),
			avatar: dto.avatar,
			is_active: dto.is_active,
			is_deleted: dto.is_deleted,
			mentor_id: dto.mentor_id,
            profile_extension: dto.profile_extension,
			password: Some(dto.password),
			created_at: dto.created_at,
			updated_at: dto.updated_at,
			role_id: Uuid::parse_str(&dto.role.id).ok(),
		}
	}

	pub fn create(dto: UsersCreateRequestDto) -> Result<Self> {
	    let password_hash = hash_password(&dto.password).map_err(|e| anyhow::anyhow!("Hash password failed: {}", e))?;
	    Ok(Self {
	        id: Uuid::new_v4().to_string(),
	        email: Some(dto.email),
	        password: Some(password_hash),
	        fullname: Some(dto.fullname),
	        is_active: dto.is_active,
            role_id: Some(Uuid::parse_str(&dto.role_id)?),
	        ..Default::default()
	    })
	}

	pub fn update(_user: UsersUpdateRequestDto, id: String) -> Self {
		Self {
			id,
			updated_at: get_iso_date(),
			created_at: String::new(),
            fullname: None,
            legal_name: None,
            email: None,
            password: None,
            avatar: None,
            is_active: false,
            is_deleted: false,
            mentor_id: None,
            profile_extension: None,
            role_id: None,
		}
	}

	pub fn partial_update(current_user: UsersDetailQueryDto, user: UsersUpdateRequestDto) -> Self {
		let mut schema = Self::from(current_user);
		schema.updated_at = get_iso_date();

		// Only update fields that are provided (Some)
		if let Some(fullname) = user.fullname {
			schema.fullname = Some(fullname);
		}
		if let Some(email) = user.email {
			schema.email = Some(email);
		}
		if let Some(password) = user.password {
			schema.password = Some(hash_password(&password).unwrap_or(password));
		}
		if let Some(is_active) = user.is_active {
			schema.is_active = is_active;
		}
		if let Some(role_id) = user.role_id {
			schema.role_id = Uuid::parse_str(&role_id).ok();
		}
		
		schema.legal_name = user.legal_name;
		schema.avatar = user.avatar;
        schema.profile_extension = user.profile_extension;

		schema
	}

	pub fn patch_password(dto: UsersDetailQueryDto, password: String) -> Self {
		Self {
			password: Some(password),
			id: dto.id.clone(),
			fullname: Some(dto.fullname),
			legal_name: dto.legal_name,
			email: Some(dto.email),
			avatar: dto.avatar,
			is_active: dto.is_active,
			is_deleted: dto.is_deleted,
			mentor_id: dto.mentor_id,
            profile_extension: dto.profile_extension,
			created_at: dto.created_at,
			updated_at: dto.updated_at,
			role_id: Uuid::parse_str(&dto.role.id).ok(),
		}
	}

	pub fn update_mentor_id(mut self, mentor_id: Option<String>) -> Self {
		self.mentor_id = mentor_id; // Removed make_thing_from_enum
		self.updated_at = get_iso_date();
		self
	}
	
	/// Convert role string to Uuid for database operations
	pub fn get_role_id(&self) -> Result<Option<Uuid>> {
		Ok(self.role_id) // Removed parsing from `role` field
	}
}

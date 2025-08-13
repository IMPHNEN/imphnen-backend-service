use super::{UsersCreateRequestDto, UsersDetailQueryDto, UsersUpdateRequestDto, ExperienceDto, EducationDto};
use imphnen_libs::{ResourceEnum, hash_password};
use imphnen_utils::extract_id;
use imphnen_utils::{get_iso_date, make_thing};
use serde::{Deserialize, Serialize};
use surrealdb::{Uuid, sql::Thing};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersSchema {
	pub id: Thing,
	pub fullname: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub legal_name: Option<String>,
	pub email: String,
	pub password: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar: Option<String>,
	pub phone_number: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone_for_verification: Option<String>,
	pub is_active: bool,
	pub is_deleted: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mentor_id: Option<Thing>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gender: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub birthdate: Option<String>,
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
	pub role: Thing,
	pub created_at: String,
	pub updated_at: String,
}

impl Default for UsersSchema {
	fn default() -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::Users.to_string(),
				&Uuid::new_v4().to_string(),
			),
			fullname: String::new(),
			legal_name: None,
			email: String::new(),
			password: hash_password("").unwrap(),
			avatar: None,
			phone_number: String::new(),
			phone_for_verification: None,
			is_active: false,
			is_deleted: false,
			mentor_id: None, // Regular users should not have a mentor_id by default
			gender: None,
			birthdate: None,
			domicile: None,
			bio: None,
			last_education: None,
			linkedin_url: None,
			github_url: None,
			cv_url: None,
			portfolio_url: None,
			website_url: None,
			twitter_url: None,
			location: None,
			skills: None,
			experience: None,
			education: None,
			role: make_thing(
				&ResourceEnum::Roles.to_string(),
				"5713cb37-dc02-4e87-8048-d7a41d352059",
			),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}
}

impl UsersSchema {
	pub fn from(dto: UsersDetailQueryDto) -> Self {
		Self {
			id: dto.id,
			fullname: dto.fullname,
			legal_name: dto.legal_name,
			email: dto.email,
			avatar: dto.avatar,
			phone_number: dto.phone_number,
			phone_for_verification: dto.phone_for_verification,
			is_active: dto.is_active,
			is_deleted: dto.is_deleted,
			mentor_id: dto.mentor_id, // Use the actual mentor_id from the DTO, could be None
			gender: dto.gender,
			birthdate: dto.birthdate,
			domicile: dto.domicile,
			bio: dto.bio,
			last_education: dto.last_education,
			linkedin_url: dto.linkedin_url,
			github_url: dto.github_url,
			cv_url: dto.cv_url,
			portfolio_url: dto.portfolio_url,
			website_url: dto.website_url,
			twitter_url: dto.twitter_url,
			location: dto.location,
			skills: dto.skills,
			experience: dto.experience,
			education: dto.education,
			password: dto.password,
			created_at: dto.created_at,
			updated_at: dto.updated_at,
			role: make_thing(&ResourceEnum::Roles.to_string(), &extract_id(&dto.role.id)),
		}
	}

	pub fn update(_user: UsersUpdateRequestDto, id: String) -> Self {
		Self {
			id: make_thing(&ResourceEnum::Users.to_string(), &id),
			updated_at: get_iso_date(),
			// Set defaults for required fields - these should be overridden by actual data from DB
			..Default::default()
		}
	}

	pub fn partial_update(current_user: UsersDetailQueryDto, user: UsersUpdateRequestDto) -> Self {
		let mut schema = Self::from(current_user);
		schema.updated_at = get_iso_date();

		// Only update fields that are provided (Some)
		if let Some(fullname) = user.fullname {
			schema.fullname = fullname;
		}
		if let Some(email) = user.email {
			schema.email = email;
		}
		if let Some(password) = user.password {
			schema.password = hash_password(&password).unwrap_or_else(|_| password);
		}
		if let Some(phone_number) = user.phone_number {
			schema.phone_number = phone_number;
		}
		if let Some(is_active) = user.is_active {
			schema.is_active = is_active;
		}
		if let Some(role_id) = user.role_id {
			schema.role = make_thing(&ResourceEnum::Roles.to_string(), &role_id);
		}
		
		// Optional fields - only update if provided
		if let Some(legal_name) = user.legal_name {
			schema.legal_name = Some(legal_name);
		}
		if let Some(phone_for_verification) = user.phone_for_verification {
			schema.phone_for_verification = Some(phone_for_verification);
		}
		if let Some(gender) = user.gender {
			schema.gender = Some(gender);
		}
		if let Some(birthdate) = user.birthdate {
			schema.birthdate = Some(birthdate);
		}
		if let Some(domicile) = user.domicile {
			schema.domicile = Some(domicile);
		}
		if let Some(bio) = user.bio {
			schema.bio = Some(bio);
		}
		if let Some(last_education) = user.last_education {
			schema.last_education = Some(last_education);
		}
		if let Some(linkedin_url) = user.linkedin_url {
			schema.linkedin_url = Some(linkedin_url);
		}
		if let Some(github_url) = user.github_url {
			schema.github_url = Some(github_url);
		}
		if let Some(cv_url) = user.cv_url {
			schema.cv_url = Some(cv_url);
		}
		if let Some(portfolio_url) = user.portfolio_url {
			schema.portfolio_url = Some(portfolio_url);
		}
		if let Some(website_url) = user.website_url {
			schema.website_url = Some(website_url);
		}
		if let Some(twitter_url) = user.twitter_url {
			schema.twitter_url = Some(twitter_url);
		}
		if let Some(location) = user.location {
			schema.location = Some(location);
		}
		if let Some(skills) = user.skills {
			schema.skills = Some(skills);
		}
		if let Some(experience) = user.experience {
			schema.experience = Some(experience);
		}
		if let Some(education) = user.education {
			schema.education = Some(education);
		}
		if let Some(avatar) = user.avatar {
			schema.avatar = Some(avatar);
		}

		schema
	}

	pub fn create(user: UsersCreateRequestDto) -> Self {
		let password = hash_password(&user.password).unwrap();
		Self {
			id: make_thing(
				&ResourceEnum::Users.to_string(),
				&Uuid::new_v4().to_string(),
			),
			fullname: user.fullname,
			legal_name: None,
			email: user.email,
			password,
			phone_number: user.phone_number,
			phone_for_verification: None,
			is_active: false,
			mentor_id: None, // Regular users should not have a mentor_id by default
			gender: None,
			birthdate: None,
			domicile: None,
			bio: None,
			last_education: None,
			linkedin_url: None,
			github_url: None,
			cv_url: None,
			portfolio_url: None,
			website_url: None,
			twitter_url: None,
			location: None,
			skills: None,
			experience: None,
			education: None,
			avatar: user.avatar,
			is_deleted: false,
			role: make_thing(&ResourceEnum::Roles.to_string(), &user.role_id),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}

	pub fn patch_password(dto: UsersDetailQueryDto, password: String) -> Self {
		Self {
			password,
			id: dto.id.clone(),
			..Self::from(dto)
		}
	}

	pub fn update_mentor_id(mut self, mentor_id: Option<String>) -> Self {
		self.mentor_id = match mentor_id {
			Some(id) => Some(make_thing(&ResourceEnum::Users.to_string(), &id)),
			None => None, // Set to None if no mentor_id provided
		};
		self.updated_at = get_iso_date();
		self
	}
}

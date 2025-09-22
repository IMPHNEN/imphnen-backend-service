use super::{TeamsCreateRequestDto, TeamsUpdateRequestDto};
use imphnen_libs::ResourceEnum;
use imphnen_utils::{get_iso_date, make_thing_from_enum};
use serde::{Deserialize, Serialize};
use surrealdb::{Uuid, sql::Thing};
use chrono::{DateTime, Utc, Duration};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeamsSchema {
	pub id: Thing,
	pub name: String,
	pub description: Option<String>,
	pub leader_id: Thing,
	pub is_open: bool,
	pub max_members: Option<i32>,
	pub skills_required: Option<Vec<String>>,
	pub location: Option<String>,
	pub avatar: Option<String>,
	pub website_url: Option<String>,
	pub github_url: Option<String>,
	pub is_active: bool,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeamMembersSchema {
	pub id: Thing,
	pub team_id: Thing,
	pub user_id: Thing,
	pub role: String,
	pub joined_at: String,
	pub is_active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeamInvitationsSchema {
	pub id: Thing,
	pub team_id: Thing,
	pub email: String,
	pub inviter_id: Thing,
	pub invite_code: String,  // Renamed from 'token' to avoid SurrealDB protected field conflict
	pub expires_at: DateTime<Utc>,
	pub status: String,
	pub invited_at: String,
	pub accepted_at: Option<String>,
}

impl Default for TeamsSchema {
	fn default() -> Self {
		Self {
			id: make_thing_from_enum(
				ResourceEnum::Teams,
				&Uuid::new_v4().to_string(),
			),
			name: String::new(),
			description: None,
			leader_id: make_thing_from_enum(
				ResourceEnum::Users,
				&Uuid::new_v4().to_string(),
			),
			is_open: false,
			max_members: None,
			skills_required: None,
			location: None,
			avatar: None,
			website_url: None,
			github_url: None,
			is_active: true,
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}
}

impl Default for TeamMembersSchema {
	fn default() -> Self {
		Self {
			id: make_thing_from_enum(
				ResourceEnum::TeamMembers,
				&Uuid::new_v4().to_string(),
			),
			team_id: make_thing_from_enum(
				ResourceEnum::Teams,
				&Uuid::new_v4().to_string(),
			),
			user_id: make_thing_from_enum(
				ResourceEnum::Users,
				&Uuid::new_v4().to_string(),
			),
			role: "member".to_string(),
			joined_at: get_iso_date(),
			is_active: true,
		}
	}
}

impl Default for TeamInvitationsSchema {
	fn default() -> Self {
		Self {
			id: make_thing_from_enum(
				ResourceEnum::TeamInvitations,
				&Uuid::new_v4().to_string(),
			),
			team_id: make_thing_from_enum(
				ResourceEnum::Teams,
				&Uuid::new_v4().to_string(),
			),
			email: String::new(),
			inviter_id: make_thing_from_enum(
				ResourceEnum::Users,
				&Uuid::new_v4().to_string(),
			),
			invite_code: String::new(),  // Renamed from 'token'
			expires_at: Utc::now() + Duration::hours(72),
			status: "pending".to_string(),
			invited_at: get_iso_date(),
			accepted_at: None,
		}
	}
}

impl TeamsSchema {
	pub fn create(dto: TeamsCreateRequestDto, leader_id: String) -> Self {
		Self {
			id: make_thing_from_enum(
				ResourceEnum::Teams,
				&Uuid::new_v4().to_string(),
			),
			name: dto.name,
			description: dto.description,
			leader_id: make_thing_from_enum(ResourceEnum::Users, &leader_id),
			is_open: dto.is_open.unwrap_or(false),
			max_members: dto.max_members,
			skills_required: dto.skills_required,
			location: dto.location,
			avatar: dto.avatar,
			website_url: dto.website_url,
			github_url: dto.github_url,
			is_active: true,
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}

	pub fn update(self, dto: TeamsUpdateRequestDto) -> Self {
		Self {
			name: dto.name.unwrap_or(self.name),
			description: dto.description.or(self.description),
			is_open: dto.is_open.unwrap_or(self.is_open),
			max_members: dto.max_members.or(self.max_members),
			skills_required: dto.skills_required.or(self.skills_required),
			location: dto.location.or(self.location),
			avatar: dto.avatar.or(self.avatar),
			website_url: dto.website_url.or(self.website_url),
			github_url: dto.github_url.or(self.github_url),
			updated_at: get_iso_date(),
			..self
		}
	}
}

impl TeamMembersSchema {
	pub fn create(team_id: String, user_id: String, role: Option<String>) -> Self {
		Self {
			id: make_thing_from_enum(
				ResourceEnum::TeamMembers,
				&Uuid::new_v4().to_string(),
			),
			team_id: make_thing_from_enum(ResourceEnum::Teams, &team_id),
			user_id: make_thing_from_enum(ResourceEnum::Users, &user_id),
			role: role.unwrap_or("member".to_string()),
			joined_at: get_iso_date(),
			is_active: true,
		}
	}
}

impl TeamInvitationsSchema {
	pub fn create(team_id: String, email: String, inviter_id: String, invite_code: String) -> Self {
		Self {
			id: make_thing_from_enum(
				ResourceEnum::TeamInvitations,
				&Uuid::new_v4().to_string(),
			),
			team_id: make_thing_from_enum(ResourceEnum::Teams, &team_id),
			email,
			inviter_id: make_thing_from_enum(ResourceEnum::Users, &inviter_id),
			invite_code,  // Renamed from 'token'
			expires_at: Utc::now() + Duration::hours(72),
			status: "pending".to_string(),
			invited_at: get_iso_date(),
			accepted_at: None,
		}
	}

	pub fn accept(mut self) -> Self {
		self.status = "accepted".to_string();
		self.accepted_at = Some(get_iso_date());
		self
	}

	pub fn is_expired(&self) -> bool {
		Utc::now() > self.expires_at
	}
}
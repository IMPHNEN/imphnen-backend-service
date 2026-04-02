use crate::users::domain::entity::{HackathonUserEntity, UpdateUserInput};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
	pub id: Uuid,
	pub email: String,
	pub fullname: String,
	pub avatar: Option<String>,
	pub phone_number: Option<String>,
	pub location: Option<String>,
	pub bio: Option<String>,
	pub skills: Option<Vec<String>>,
	pub is_active: Option<bool>,
	pub created_at: Option<DateTime<Utc>>,
	pub updated_at: Option<DateTime<Utc>>,
}

impl From<HackathonUserEntity> for UserResponse {
	fn from(e: HackathonUserEntity) -> Self {
		Self {
			id: e.id,
			email: e.email,
			fullname: e.fullname,
			avatar: e.avatar,
			phone_number: e.phone_number,
			location: e.location,
			bio: e.bio,
			skills: e.skills,
			is_active: e.is_active,
			created_at: e.created_at,
			updated_at: e.updated_at,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
	pub fullname: Option<String>,
	pub phone_number: Option<String>,
	pub avatar: Option<String>,
	pub location: Option<String>,
	pub bio: Option<String>,
	pub skills: Option<Vec<String>>,
}

impl From<UpdateUserRequest> for UpdateUserInput {
	fn from(r: UpdateUserRequest) -> Self {
		Self {
			fullname: r.fullname,
			phone_number: r.phone_number,
			avatar: r.avatar,
			location: r.location,
			bio: r.bio,
			skills: r.skills,
		}
	}
}

use crate::ResourceEnum;
use imphnen_utils::make_thing_from_enum;
use serde::{Deserialize, Serialize};
use surrealdb::{Uuid, sql::Thing};

use super::{PermissionsItemDto, PermissionsQueryDto};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PermissionsSchema {
	pub id: Thing,
	pub name: String,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl Default for PermissionsSchema {
	fn default() -> Self {
		Self {
			id: make_thing_from_enum(
				ResourceEnum::Permissions,
				&Uuid::new_v4().to_string(),
			),
			name: String::new(),
			is_deleted: false,
			created_at: None,
			updated_at: None,
		}
	}
}

impl PermissionsSchema {
	pub fn list(&self) -> PermissionsItemDto {
		PermissionsItemDto {
			id: self.id.id.to_raw(),
			name: self.name.clone(),
			created_at: self.created_at.clone(),
			updated_at: self.updated_at.clone(),
		}
	}

	pub fn from(dto: PermissionsQueryDto) -> Self {
		Self {
			id: dto.id.unwrap_or_else(|| make_thing_from_enum(ResourceEnum::Permissions, "unknown")),
			name: dto.name.unwrap_or_default(),
			is_deleted: false,
			created_at: dto.created_at,
			updated_at: dto.updated_at,
		}
	}
}

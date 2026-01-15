use serde::{Deserialize, Serialize};
use uuid::Uuid;

use imphnen_entities::{PermissionsItemDto, PermissionsQueryDto};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PermissionsSchema {
	pub id: Uuid,
	pub name: String,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl Default for PermissionsSchema {
	fn default() -> Self {
		Self {
			id: Uuid::new_v4(),
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
			id: self.to_string(),
			name: self.name.clone(),
			created_at: self.created_at.clone(),
			updated_at: self.updated_at.clone(),
		}
	}

	pub fn from(dto: PermissionsQueryDto) -> Self {
		Self {
			id: Uuid::parse_str(&dto.id.unwrap_or_default()).unwrap_or(Uuid::new_v4()),
			name: dto.name.unwrap_or_default(),
			is_deleted: false,
			created_at: dto.created_at,
			updated_at: dto.updated_at,
		}
	}
}

impl std::fmt::Display for PermissionsSchema {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.id)
	}
}

use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_roles")]
pub struct Model {
	#[sea_orm(primary_key, default = "gen_random_uuid()", auto_increment = false)]
	pub id: Uuid,

	#[sea_orm(unique, not_null)]
	pub name: String,

	#[sea_orm(not_null)]
	pub description: String,

	#[sea_orm(default = "false")]
	pub is_system_role: bool,

	#[sea_orm(default = "false")]
	pub is_default: bool,

	#[sea_orm(type = "jsonb", nullable)]
	pub permissions: Option<serde_json::Value>,

	#[sea_orm(not_null, default = "now()")]
	pub created_at: DateTime<Utc>,

	#[sea_orm(not_null, default = "now()")]
	pub updated_at: DateTime<Utc>,

	#[sea_orm(nullable)]
	pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, sea_orm::EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Default, Serialize, Deserialize)]
pub struct RoleBuilder {
	name: Option<String>,
	description: Option<String>,
	is_system_role: Option<bool>,
	is_default: Option<bool>,
	permissions: Option<Vec<String>>,
}

impl RoleBuilder {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	#[must_use]
	pub fn name(mut self, name: String) -> Self {
		self.name = Some(name);
		self
	}

	#[must_use]
	pub fn description(mut self, description: String) -> Self {
		self.description = Some(description);
		self
	}

	#[must_use]
	pub fn is_system_role(mut self, is_system_role: bool) -> Self {
		self.is_system_role = Some(is_system_role);
		self
	}

	#[must_use]
	pub fn is_default(mut self, is_default: bool) -> Self {
		self.is_default = Some(is_default);
		self
	}

	#[must_use]
	pub fn permissions(mut self, permissions: Vec<String>) -> Self {
		self.permissions = Some(permissions);
		self
	}

	pub fn build(self) -> Result<ActiveModel, String> {
		let mut active_model = <ActiveModel as std::default::Default>::default();

		if let Some(name) = self.name {
			active_model.name = Set(name);
		} else {
			return Err("Role name is required".to_string());
		}

		if let Some(description) = self.description {
			active_model.description = Set(description);
		} else {
			return Err("Role description is required".to_string());
		}

		if let Some(is_system_role) = self.is_system_role {
			active_model.is_system_role = Set(is_system_role);
		}

		if let Some(is_default) = self.is_default {
			active_model.is_default = Set(is_default);
		}

		if let Some(permissions) = self.permissions {
			active_model.permissions = Set(Some(serde_json::Value::Array(
				permissions
					.into_iter()
					.map(serde_json::Value::String)
					.collect(),
			)));
		}

		Ok(active_model)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_role_model_creation() {
		let role = RoleBuilder::new()
			.name("admin".to_string())
			.description("Administrator role".to_string())
			.is_system_role(true)
			.is_default(false)
			.build();

		assert!(role.is_ok());
		let role_model = role.unwrap();
		assert_eq!(role_model.name, Set("admin".to_string()));
		assert_eq!(
			role_model.description,
			Set("Administrator role".to_string())
		);
		assert_eq!(role_model.is_system_role, Set(true));
		assert_eq!(role_model.is_default, Set(false));
	}
}

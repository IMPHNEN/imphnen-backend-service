use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
	Clone,
	Debug,
	PartialEq,
	DeriveEntityModel,
	Serialize,
	Deserialize,
	imphnen_macros::Builder,
)]
#[sea_orm(table_name = "app_users")]
pub struct Model {
	#[sea_orm(primary_key, default = "gen_random_uuid()", auto_increment = false)]
	pub id: Uuid,

	#[sea_orm(unique, not_null)]
	pub email: String,

	#[sea_orm(not_null)]
	pub password_hash: String,

	#[sea_orm(not_null)]
	pub username: String,

	#[sea_orm(column_name = "role_id", nullable)]
	pub role_id: Option<Uuid>,

	#[sea_orm(nullable)]
	pub first_name: Option<String>,

	#[sea_orm(nullable)]
	pub last_name: Option<String>,

	#[sea_orm(nullable)]
	pub avatar_url: Option<String>,

	#[sea_orm(default = "false")]
	pub is_verified: bool,

	#[sea_orm(default = "false")]
	pub is_active: bool,

	#[sea_orm(type = "jsonb", nullable)]
	pub metadata: Option<serde_json::Value>,

	#[sea_orm(not_null, default = "now()")]
	pub created_at: DateTime<Utc>,

	#[sea_orm(not_null, default = "now()")]
	pub updated_at: DateTime<Utc>,

	#[sea_orm(nullable)]
	pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, sea_orm::EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::roles_permissions::Entity")]
	RolesPermissions,
	#[sea_orm(
		belongs_to = "super::roles::Entity",
		from = "Column::RoleId",
		to = "super::roles::Column::Id"
	)]
	Role,
}

impl Related<super::roles_permissions::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::RolesPermissions.def()
	}
}

impl Related<super::roles::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Role.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

pub type UserBuilder = ModelBuilder;

impl ModelBuilder {
	pub fn build(self) -> Result<ActiveModel, String> {
		let mut active_model = <ActiveModel as std::default::Default>::default();

		if let Some(email) = self.email {
			active_model.email = Set(email);
		} else {
			return Err("Email is required".to_string());
		}

		if let Some(password_hash) = self.password_hash {
			active_model.password_hash = Set(password_hash);
		} else {
			return Err("Password hash is required".to_string());
		}

		if let Some(username) = self.username {
			active_model.username = Set(username);
		} else {
			return Err("Username is required".to_string());
		}

		if let Some(role_id) = self.role_id {
			active_model.role_id = Set(Some(role_id));
		}

		if let Some(first_name) = self.first_name {
			active_model.first_name = Set(Some(first_name));
		}

		if let Some(last_name) = self.last_name {
			active_model.last_name = Set(Some(last_name));
		}

		if let Some(avatar_url) = self.avatar_url {
			active_model.avatar_url = Set(Some(avatar_url));
		}

		if let Some(is_verified) = self.is_verified {
			active_model.is_verified = Set(is_verified);
		}

		if let Some(is_active) = self.is_active {
			active_model.is_active = Set(is_active);
		}

		if let Some(metadata) = self.metadata {
			active_model.metadata = Set(Some(metadata));
		}

		Ok(active_model)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_user_model_creation() {
		let user = UserBuilder::new()
			.email("test@example.com".to_string())
			.password_hash("hashed_password".to_string())
			.username("testuser".to_string())
			.first_name("Test".to_string())
			.last_name("User".to_string())
			.is_verified(true)
			.is_active(true)
			.build();

		assert!(user.is_ok());
		let user_model = user.unwrap();
		assert_eq!(user_model.email, Set("test@example.com".to_string()));
		assert_eq!(user_model.password_hash, Set("hashed_password".to_string()));
		assert_eq!(user_model.username, Set("testuser".to_string()));
		assert_eq!(user_model.first_name, Set(Some("Test".to_string())));
		assert_eq!(user_model.last_name, Set(Some("User".to_string())));
		assert_eq!(user_model.is_verified, Set(true));
		assert_eq!(user_model.is_active, Set(true));
	}

	#[test]
	fn test_user_model_missing_required_fields() {
		let user = UserBuilder::new()
			.email("test@example.com".to_string())
			.username("testuser".to_string())
			.build();

		assert!(user.is_err());
		assert_eq!(user.unwrap_err(), "Password hash is required");
	}
}

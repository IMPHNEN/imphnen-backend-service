use super::RolesListItemDto;
use imphnen_entities::seaorm::auth::roles::Model as RolesModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolesSchema {
	pub id: Uuid,
	pub name: String,
	pub description: String,
	pub is_system_role: bool,
	pub is_default: bool,
	pub permissions: Vec<String>,
	pub created_at: String,
	pub updated_at: String,
	pub deleted_at: Option<String>,
}

impl From<&RolesModel> for RolesSchema {
	fn from(model: &RolesModel) -> Self {
		let permissions = model.permissions
			.as_ref()
			.and_then(|p| serde_json::from_value(p.clone()).ok())
			.unwrap_or_default();
		
		Self {
			id: model.id,
			name: model.name.clone(),
			description: model.description.clone(),
			is_system_role: model.is_system_role,
			is_default: model.is_default,
			permissions,
			created_at: model.created_at.to_rfc3339(),
			updated_at: model.updated_at.to_rfc3339(),
			deleted_at: model.deleted_at.map(|dt| dt.to_rfc3339()),
		}
	}
}

impl RolesSchema {
	pub fn list(&self) -> RolesListItemDto {
		RolesListItemDto {
			id: self.id.to_string(),
			name: self.name.clone(),
			permissions_count: self.permissions.len(),
			created_at: Some(self.created_at.clone()),
			updated_at: Some(self.updated_at.clone()),
		}
	}
}

use imphnen_entities::{PermissionsItemDto, PermissionsQueryDto, PermissionsEnum};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use strum::IntoEnumIterator;

use super::RolesSchema;
use imphnen_entities::seaorm::auth::roles::Model as RolesModel;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct RolesRequestUpdateDto {
	#[validate(length(min = 1, message = "Role name must not be empty"))]
	pub name: Option<String>,
	pub permissions: Option<Vec<String>>,
	pub overwrite: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct RolesRequestCreateDto {
	#[validate(length(min = 1, message = "Role name must not be empty"))]
	pub name: String,
	pub permissions: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesListItemDto {
	pub id: String,
	pub name: String,
	pub permissions_count: usize,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Default)] // Added Default derive
pub struct RolesDetailItemDto {
	pub id: String,
	pub name: String,
	pub description: String,
	pub is_system_role: bool,
	pub is_default: bool,
	pub permissions: Vec<PermissionsItemDto>,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl From<&RolesModel> for RolesDetailItemDto {
	fn from(model: &RolesModel) -> Self {
		let schema = RolesSchema::from(model);
		
		let mut permissions_dto = vec![];
		if let Some(serde_json::Value::Array(perms)) = &model.permissions {
			for p in perms {
				if let Some(p_str) = p.as_str() {
					// Find matching enum
					for enum_val in PermissionsEnum::iter() {
						if enum_val.to_string() == p_str {
							permissions_dto.push(PermissionsItemDto {
								id: enum_val.id(),
								name: p_str.to_string(),
								created_at: None,
								updated_at: None,
							});
							break;
						}
					}
				}
			}
		}

		Self {
			id: schema.id.to_string(),
			name: schema.name.clone(),
			description: schema.description.clone(),
			is_system_role: schema.is_system_role,
			is_default: schema.is_default,
			permissions: permissions_dto,
			created_at: Some(schema.created_at.clone()),
			updated_at: Some(schema.updated_at.clone()),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolesDetailQueryDto {
	pub id: Uuid,
	pub name: String,
	pub permissions: Option<Vec<Option<PermissionsQueryDto>>>,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl Default for RolesDetailQueryDto {
	fn default() -> Self {
		Self {
			id: Uuid::new_v4(),
			name: String::new(),
			permissions: None,
			is_deleted: false,
			created_at: None,
			updated_at: None,
		}
	}
}

impl std::fmt::Display for RolesDetailQueryDto {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.id)
	}
}

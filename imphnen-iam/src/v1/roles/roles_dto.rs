use crate::{PermissionsItemDto, PermissionsQueryDto};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;
use validator::Validate;

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

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesDetailItemDto {
	pub id: String,
	pub name: String,
	pub is_deleted: bool,
	pub permissions: Vec<PermissionsItemDto>,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl RolesDetailItemDto {
	pub fn from(dto: &RolesDetailQueryDto) -> Self {
		Self {
			id: dto.id.id.to_raw(),
			name: dto.name.clone(),
			is_deleted: dto.is_deleted,
			permissions: dto
				.permissions
				.iter()
				.map(PermissionsItemDto::from)
				.collect(),
			created_at: dto.created_at.clone(),
			updated_at: dto.updated_at.clone(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolesDetailQueryDto {
	pub id: Thing,
	pub name: String,
	pub permissions: Vec<PermissionsQueryDto>,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

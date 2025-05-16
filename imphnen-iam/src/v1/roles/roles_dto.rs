use crate::{PermissionsItemDto, PermissionsQueryDto};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct RolesRequestDto {
	#[validate(length(min = 1, message = "Role name must not be empty"))]
	pub name: String,
	pub permissions: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesListItemDto {
	pub id: String,
	pub name: String,
	pub permissions_count: u64,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesDetailItemDto {
	pub id: String,
	pub name: String,
	pub permissions: Vec<PermissionsItemDto>,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
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

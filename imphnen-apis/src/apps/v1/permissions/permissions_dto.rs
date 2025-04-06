use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct PermissionsRequestDto {
	#[validate(length(min = 1, message = "Permission name must not be empty"))]
	pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PermissionsItemDto {
	pub id: String,
	pub name: String,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PermissionsItemDtoRaw {
	pub id: Thing,
	pub name: String,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

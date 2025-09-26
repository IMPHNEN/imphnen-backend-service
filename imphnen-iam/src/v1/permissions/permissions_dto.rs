use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct PermissionsRequestDto {
	#[validate(length(min = 1, message = "Permission name must not be empty"))]
	pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct PermissionsUpdateRequestDto {
	#[validate(length(min = 1, message = "Permission name must not be empty"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}

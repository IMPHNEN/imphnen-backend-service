use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProfileRequest {
	pub name: Option<String>,
	pub email: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateRoleRequest {
	pub role: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
	pub id: String,
	pub email: String,
	pub name: String,
	pub role: String,
	pub provider: String,
}

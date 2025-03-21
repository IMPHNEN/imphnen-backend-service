use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersItemDto {
	pub email: String,
	pub fullname: String,
	pub is_active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequestDto {
	pub email: String,
	pub password: String,
	pub fullname: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateRequestDto {
	pub email: String,
	pub fullname: String,
	pub old_email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateRequest {
	pub email: String,
	pub fullname: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteRequestDto {
	pub email: String,
}
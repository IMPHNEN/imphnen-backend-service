use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersItemDto {
	pub email: String,
	pub fullname: String,
	pub is_active: bool,
}

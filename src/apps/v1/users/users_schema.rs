use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersSchema {
	pub email: String,
	pub fullname: String,
	pub password: String,
	pub is_active: bool,
}

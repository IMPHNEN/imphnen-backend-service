use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct GachaClaimRequestDto {
	#[validate(length(min = 1, message = "User ID must not be empty"))]
	pub user_id: String,

	#[validate(length(min = 1, message = "Item ID must not be empty"))]
	pub item_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaClaimDto {
	pub id: String,
	pub user: String,
	pub item: String,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GachaClaimDtoRaw {
	pub id: Thing,
	pub user: Thing,
	pub item: Thing,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

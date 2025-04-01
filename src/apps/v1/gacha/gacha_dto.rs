use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct GachaItemRequestDto {
	#[validate(length(min = 1, message = "Item name must not be empty"))]
	pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct GachaClaimRequestDto {
	#[validate(length(min = 1, message = "User ID must not be empty"))]
	pub user_id: String,

	#[validate(length(min = 1, message = "Item ID must not be empty"))]
	pub item_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct GachaRollRequestDto {
	#[validate(length(min = 1, message = "Item ID must not be empty"))]
	pub item_id: String,

	#[validate(range(min = 1, message = "Weight must be greater than zero"))]
	pub weight: f32,

	#[validate(range(min = 1, message = "Quantity must be at least 1"))]
	pub quantity: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaItemDto {
	pub id: String,
	pub name: String,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GachaItemDtoRaw {
	pub id: Thing,
	pub name: String,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
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

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaRollDto {
	pub id: String,
	pub item: String,
	pub weight: String,
	pub quantity: i32,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GachaRollDtoRaw {
	pub id: Thing,
	pub item: Thing,
	pub weight: String,
	pub quantity: i32,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

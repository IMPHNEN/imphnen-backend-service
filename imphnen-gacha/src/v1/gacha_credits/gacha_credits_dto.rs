use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct GachaCreditRequestDto {
	#[validate(length(min = 1, message = "User ID must not be empty"))]
	pub user_id: String,

	#[validate(range(
		min = 1,
		message = "Amount must be at least 1 credit"
	))]
	pub amount: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GachaCreditResponseDto {
	pub id: String,
	pub user_id: String,
	pub available_rolls: i32,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl From<&crate::v1::gacha_credits::gacha_credits_schema::GachaCreditSchema> for GachaCreditResponseDto {
	fn from(credit: &crate::v1::gacha_credits::gacha_credits_schema::GachaCreditSchema) -> Self {
		Self {
			id: credit.id.id.to_raw(),
			user_id: credit.user.id.to_raw(),
			available_rolls: credit.available_rolls,
			is_deleted: credit.is_deleted,
			created_at: credit.created_at.clone(),
			updated_at: credit.updated_at.clone(),
		}
	}
}

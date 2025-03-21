use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::v1::UsersItemDto;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaCreateClaimRequestDto {
	pub item_name: String,
	pub transaction_number: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaCreateItemRequestDto {
	pub item_name: String,
	pub item_image: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaCreateRollRequestDto {
	pub item_name: String,
	pub weight: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaItemResponseDto {
	pub item_name: String,
	pub item_image: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaClaimResponseDto {
	pub transaction_number: String,
	pub user: UsersItemDto,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaRollResponseDto {
	pub item: GachaItemResponseDto,
}

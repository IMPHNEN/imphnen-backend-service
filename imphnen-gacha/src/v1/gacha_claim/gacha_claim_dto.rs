use crate::{GachaItemDto, GachaItemSchema};
use imphnen_iam::{UsersDetailItemDto, UsersDetailQueryDto};
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
pub struct GachaClaimItemDto {
	pub id: String,
	pub user: UsersDetailItemDto,
	pub item: GachaItemDto,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GachaClaimQueryDto {
	pub id: Thing,
	pub user: UsersDetailQueryDto,
	pub item: GachaItemSchema,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl GachaClaimItemDto {
	pub fn from(dto: &GachaClaimQueryDto) -> Self {
		Self {
			id: dto.id.id.to_raw(),
			user: UsersDetailItemDto::from(&dto.user),
			item: GachaItemDto::from(dto.item.clone()),
			is_deleted: dto.is_deleted,
			created_at: dto.created_at.clone(),
			updated_at: dto.updated_at.clone(),
		}
	}
}

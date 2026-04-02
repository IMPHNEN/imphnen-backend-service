use crate::gacha_claims::domain::gacha_claim::GachaClaimDetail;
use crate::gacha_items::infrastructure::http::dto::GachaItemDto;
use imphnen_iam::users::infrastructure::http::dto::UsersDetailItemDto;
use imphnen_libs::ZodValidate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaClaimCreateRequestDto {
	pub user_id: String,
	pub item_id: String,
}

impl ZodValidate for GachaClaimCreateRequestDto {
	fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
		serde_json::from_value(value.clone()).map_err(|e| e.to_string())
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaClaimDetailDto {
	pub id: String,
	pub user: UsersDetailItemDto,
	pub item: GachaItemDto,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

impl From<GachaClaimDetail> for GachaClaimDetailDto {
	fn from(detail: GachaClaimDetail) -> Self {
		GachaClaimDetailDto {
			id: detail.id.to_string(),
			user: UsersDetailItemDto::from(&detail.user),
			item: GachaItemDto::from(detail.item),
			is_deleted: detail.is_deleted,
			created_at: detail.created_at.to_rfc3339(),
			updated_at: detail.updated_at.to_rfc3339(),
		}
	}
}

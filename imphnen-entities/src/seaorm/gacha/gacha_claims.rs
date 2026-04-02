use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_gacha_claims")]
pub struct Model {
	#[sea_orm(primary_key, default = "gen_random_uuid()", auto_increment = false)]
	pub id: Uuid,

	#[sea_orm(not_null)]
	pub user_id: Uuid,

	#[sea_orm(not_null)]
	pub gacha_item_id: Uuid,

	#[sea_orm(not_null)]
	pub claim_id: Uuid,

	#[sea_orm(not_null)]
	pub claim_type: String,

	#[sea_orm(not_null)]
	pub status: String,

	#[sea_orm(default = "0")]
	pub quantity: i32,

	#[sea_orm(type = "jsonb", nullable)]
	pub metadata: Option<serde_json::Value>,

	#[sea_orm(not_null, default = "now()")]
	pub claimed_at: DateTime<Utc>,

	#[sea_orm(not_null, default = "now()")]
	pub created_at: DateTime<Utc>,

	#[sea_orm(not_null, default = "now()")]
	pub updated_at: DateTime<Utc>,

	#[sea_orm(nullable)]
	pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, sea_orm::EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Default, Serialize, Deserialize)]
pub struct GachaClaimBuilder {
	user_id: Option<Uuid>,
	gacha_item_id: Option<Uuid>,
	claim_type: Option<String>,
	status: Option<String>,
	quantity: Option<i32>,
	metadata: Option<serde_json::Value>,
}

impl GachaClaimBuilder {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	#[must_use]
	pub fn user_id(mut self, user_id: Uuid) -> Self {
		self.user_id = Some(user_id);
		self
	}

	#[must_use]
	pub fn gacha_item_id(mut self, gacha_item_id: Uuid) -> Self {
		self.gacha_item_id = Some(gacha_item_id);
		self
	}

	#[must_use]
	pub fn claim_type(mut self, claim_type: String) -> Self {
		self.claim_type = Some(claim_type);
		self
	}

	#[must_use]
	pub fn status(mut self, status: String) -> Self {
		self.status = Some(status);
		self
	}

	#[must_use]
	pub fn quantity(mut self, quantity: i32) -> Self {
		self.quantity = Some(quantity);
		self
	}

	#[must_use]
	pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
		self.metadata = Some(metadata);
		self
	}

	pub fn build(self) -> Result<ActiveModel, String> {
		let mut active_model = <ActiveModel as std::default::Default>::default();

		if let Some(user_id) = self.user_id {
			active_model.user_id = Set(user_id);
		} else {
			return Err("User ID is required".to_string());
		}

		if let Some(gacha_item_id) = self.gacha_item_id {
			active_model.gacha_item_id = Set(gacha_item_id);
		} else {
			return Err("Gacha Item ID is required".to_string());
		}

		if let Some(claim_type) = self.claim_type {
			active_model.claim_type = Set(claim_type);
		} else {
			return Err("Claim type is required".to_string());
		}

		if let Some(status) = self.status {
			active_model.status = Set(status);
		} else {
			return Err("Status is required".to_string());
		}

		if let Some(quantity) = self.quantity {
			active_model.quantity = Set(quantity);
		}

		if let Some(metadata) = self.metadata {
			active_model.metadata = Set(Some(metadata));
		}

		Ok(active_model)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::seaorm::common::utils::generate_uuid;

	#[test]
	fn test_gacha_claim_model_creation() {
		let user_id = generate_uuid();
		let gacha_item_id = generate_uuid();

		let claim = GachaClaimBuilder::new()
			.user_id(user_id)
			.gacha_item_id(gacha_item_id)
			.claim_type("direct".to_string())
			.status("claimed".to_string())
			.quantity(1)
			.build();

		assert!(claim.is_ok());
		let claim_model = claim.unwrap();
		assert_eq!(claim_model.user_id, Set(user_id));
		assert_eq!(claim_model.gacha_item_id, Set(gacha_item_id));
		assert_eq!(claim_model.claim_type, Set("direct".to_string()));
		assert_eq!(claim_model.status, Set("claimed".to_string()));
		assert_eq!(claim_model.quantity, Set(1));
	}
}

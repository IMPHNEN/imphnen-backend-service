use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_gacha_items")]
pub struct Model {
	#[sea_orm(primary_key, default = "gen_random_uuid()", auto_increment = false)]
	pub id: Uuid,

	#[sea_orm(unique, not_null)]
	pub item_code: String,

	#[sea_orm(not_null)]
	pub name: String,

	#[sea_orm(not_null)]
	pub description: String,

	#[sea_orm(not_null)]
	pub rarity: String,

	#[sea_orm(not_null)]
	pub type_: String,

	#[sea_orm(not_null)]
	pub category: String,

	#[sea_orm(not_null)]
	pub value: i32,

	#[sea_orm(not_null)]
	pub weight: f64,

	#[sea_orm(default = "0")]
	pub stock: i32,

	#[sea_orm(default = "false")]
	pub is_limited: bool,

	#[sea_orm(type = "jsonb", nullable)]
	pub metadata: Option<serde_json::Value>,

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
pub struct GachaItemBuilder {
	pub item_code: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub rarity: Option<String>,
	pub type_: Option<String>,
	pub category: Option<String>,
	pub value: Option<i32>,
	pub weight: Option<f64>,
	pub stock: Option<i32>,
	pub is_limited: Option<bool>,
	pub metadata: Option<serde_json::Value>,
}

impl GachaItemBuilder {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	#[must_use]
	pub fn item_code(mut self, item_code: String) -> Self {
		self.item_code = Some(item_code);
		self
	}

	#[must_use]
	pub fn name(mut self, name: String) -> Self {
		self.name = Some(name);
		self
	}

	#[must_use]
	pub fn description(mut self, description: String) -> Self {
		self.description = Some(description);
		self
	}

	#[must_use]
	pub fn rarity(mut self, rarity: String) -> Self {
		self.rarity = Some(rarity);
		self
	}

	#[must_use]
	pub fn type_(mut self, type_: String) -> Self {
		self.type_ = Some(type_);
		self
	}

	#[must_use]
	pub fn category(mut self, category: String) -> Self {
		self.category = Some(category);
		self
	}

	#[must_use]
	pub fn value(mut self, value: i32) -> Self {
		self.value = Some(value);
		self
	}

	#[must_use]
	pub fn weight(mut self, weight: f64) -> Self {
		self.weight = Some(weight);
		self
	}

	#[must_use]
	pub fn stock(mut self, stock: i32) -> Self {
		self.stock = Some(stock);
		self
	}

	#[must_use]
	pub fn is_limited(mut self, is_limited: bool) -> Self {
		self.is_limited = Some(is_limited);
		self
	}

	#[must_use]
	pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
		self.metadata = Some(metadata);
		self
	}
}

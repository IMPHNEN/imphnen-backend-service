use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GachaClaimSchema {
	pub transaction_number: String,
	pub item: Thing,
	pub user: Thing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GachaRollSchema {
	pub weight: String,
	pub item: Thing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GachaItemSchema {
	pub item_image: String,
	pub item_name: String,
}

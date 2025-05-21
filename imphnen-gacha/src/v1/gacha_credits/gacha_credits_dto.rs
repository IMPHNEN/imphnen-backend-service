use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GachaCreditRequestDto {
	pub user_id: String,
	pub amount: i32,
}

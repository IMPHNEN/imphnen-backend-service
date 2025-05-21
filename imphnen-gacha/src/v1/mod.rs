use axum::Router;

pub mod gacha_claims;
pub mod gacha_credits;
pub mod gacha_items;
pub mod gacha_rolls;

pub use gacha_claims::*;
pub use gacha_credits::*;
pub use gacha_items::*;
pub use gacha_rolls::*;

pub fn gacha_router() -> Router {
	Router::new()
		.nest("/gacha/claims", gacha_claim_router())
		.nest("/gacha/items", gacha_item_router())
		.nest("/gacha/rolls", gacha_roll_router())
}

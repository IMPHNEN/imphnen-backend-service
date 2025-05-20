use axum::Router;

pub mod gacha_claim;
pub mod gacha_item;
pub mod gacha_roll;

pub use gacha_claim::*;
pub use gacha_item::*;
pub use gacha_roll::*;

pub fn gacha_router() -> Router {
	Router::new()
		.nest("/gacha/claims", gacha_claim_router())
		.nest("/gacha/items", gacha_item_router())
		.nest("/gacha/rolls", gacha_roll_router())
}

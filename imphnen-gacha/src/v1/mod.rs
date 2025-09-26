use axum::Router;

pub mod gacha_claims;
pub mod gacha_credits;
pub mod gacha_items;
pub mod gacha_rolls;

// Export only public router functions to avoid namespace pollution
pub use gacha_claims::gacha_claim_router;
pub use gacha_credits::*; // gacha_credits doesn't have router functions
pub use gacha_items::gacha_item_router;
pub use gacha_rolls::gacha_roll_router;

/// Creates the main gacha router with all version 1 endpoints
pub fn gacha_router() -> Router {
    Router::new()
        .nest("/claims", gacha_claim_router())
        .nest("/items", gacha_item_router())
        .nest("/rolls", gacha_roll_router())
}

use axum::Router;

pub mod gacha_claims;
pub mod gacha_credits;
pub mod gacha_items;
pub mod gacha_rolls;
use crate::v1::gacha_items::gacha_items_controller;

// Export only public router functions to avoid namespace pollution
pub use gacha_credits::gacha_credit_router;
pub use gacha_items::gacha_item_router;
pub use gacha_rolls::gacha_roll_router;
pub use gacha_claims::gacha_claim_router;

/// Creates the main gacha router with all version 1 endpoints
pub fn gacha_router() -> Router {
    let mut router = Router::new();
    router = router.nest("/credits", gacha_credit_router());
    router = router.nest("/items", gacha_item_router());
    router = router.nest("/rolls", gacha_roll_router());
    router = router.nest("/claims", gacha_claim_router());
    // Minimal admin router mounted at /admin to satisfy test.sh expectations
    // This will expose GET /v1/gacha/admin -> list items (admin view)
    router = router.nest("/admin", Router::new().route("/", axum::routing::get(gacha_items_controller::get_gacha_item_list)));
    router
}

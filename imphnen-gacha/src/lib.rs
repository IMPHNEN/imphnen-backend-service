pub mod gacha_claims;
pub mod gacha_credits;
pub mod gacha_items;
pub mod gacha_rolls;

pub use imphnen_entities::{
	MessageResponseDto, PermissionsEnum, ResponseListSuccessDto, ResponseSuccessDto,
};
pub use imphnen_libs::AppState;

use axum::Router;
use gacha_claims::gacha_claim_router;
use gacha_credits::gacha_credit_router;
use gacha_items::gacha_item_router;
use gacha_rolls::gacha_roll_router;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub fn gacha_router(db: DatabaseConnection, state: Arc<AppState>) -> Router {
	let mut router = Router::new();
	router = router.nest("/credits", gacha_credit_router(db.clone()));
	router = router.nest("/items", gacha_item_router(db.clone()));
	router = router.nest("/rolls", gacha_roll_router(db.clone(), state.clone()));
	router = router.nest("/claims", gacha_claim_router(db.clone(), state));
	router = router.nest(
		"/admin",
		Router::new()
			.route(
				"/",
				axum::routing::get(
					gacha_items::infrastructure::http::handlers::get_gacha_item_list,
				),
			)
			.layer(axum::Extension(Arc::new(
				gacha_items::application::GachaItemServiceImpl::new(Arc::new(
					gacha_items::infrastructure::persistence::PostgresGachaItemRepository::new(
						db,
					),
				)),
			)
				as Arc<dyn gacha_items::domain::GachaItemService>)),
	);
	router
}

use super::handlers::{
	delete_gacha_item, get_gacha_item_by_id, get_gacha_item_list,
	post_create_gacha_item, put_update_gacha_item,
};
use crate::gacha_items::application::GachaItemServiceImpl;
use crate::gacha_items::domain::GachaItemService;
use crate::gacha_items::infrastructure::persistence::PostgresGachaItemRepository;
use axum::{
	Extension, Router,
	routing::{delete, get, post, put},
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

fn build_service(db: DatabaseConnection) -> Arc<dyn GachaItemService> {
	let repo = Arc::new(PostgresGachaItemRepository::new(db));
	Arc::new(GachaItemServiceImpl::new(repo))
}

pub fn gacha_item_router(db: DatabaseConnection) -> Router {
	let service = build_service(db);
	Router::new()
		.route("/", get(get_gacha_item_list))
		.route("/detail/{id}", get(get_gacha_item_by_id))
		.route("/create", post(post_create_gacha_item))
		.route("/update/{id}", put(put_update_gacha_item))
		.route("/delete/{id}", delete(delete_gacha_item))
		.layer(Extension(service))
}

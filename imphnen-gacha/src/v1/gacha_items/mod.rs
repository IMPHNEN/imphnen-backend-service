use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub mod gacha_items_controller;
pub mod gacha_items_dto;
pub mod gacha_items_repository;
pub mod gacha_items_schema;
pub mod gacha_items_service;

// Export only public API functions and types
pub use gacha_items_controller::{
    get_gacha_item_list,
    post_create_gacha_item,
    get_gacha_item_by_id,
    put_update_gacha_item,
    delete_gacha_item,
};
pub use gacha_items_dto::GachaItemDto;

/// Creates router for gacha items endpoints
pub fn gacha_item_router() -> Router {
    Router::new()
        .route("/", get(get_gacha_item_list))
        .route("/create", post(post_create_gacha_item))
        .route("/detail/{id}", get(get_gacha_item_by_id))
        .route("/update/{id}", put(put_update_gacha_item))
        .route("/delete/{id}", delete(delete_gacha_item))
}

use axum::{
    Router,
    routing::{get, post},
};

pub mod gacha_rolls_controller;
pub mod gacha_rolls_dto;
pub mod gacha_rolls_repository;
pub mod gacha_rolls_schema;
pub mod gacha_rolls_service;

// Export only public API functions and types
pub use gacha_rolls_controller::{
    post_create_gacha_roll,
    post_execute_gacha_roll,
    get_detail_gacha_roll,
};
pub use gacha_rolls_dto::GachaRollItemDto;

/// Creates router for gacha rolls endpoints
pub fn gacha_roll_router() -> Router {
    Router::new()
        .route("/create", post(post_create_gacha_roll))
        .route("/execute", post(post_execute_gacha_roll))
        .route("/detail/{id}", get(get_detail_gacha_roll))
}

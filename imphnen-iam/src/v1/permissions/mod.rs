use axum::{
	Router,
	routing::{delete, get, post, put},
};
pub mod permissions_controller;
pub mod permissions_dto;
pub mod permissions_enum;
pub mod permissions_guard;
pub mod permissions_repository;
pub mod permissions_schema;
pub mod permissions_service;

// Export only essential types and functions from each submodule
pub use permissions_controller::{
    get_permission_list,
    get_permission_by_id,
    post_create_permission,
    put_update_permission,
    delete_permission
};

pub use permissions_dto::{
    PermissionsRequestDto,
    PermissionsUpdateRequestDto,
};

pub use permissions_enum::PermissionsEnum;
pub use permissions_guard::permissions_guard;
pub use permissions_repository::PermissionsRepository;
pub use permissions_schema::PermissionsSchema;

pub fn permissions_router() -> Router {
	Router::new()
		.route("/", get(get_permission_list))
		.route("/create", post(post_create_permission))
		.route("/detail/{id}", get(get_permission_by_id))
		.route("/update/{id}", put(put_update_permission))
		.route("/delete/{id}", delete(delete_permission))
}

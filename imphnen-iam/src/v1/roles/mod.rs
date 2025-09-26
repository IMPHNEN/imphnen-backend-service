use axum::{
	Router,
	routing::{delete, get, post, put},
};

pub mod roles_controller;
pub mod roles_dto;
pub mod roles_enum;
pub mod roles_repository;
pub mod roles_schema;
pub mod roles_service;

// Export only essential types and functions from each submodule
pub use roles_controller::{
    get_role_list,
    get_role_by_id,
    post_create_role,
    put_update_role,
    delete_role
};

pub use roles_dto::{
    RolesRequestCreateDto,
    RolesRequestUpdateDto,
    RolesDetailItemDto,
    RolesListItemDto,
    RolesDetailQueryDto,
};

pub use roles_enum::RolesEnum;
pub use roles_repository::RolesRepository;
pub use roles_schema::RolesSchema;

pub fn roles_router() -> Router {
	Router::new()
		.route("/", get(get_role_list))
		.route("/detail/{id}", get(get_role_by_id))
		.route("/create", post(post_create_role))
		.route("/update/{id}", put(put_update_role))
		.route("/delete/{id}", delete(delete_role))
}

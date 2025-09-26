use axum::{
	Router,
	routing::{delete, get, post, put},
};

pub mod users_controller;
pub mod users_dto;
pub mod users_repository;
pub mod users_schema;
pub mod users_service;

// Export only essential types and functions from each submodule
pub use users_controller::{
    get_user_list,
    get_user_by_id,
    get_user_me,
    post_create_user,
    put_update_user,
    put_update_user_me,
    delete_user,
    patch_user_active_status,
    upload_file
};

pub use users_dto::{
    UsersActiveInactiveRequestDto,
    UsersCreateRequestDto,
    UsersUpdateRequestDto,
    UsersSetNewPasswordRequestDto,
    UsersDetailItemDto,
    UsersListItemDto,
    UsersListQueryDto,
};

pub use users_repository::UsersRepository;
pub use users_schema::UsersSchema;

pub fn users_router() -> Router {
	Router::new()
		.route("/", get(get_user_list))
		.route("/activate/{id}", put(patch_user_active_status))
		.route("/create", post(post_create_user))
		.route("/me", get(get_user_me))
		.route("/delete/{id}", delete(delete_user))
		.route("/detail/{id}", get(get_user_by_id))
		.route("/update/{id}", put(put_update_user))
		.route("/update/me", put(put_update_user_me))
		.route("/upload", post(upload_file))
}

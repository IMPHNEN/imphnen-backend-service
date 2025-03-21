use axum::{http::StatusCode, response::Response};

use super::{
	user_repository::UserRepository, CreateUserRequestDto, DeleteRequestDto,
	UpdateRequestDto,
};
use crate::{
	common_response, hash_password, success_list_response, success_response, v1::UsersItemDto, AppState, ResponseListSuccessDto, ResponseSuccessDto
};

pub struct UserService;

impl UserService {
	pub async fn mutation_create_user(
		payload: CreateUserRequestDto,
		state: &AppState,
	) -> Response {
		let repository = UserRepository::new(state);
		if repository.query_user_by_email(&payload.email).await.is_ok() {
			return common_response(StatusCode::BAD_REQUEST, "User already exists");
		}

		let hashed_password = match hash_password(&payload.password) {
			Ok(hash) => hash,
			Err(_) => {
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Failed to hash password",
				);
			}
		};

		let new_user = CreateUserRequestDto {
			email: payload.email,
			password: hashed_password,
			fullname: payload.fullname,
		};

		match repository.query_create_user(new_user).await {
			Ok(_) => common_response(StatusCode::CREATED, "Create user is successful"),
			Err(err) => {
				common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
			}
		}
	}

	pub async fn read_all_user(state: &AppState) -> Response {
		let repository = UserRepository::new(state);
		match repository.query_all_user().await {
			Ok(user) => {
				let response: ResponseSuccessDto<Vec<UsersItemDto>> = ResponseSuccessDto {
					data: user
						.iter()
						.map(|v| UsersItemDto {
							fullname: v.fullname.clone(),
							email: v.email.clone(),
							is_active: v.is_active,
						})
						.collect(),
				};
				success_list_response(ResponseListSuccessDto { data: response, meta: None })
			}
			Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
		}
	}

	pub async fn read_user_by_email(email: &str, state: &AppState) -> Response {
		let repository = UserRepository::new(state);
		match repository.query_user_by_email(email).await {
			Ok(user) => success_response(ResponseSuccessDto { data: user }),
			Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
		}
	}

	pub async fn mutation_update_user(
		payload: UpdateRequestDto,
		state: &AppState,
	) -> Response {
		let repository = UserRepository::new(state);
		let user = UpdateRequestDto {
			email: payload.email,
			fullname: payload.fullname,
			old_email: payload.old_email,
		};

		match repository.query_update_user(user).await {
			Ok(_) => common_response(StatusCode::CREATED, "Update user is successful"),
			Err(err) => {
				common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
			}
		}
	}

	pub async fn mutation_delete_user(
		payload: DeleteRequestDto,
		state: &AppState,
	) -> Response {
		let repository = UserRepository::new(state);
		match repository.query_delete_user(&payload.email).await {
			Ok(_) => common_response(StatusCode::CREATED, "Delete user is successful"),
			Err(err) => {
				common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
			}
		}
	}
}

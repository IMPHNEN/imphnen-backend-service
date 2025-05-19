use super::{
	UsersActiveInactiveRequestDto, UsersCreateRequestDto, UsersDetailItemDto,
	UsersSetNewPasswordRequestDto, UsersUpdateRequestDto,
};
use crate::{
	AppState, MetaRequestDto, ResponseListSuccessDto, UsersRepository, UsersSchema,
};
use crate::{
	ResourceEnum, ResponseSuccessDto, common_response, extract_email, make_thing,
	success_list_response, success_response, validate_request,
};
use axum::http::HeaderMap;
use axum::{http::StatusCode, response::Response};
use imphnen_libs::{hash_password, verify_password};

pub struct UsersService;

impl UsersService {
	pub async fn get_user_list(state: &AppState, meta: MetaRequestDto) -> Response {
		let repo = UsersRepository::new(state);
		match repo.query_user_list(meta).await {
			Ok(data) => {
				let response = ResponseListSuccessDto {
					data: data.data,
					meta: data.meta,
				};
				success_list_response(response)
			}
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn get_user_by_id(state: &AppState, id: String) -> Response {
		let repo = UsersRepository::new(state);
		match repo.query_user_by_id(id).await {
			Ok(user) if !user.is_deleted => success_response(ResponseSuccessDto {
				data: UsersDetailItemDto::from(&user),
			}),
			Ok(_) => common_response(StatusCode::NOT_FOUND, "User not found"),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn get_user_me(headers: HeaderMap, state: &AppState) -> Response {
		let repo = UsersRepository::new(state);
		let email = match extract_email(&headers) {
			Some(email) => email,
			None => return common_response(StatusCode::UNAUTHORIZED, "Invalid token"),
		};
		match repo.query_user_by_email(email).await {
			Ok(user) if !user.is_deleted => success_response(ResponseSuccessDto {
				data: UsersDetailItemDto::from(&user),
			}),
			Ok(_) => common_response(StatusCode::NOT_FOUND, "User not found"),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn create_user(
		state: &AppState,
		new_user: UsersCreateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&new_user) {
			return common_response(status, &message);
		}
		let repo = UsersRepository::new(state);
		if repo
			.query_user_by_email(new_user.email.clone())
			.await
			.is_ok()
		{
			return common_response(StatusCode::BAD_REQUEST, "User already exists");
		}
		match repo.query_create_user(UsersSchema::create(new_user)).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(err) => {
				common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
			}
		}
	}

	pub async fn update_user(
		state: &AppState,
		id: String,
		user: UsersUpdateRequestDto,
	) -> Response {
		let repo = UsersRepository::new(state);
		if let Err((status, message)) = validate_request(&user) {
			return common_response(status, &message);
		}
		let updated_user = UsersSchema::update(user, id);
		match repo.query_update_user(updated_user).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn update_user_me(
		state: &AppState,
		headers: HeaderMap,
		user: UsersUpdateRequestDto,
	) -> Response {
		let repo = UsersRepository::new(state);
		let email = match extract_email(&headers) {
			Some(email) => email,
			None => return common_response(StatusCode::UNAUTHORIZED, "Unauthorized"),
		};
		let user_data = match repo.query_user_by_email(email.clone()).await {
			Ok(user) => user,
			Err(_) => return common_response(StatusCode::NOT_FOUND, "User not found"),
		};
		if let Err((status, message)) = validate_request(&user) {
			return common_response(status, &message);
		}
		let updated_user = UsersSchema::update(user, user_data.id.id.to_raw());
		match repo.query_update_user(updated_user).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn set_user_active_status(
		state: &AppState,
		id: String,
		payload: UsersActiveInactiveRequestDto,
	) -> Response {
		let repo = UsersRepository::new(state);
		let thing_id = make_thing(&ResourceEnum::Users.to_string(), &id);
		match repo.query_user_by_id(thing_id.id.to_raw()).await {
			Ok(user) if !user.is_deleted => {
				let patch = UsersSchema {
					id: user.id.clone(),
					is_active: payload.is_active,
					..UsersSchema::from(user)
				};
				match repo.query_update_user(patch).await {
					Ok(msg) => common_response(StatusCode::OK, &msg),
					Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
				}
			}
			Ok(_) => common_response(StatusCode::NOT_FOUND, "User not found"),
			Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
		}
	}

	pub async fn update_user_password(
		state: &AppState,
		email: String,
		payload: UsersSetNewPasswordRequestDto,
	) -> Response {
		let repo = UsersRepository::new(state);
		let user = match repo.query_user_by_email(email.clone()).await {
			Ok(user) if !user.is_deleted => user,
			_ => return common_response(StatusCode::NOT_FOUND, "User not found"),
		};
		let verify_result = match verify_password(&payload.old_password, &user.password)
		{
			Ok(result) => result,
			Err(_) => {
				return common_response(
					StatusCode::BAD_REQUEST,
					"Old password is incorrect",
				);
			}
		};
		if !verify_result {
			return common_response(StatusCode::BAD_REQUEST, "Old password is incorrect");
		}
		let new_password = match hash_password(&payload.password) {
			Ok(pw) => pw,
			Err(_) => {
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Failed to hash password",
				);
			}
		};
		let patch = UsersSchema {
			id: user.id.clone(),
			password: new_password,
			..Default::default()
		};
		match repo.query_update_user(patch).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn delete_user(state: &AppState, id: String) -> Response {
		let repo = UsersRepository::new(state);
		if repo.query_user_by_id(id.clone()).await.is_err() {
			return common_response(StatusCode::BAD_REQUEST, "User not found");
		}
		match repo.query_delete_user(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}
}

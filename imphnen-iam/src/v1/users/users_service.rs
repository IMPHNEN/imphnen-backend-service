use crate::{
	common_response, extract_email, get_iso_date, hash_password, make_thing,
	success_list_response, success_response, validate_request, ResourceEnum,
	ResponseSuccessDto,
};
use crate::{
	AppState, MetaRequestDto, ResponseListSuccessDto, UsersActiveInactiveSchema,
	UsersRepository, UsersSchema, UsersSetNewPasswordSchema,
};
use axum::http::HeaderMap;
use axum::{http::StatusCode, response::Response};

use super::{
	UsersActiveInactiveRequestDto, UsersCreateRequestDto, UsersDetailItemDto,
	UsersUpdateRequestDto,
};

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
				data: UsersDetailItemDto {
					id: user.id,
					role: user.role,
					fullname: user.fullname,
					email: user.email,
					avatar: user.avatar,
					phone_number: user.phone_number,
					is_active: user.is_active,
					gender: user.gender,
					birthdate: user.birthdate,
				},
			}),
			Ok(_) => common_response(StatusCode::NOT_FOUND, "User not found"),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn get_user_me(headers: HeaderMap, state: &AppState) -> Response {
		let repo = UsersRepository::new(state);
		let email = extract_email(&headers).unwrap();
		let user = repo.query_user_by_email(email).await.unwrap();
		match repo.query_user_by_id(user.id.id.to_raw()).await {
			Ok(user) => success_response(ResponseSuccessDto {
				data: UsersDetailItemDto {
					id: user.id,
					role: user.role,
					fullname: user.fullname,
					email: user.email,
					avatar: user.avatar,
					phone_number: user.phone_number,
					is_active: user.is_active,
					gender: user.gender,
					birthdate: user.birthdate,
				},
			}),
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
		let role_thing = make_thing(&ResourceEnum::Roles.to_string(), &new_user.role_id);
		match repo
			.query_create_user(UsersSchema {
				email: new_user.email.clone(),
				fullname: new_user.fullname.clone(),
				password: hash_password(&new_user.password).unwrap(),
				phone_number: new_user.phone_number.clone(),
				is_active: new_user.is_active.clone(),
				role: role_thing,
				..Default::default()
			})
			.await
		{
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

		let user_id = make_thing(&ResourceEnum::Users.to_string(), &id);
		let role_id = make_thing(&ResourceEnum::Roles.to_string(), "");

		let updated_user = UsersSchema {
			id: user_id,
			fullname: user.fullname,
			email: user.email,
			phone_number: user.phone_number,
			is_active: user.is_active,
			gender: user.gender,
			birthdate: user.birthdate,
			avatar: user.avatar,
			role: role_id,
			updated_at: get_iso_date(),
			..Default::default()
		};

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
		let email = extract_email(&headers).unwrap();
		let user_data = repo.query_user_by_email(email).await.unwrap();
		if let Err((status, message)) = validate_request(&user) {
			return common_response(status, &message);
		}
		let user_id =
			make_thing(&ResourceEnum::Users.to_string(), &user_data.id.id.to_raw());
		let role_id = make_thing(&ResourceEnum::Roles.to_string(), "");
		let updated_user = UsersSchema {
			id: user_id,
			fullname: user.fullname,
			email: user.email,
			phone_number: user.phone_number,

			is_active: user.is_active,

			gender: user.gender,
			birthdate: user.birthdate,
			avatar: user.avatar,

			role: role_id,
			updated_at: get_iso_date(),
			..Default::default()
		};
		match repo.query_update_user(updated_user).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn set_user_active_status(
		state: &AppState,
		id: String,
		status: UsersActiveInactiveRequestDto,
	) -> Response {
		let repo = UsersRepository::new(state);
		let thing_id = make_thing(&ResourceEnum::Users.to_string(), &id);
		match repo.query_user_by_id(thing_id.id.to_raw()).await {
			Ok(_) => match repo
				.query_active_inactive_user_by_id(
					id,
					UsersActiveInactiveSchema {
						is_active: status.is_active,
					},
				)
				.await
			{
				Ok(msg) => common_response(StatusCode::OK, &msg),
				Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
			},
			Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
		}
	}

	pub async fn update_user_password(
		state: &AppState,
		email: String,
		new_password: UsersSetNewPasswordSchema,
	) -> Response {
		let repo = UsersRepository::new(state);
		match repo.query_update_password_user(email, new_password).await {
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

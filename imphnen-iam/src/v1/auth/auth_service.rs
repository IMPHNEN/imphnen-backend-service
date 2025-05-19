use super::{
	AuthLoginRequestDto, AuthLoginResponsetDto, AuthNewPasswordRequestDto,
	AuthRefreshTokenRequestDto, AuthRegisterRequestDto, AuthRepository,
	AuthResendOtpRequestDto, AuthVerifyEmailRequestDto, TokenDto,
};
use crate::{
	AppState, Env, ResourceEnum, ResponseSuccessDto, RolesEnum, RolesRepository,
	UsersDetailItemDto, UsersRepository, UsersSchema, common_response,
	decode_refresh_token, encode_access_token, encode_refresh_token,
	encode_reset_password_token, extract_email_token, generate_otp, get_iso_date,
	hash_password, make_thing, send_email, success_response, validate_request,
	verify_password,
};
use axum::{http::StatusCode, response::Response};
use surrealdb::Uuid;

pub struct AuthService;

impl AuthService {
	pub async fn mutation_login(
		payload: AuthLoginRequestDto,
		state: &AppState,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}

		let user_repo = UsersRepository::new(state);
		let auth_repo = AuthRepository::new(state);

		match user_repo.query_user_by_email(payload.email.clone()).await {
			Ok(user) => {
				let is_password_correct =
					verify_password(&payload.password, &user.password).unwrap_or(false);

				if !is_password_correct {
					return common_response(
						StatusCode::BAD_REQUEST,
						"Email or password not correct",
					);
				}

				if !user.is_active {
					return common_response(
						StatusCode::BAD_REQUEST,
						"Account not active, please verify your email",
					);
				}

				let access_token = match encode_access_token(payload.email.clone()) {
					Ok(token) => token,
					Err(_) => {
						return common_response(
							StatusCode::INTERNAL_SERVER_ERROR,
							"Failed to generate access token",
						);
					}
				};

				let refresh_token = match encode_refresh_token(payload.email.clone()) {
					Ok(token) => token,
					Err(_) => {
						return common_response(
							StatusCode::INTERNAL_SERVER_ERROR,
							"Failed to generate refresh token",
						);
					}
				};

				let response = ResponseSuccessDto {
					data: AuthLoginResponsetDto {
						user: UsersDetailItemDto::from(&user),
						token: TokenDto {
							access_token,
							refresh_token,
						},
					},
				};

				if let Err(_err) = auth_repo.query_store_user(user).await {
					return common_response(StatusCode::BAD_REQUEST, "User already login");
				}

				success_response(response)
			}
			Err(err) => common_response(StatusCode::UNAUTHORIZED, &err.to_string()),
		}
	}

	pub async fn mutation_register(
		payload: AuthRegisterRequestDto,
		state: &AppState,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}

		let user_repo = UsersRepository::new(state);
		let auth_repo = AuthRepository::new(state);
		let role_repo = RolesRepository::new(state);
		let role = match role_repo
			.query_role_by_name(RolesEnum::User.to_string())
			.await
		{
			Ok(role) => role,
			Err(_) => return common_response(StatusCode::BAD_REQUEST, "Role Not Found"),
		};

		if user_repo
			.query_user_by_email(payload.email.clone())
			.await
			.is_ok()
		{
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

		let new_user = AuthRegisterRequestDto {
			email: payload.email,
			password: hashed_password,
			fullname: payload.fullname,
			phone_number: payload.phone_number,
		};

		let otp = generate_otp::OtpManager::generate_otp();

		match auth_repo
			.query_store_otp(new_user.email.clone(), otp.clone())
			.await
		{
			Ok(_) => {
				let message = format!("your otp code is {}", otp);
				if let Err(err) = send_email(&new_user.email, "OTP Verification", &message) {
					return common_response(
						StatusCode::INTERNAL_SERVER_ERROR,
						&err.to_string(),
					);
				}
			}
			Err(err) => {
				return common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string());
			}
		}

		let role_thing = make_thing(&ResourceEnum::Roles.to_string(), &role.id);
		let user_thing = make_thing(
			&ResourceEnum::Users.to_string(),
			&Uuid::new_v4().to_string(),
		);

		match user_repo
			.query_create_user(UsersSchema {
				id: user_thing,
				email: new_user.email.clone(),
				fullname: new_user.fullname.clone(),
				password: new_user.password.clone(),
				phone_number: new_user.phone_number.clone(),
				created_at: get_iso_date(),
				updated_at: get_iso_date(),
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

	pub async fn mutation_resend_otp(
		payload: AuthResendOtpRequestDto,
		state: &AppState,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}

		let user_repo = UsersRepository::new(state);
		if user_repo
			.query_user_by_email(payload.email.clone())
			.await
			.is_err()
		{
			return common_response(StatusCode::BAD_REQUEST, "User not found");
		}

		let auth_repo = AuthRepository::new(state);
		let _ = auth_repo.query_get_stored_otp(payload.email.clone()).await;

		let otp = generate_otp::OtpManager::generate_otp();
		let message = format!("Your OTP code is {}", otp);
		match auth_repo.query_store_otp(payload.email.clone(), otp).await {
			Ok(_) => match send_email(&payload.email, "OTP Verification", &message) {
				Ok(_) => common_response(StatusCode::OK, "OTP resent successfully"),
				Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
			},
			Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
		}
	}

	pub async fn mutation_refresh_token(
		payload: AuthRefreshTokenRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let email = match decode_refresh_token(&payload.refresh_token) {
			Ok(token) => token.claims.sub,
			Err(_) => {
				return common_response(StatusCode::UNAUTHORIZED, "Invalid refresh token");
			}
		};
		let access_token = match encode_access_token(email.clone()) {
			Ok(token) => token,
			Err(_) => {
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Failed to generate access token",
				);
			}
		};
		let refresh_token = match encode_refresh_token(email.clone()) {
			Ok(token) => token,
			Err(_) => {
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Failed to generate refresh token",
				);
			}
		};
		let response = ResponseSuccessDto {
			data: TokenDto {
				access_token,
				refresh_token,
			},
		};
		success_response(response)
	}

	pub async fn mutation_forgot_password(
		payload: AuthResendOtpRequestDto,
		state: &AppState,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let user_repo = UsersRepository::new(state);
		let user_result = user_repo.query_user_by_email(payload.email.clone()).await;
		let user = match user_result {
			Ok(user) => user,
			Err(err) if err.to_string().contains("User not found") => {
				return common_response(StatusCode::BAD_REQUEST, "User not found");
			}
			Err(err) => {
				return common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string());
			}
		};
		let token = match encode_reset_password_token(user.email) {
			Ok(token) => token,
			Err(_) => {
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Failed to generate access token",
				);
			}
		};
		let env = Env::new();
		let fe_url = env.fe_url;
		let message = format!(
			"You have requested a password reset. Please click the link below to continue: {}/auth/reset-password?token={}",
			fe_url, token
		);
		match send_email(&payload.email, "Reset Password Request", &message) {
			Ok(_) => common_response(StatusCode::OK, "Reset Password request send"),
			Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
		}
	}

	pub async fn mutation_verify_email(
		payload: AuthVerifyEmailRequestDto,
		state: &AppState,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let user_repo = UsersRepository::new(state);
		let auth_repo = AuthRepository::new(state);
		let email = payload.email.clone();
		let user = match user_repo.query_user_by_email(email.clone()).await {
			Ok(user) if !user.is_deleted => user,
			_ => return common_response(StatusCode::NOT_FOUND, "User not found"),
		};
		let patch = UsersSchema {
			id: user.id.clone(),
			is_active: true,
			..UsersSchema::from(user)
		};
		match auth_repo.query_get_stored_otp(email.clone()).await {
			Ok(stored_otp) => match stored_otp == payload.otp {
				true => match user_repo.query_update_user(patch).await {
					Ok(_) => match auth_repo.query_delete_stored_otp(email).await {
						Ok(_) => common_response(StatusCode::OK, "Email verified successfully"),
						Err(e) => {
							common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string())
						}
					},
					Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
				},
				false => match auth_repo.query_delete_stored_otp(email).await {
					Ok(_) => common_response(StatusCode::BAD_REQUEST, "Failed to verify OTP"),
					Err(e) => common_response(
						StatusCode::INTERNAL_SERVER_ERROR,
						&format!("Failed to delete OTP: {}", e),
					),
				},
			},
			Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
		}
	}

	pub async fn mutation_new_password(
		payload: AuthNewPasswordRequestDto,
		state: &AppState,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = UsersRepository::new(state);
		let email = match extract_email_token(payload.token.clone()) {
			Some(email) => email,
			None => {
				return common_response(StatusCode::BAD_REQUEST, "Invalid or missing token");
			}
		};
		let password = match hash_password(&payload.password) {
			Ok(p) => p,
			Err(_) => {
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Failed to hash password",
				);
			}
		};
		let user = match repo.query_user_by_email(email.clone()).await {
			Ok(user) if !user.is_deleted => user,
			_ => return common_response(StatusCode::NOT_FOUND, "User not found"),
		};
		let patch = UsersSchema {
			id: user.id.clone(),
			password,
			..Default::default()
		};
		match repo.query_update_user(patch).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}
}

use axum::{http::StatusCode, response::Response};

use super::{
	AuthActiveInactiveRequestDto, AuthLoginRequestDto, AuthLoginResponsetDto,
	AuthRegisterRequestDto, AuthRepository, AuthResendOtpRequestDto,
	AuthVerifyEmailRequestDto, TokenDto,
};
use crate::{
	common_response, encode_access_token, encode_refresh_token, generate_otp,
	hash_password, send_email, success_response, v1::UsersItemDto, verify_password,
	AppState, Env, ResponseSuccessDto,
};

pub struct AuthService;

impl AuthService {
	pub async fn mutation_login(
		payload: AuthLoginRequestDto,
		state: &AppState,
	) -> Response {
		let repository = AuthRepository::new(state);

		match repository.query_user_by_email(payload.email.clone()).await {
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
						)
					}
				};

				let refresh_token = match encode_refresh_token(payload.email.clone()) {
					Ok(token) => token,
					Err(_) => {
						return common_response(
							StatusCode::INTERNAL_SERVER_ERROR,
							"Failed to generate refresh token",
						)
					}
				};

				let response = ResponseSuccessDto {
					data: AuthLoginResponsetDto {
						user: UsersItemDto {
							fullname: user.fullname.clone(),
							email: user.email.clone(),
							is_active: user.is_active,
						},
						token: TokenDto {
							access_token,
							refresh_token,
						},
					},
				};

				if let Err(_) = repository.query_store_user_data(AuthRegisterRequestDto {
					fullname: user.fullname,
					password: user.password,
					email: user.email,
				}) {
					return common_response(StatusCode::BAD_REQUEST, "Failed to store data");
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
		let repository = AuthRepository::new(state);
		if repository
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
		};

		let otp = generate_otp::OtpManager::generate_otp();

		repository
			.query_store_otp(new_user.email.clone(), otp.clone())
			.unwrap();

		let message = format!("your otp code is {}", otp);

		send_email(&new_user.email.clone(), "OTP Verification", &message).unwrap();

		match repository.query_create_user(new_user).await {
			Ok(_) => common_response(StatusCode::CREATED, "Registration successful"),
			Err(err) => {
				common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
			}
		}
	}

	pub async fn mutation_resend_otp(
		payload: AuthResendOtpRequestDto,
		state: &AppState,
	) -> Response {
		let repository = AuthRepository::new(state);
		let otp = generate_otp::OtpManager::generate_otp();
		let message = format!("Your OTP code is {}", otp);
		match repository.query_store_otp(payload.email.clone(), otp) {
			Ok(_) => match send_email(&payload.email, "OTP Verification", &message) {
				Ok(_) => common_response(StatusCode::OK, "OTP resent successfully"),
				Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
			},
			Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
		}
	}

	pub async fn mutation_forgot_password(
		payload: AuthResendOtpRequestDto,
		state: &AppState,
	) -> Response {
		let repository = AuthRepository::new(state);
		if repository
			.query_user_by_email(payload.email.clone())
			.await
			.is_err()
		{
			return common_response(StatusCode::BAD_REQUEST, "User not found");
		}
		let token = match encode_access_token(payload.email.clone()) {
			Ok(token) => token,
			Err(_) => {
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Failed to generate access token",
				)
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
		let repository = AuthRepository::new(state);
		match repository.query_get_stored_otp(payload.email.clone()) {
			Ok(stored_otp) => {
				let user_otp = payload.otp;
				let is_otp_valid = stored_otp == user_otp;
				if is_otp_valid {
					match repository
						.query_active_inactive_user(AuthActiveInactiveRequestDto {
							email: payload.email.clone(),
							is_active: true,
						})
						.await
					{
						Ok(_) => {
							if let Err(e) =
								repository.query_delete_stored_otp(payload.email.clone())
							{
								return common_response(
									StatusCode::INTERNAL_SERVER_ERROR,
									&format!("Failed to delete OTP: {}", e),
								);
							}
							common_response(StatusCode::OK, "Email verified successfully")
						}
						Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
					}
				} else {
					if let Err(e) = repository.query_delete_stored_otp(payload.email.clone()) {
						return common_response(
							StatusCode::INTERNAL_SERVER_ERROR,
							&format!("Failed to delete OTP: {}", e),
						);
					}
					common_response(StatusCode::BAD_REQUEST, "Failed to verify OTP")
				}
			}
			Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
		}
	}
}

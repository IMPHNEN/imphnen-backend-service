use super::{
	AuthLoginRequestDto, AuthLoginResponsetDto, AuthNewPasswordRequestDto,
	AuthRefreshTokenRequestDto, AuthRegisterRequestDto, AuthRepository,
	AuthResendOtpRequestDto, AuthVerifyEmailRequestDto, TokenDto,
};
use crate::{
	AppState, ResourceEnum, ResponseSuccessDto, RolesEnum, RolesRepository,
	UsersDetailItemDto, UsersRepository, UsersSchema, common_response,
	decode_refresh_token, encode_access_token, encode_refresh_token,
	encode_reset_password_token, extract_email_token, generate_otp, get_iso_date,
	hash_password, make_thing, send_email, success_response, validate_request,
	verify_password, surrealdb_init_ws, surrealdb_init_mem,
};
use axum::{http::StatusCode, response::Response};
use surrealdb::Uuid;
use tracing::error;

use async_trait::async_trait;

#[async_trait]
pub trait AuthServiceTrait: Send + Sync + 'static {
    async fn mutation_login(
        payload: AuthLoginRequestDto,
        state: &AppState,
    ) -> Response;
    async fn mutation_mentor_login(
        payload: AuthLoginRequestDto,
        state: &AppState,
    ) -> Response;
    async fn mutation_register(
        payload: AuthRegisterRequestDto,
        state: &AppState,
    ) -> Response;
    async fn mutation_resend_otp(
        payload: AuthResendOtpRequestDto,
        state: &AppState,
    ) -> Response;
    async fn mutation_refresh_token(
        payload: AuthRefreshTokenRequestDto,
    ) -> Response;
    async fn mutation_forgot_password(
        payload: AuthResendOtpRequestDto,
        state: &AppState,
    ) -> Response;
    async fn mutation_verify_email(
        payload: AuthVerifyEmailRequestDto,
        state: &AppState,
    ) -> Response;
    async fn mutation_new_password(
        payload: AuthNewPasswordRequestDto,
        state: &AppState,
    ) -> Response;
}

#[derive(Clone)] // Added Clone derive
pub struct AuthService;

#[async_trait]
impl AuthServiceTrait for AuthService {
	async fn mutation_login(
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

				let permissions: Vec<String> = user.role.permissions.iter().map(|p| p.name.clone()).collect();
let access_token = match encode_access_token(payload.email.clone(), user.id.id.to_raw(), permissions) {
					Ok(token) => token,
					Err(_e) => {
						error!(
							"Failed to generate access token for {}: {}",
							payload.email, _e
						);
						return common_response(
							StatusCode::INTERNAL_SERVER_ERROR,
							"Failed to generate access token",
						);
					}
				};

				let permissions: Vec<String> = user.role.permissions.iter().map(|p| p.name.clone()).collect();
let refresh_token = match encode_refresh_token(payload.email.clone(), user.id.id.to_raw(), permissions) {
					Ok(token) => token,
					Err(_e) => {
						error!(
							"Failed to generate refresh token for {}: {}",
							payload.email, _e
						);
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

				if let Err(err_store) = auth_repo.query_store_user(user.clone()).await {
					error!(
						"Failed to store user cache for {}: {}",
						user.email, err_store
					);
					return common_response(
						StatusCode::BAD_REQUEST,
						"User already login or failed to cache",
					);
				}
				success_response(response)
			}
			Err(err_find) => {
				common_response(StatusCode::UNAUTHORIZED, &err_find.to_string())
			}
		}
	}

	async fn mutation_mentor_login(
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

				let user_detail = UsersDetailItemDto::from(&user);

				if user_detail.role.name != RolesEnum::Mentor.to_string() {
					return common_response(
						StatusCode::FORBIDDEN,
						"User does not have mentor privileges",
					);
				}

				let permissions: Vec<String> = user.role.permissions.iter().map(|p| p.name.clone()).collect();
let access_token = match encode_access_token(payload.email.clone(), user.id.id.to_raw(), permissions) {
					Ok(token) => token,
					Err(_e) => {
						error!(
							"Failed to generate access token for {}: {}",
							payload.email, _e
						);
						return common_response(
							StatusCode::INTERNAL_SERVER_ERROR,
							"Failed to generate access token",
						);
					}
				};

				let permissions: Vec<String> = user.role.permissions.iter().map(|p| p.name.clone()).collect();
let refresh_token = match encode_refresh_token(payload.email.clone(), user.id.id.to_raw(), permissions) {
					Ok(token) => token,
					Err(_e) => {
						error!(
							"Failed to generate refresh token for {}: {}",
							payload.email, _e
						);
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

				if let Err(err_store) = auth_repo.query_store_user(user.clone()).await {
					error!(
						"Failed to store user cache for {}: {}",
						user.email, err_store
					);
					return common_response(
						StatusCode::BAD_REQUEST,
						"User already login or failed to cache",
					);
				}
				success_response(response)
			}
			Err(err_find) => {
				common_response(StatusCode::UNAUTHORIZED, &err_find.to_string())
			}
		}
	}

	async fn mutation_register(
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
			Err(_e) => {
				error!("Failed to retrieve User role during registration: {}", _e);
				return common_response(StatusCode::BAD_REQUEST, "Role Not Found");
			}
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
			Err(_e) => {
				error!(
					"Failed to hash password during registration for {}: {}",
					payload.email, _e
				);
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Failed to hash password",
				);
			}
		};
		let new_user = AuthRegisterRequestDto {
			email: payload.email.clone(),
			password: hashed_password,
			fullname: payload.fullname,
			phone_number: payload.phone_number,
		};
		let otp = generate_otp::OtpManager::generate_otp();
		match auth_repo.query_store_otp(new_user.email.clone(), otp).await {
			Ok(_) => {
				let message = format!("your otp code is {otp}");
				if let Err(err_send) =
					send_email(&new_user.email, "OTP Verification", &message)
				{
					error!(
						"Failed to send OTP email to {}: {}",
						new_user.email, err_send
					);
					return common_response(
						StatusCode::INTERNAL_SERVER_ERROR,
						&err_send.to_string(),
					);
				}
			}
			Err(err_store) => {
				error!("Failed to store OTP for {}: {}", new_user.email, err_store);
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					&err_store.to_string(),
				);
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
				is_active: true,
				..Default::default()
			})
			.await
		{
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(err_create) => {
				error!("Failed to create user {}: {}", new_user.email, err_create);
				common_response(StatusCode::INTERNAL_SERVER_ERROR, &err_create.to_string())
			}
		}
	}

	async fn mutation_resend_otp(
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
		let message = format!("Your OTP code is {otp}");
		match auth_repo.query_store_otp(payload.email.clone(), otp).await {
			Ok(_) => match send_email(&payload.email, "OTP Verification", &message) {
				Ok(_) => common_response(StatusCode::OK, "OTP resent successfully"),
				Err(err_send) => {
					error!(
						"Failed to send OTP email to {}: {}",
						payload.email, err_send
					);
					common_response(StatusCode::BAD_REQUEST, &err_send.to_string())
				}
			},
			Err(err_store) => {
				error!("Failed to store OTP for {}: {}", payload.email, err_store);
				common_response(StatusCode::BAD_REQUEST, &err_store.to_string())
			}
		}
	}

	async fn mutation_refresh_token(payload: AuthRefreshTokenRequestDto) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		
		let surrealdb_ws = match surrealdb_init_ws().await {
	           Ok(db) => db,
	           Err(e) => {
	               error!("Failed to initialize websocket database: {}", e);
	               return common_response(StatusCode::INTERNAL_SERVER_ERROR, "Database initialization error");
	           }
	       };
	       let surrealdb_mem = match surrealdb_init_mem().await {
	           Ok(db) => db,
	           Err(e) => {
	               error!("Failed to initialize memory database: {}", e);
	               return common_response(StatusCode::INTERNAL_SERVER_ERROR, "Database initialization error");
	           }
	       };
		let state = AppState { surrealdb_ws, surrealdb_mem };
		let user_repo = UsersRepository::new(&state);
		let user = match decode_refresh_token(&payload.refresh_token) {
			Ok(token_data) => {
				match user_repo.query_user_by_email(token_data.claims.sub.clone()).await {
					Ok(user) => user,
					Err(_) => return common_response(StatusCode::UNAUTHORIZED, "User not found"),
				}
			},
			Err(_e) => {
				return common_response(StatusCode::UNAUTHORIZED, "Invalid refresh token");
			}
		};

		let permissions: Vec<String> = user.role.permissions.iter().map(|p| p.name.clone()).collect();
		let access_token = match encode_access_token(user.email.clone(), user.id.id.to_raw(), permissions.clone()) {
			Ok(token) => token,
			Err(_e) => {
				error!("Failed to generate access token for {}: {}", user.email, _e);
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Failed to generate access token",
				);
			}
		};
		let refresh_token = match encode_refresh_token(user.email.clone(), user.id.id.to_raw(), permissions) {
			Ok(token) => token,
			Err(_e) => {
				error!("Failed to generate refresh token for {}: {}", user.email, _e);
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

	async fn mutation_forgot_password(
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
			Err(err_find) if err_find.to_string().contains("User not found") => {
				return common_response(StatusCode::BAD_REQUEST, "User not found");
			}
			Err(err_other) => {
				error!(
					"Error finding user for forgot password {}: {}",
					payload.email, err_other
				);
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					&err_other.to_string(),
				);
			}
		};
		let permissions: Vec<String> = user.role.permissions.iter().map(|p| p.name.clone()).collect();
let token = match encode_reset_password_token(user.email.clone(), user.id.id.to_raw(), permissions) {
			Ok(token) => token,
			Err(_e) => {
				error!(
					"Failed to generate reset password token for {}: {}",
					user.email, _e
				);
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Failed to generate access token",
				);
			}
		};
		let env = &crate::enviroment::ENV;
		let fe_url = env.fe_url.clone();
		let message = format!(
			"You have requested a password reset. Please click the link below to continue: {fe_url}/auth/reset-password?token={token}"
		);
		match send_email(&payload.email, "Reset Password Request", &message) {
			Ok(_) => common_response(StatusCode::OK, "Reset Password request send"),
			Err(err_send) => {
				error!(
					"Failed to send reset password email to {}: {}",
					payload.email, err_send
				);
				common_response(StatusCode::BAD_REQUEST, &err_send.to_string())
			}
		}
	}

	async fn mutation_verify_email(
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
			_ => {
				return common_response(StatusCode::NOT_FOUND, "User not found");
			}
		};
		let patch = UsersSchema {
			id: user.id.clone(),
			is_active: true,
			..UsersSchema::from(user.clone())
		};
		match auth_repo.query_get_stored_otp(email.clone()).await {
			Ok(stored_otp) => match stored_otp == payload.otp {
				true => match user_repo.query_update_user(patch).await {
					Ok(_) => match auth_repo.query_delete_stored_otp(email.clone()).await {
						Ok(_) => common_response(StatusCode::OK, "Email verified successfully"),
						Err(e_del) => {
							error!("Failed to delete OTP for {}: {}", email, e_del);
							common_response(StatusCode::INTERNAL_SERVER_ERROR, &e_del.to_string())
						}
					},
					Err(err_update) => {
						common_response(StatusCode::BAD_REQUEST, &err_update.to_string())
					}
				},
				false => match auth_repo.query_delete_stored_otp(email.clone()).await {
					Ok(_) => common_response(StatusCode::BAD_REQUEST, "Failed to verify OTP"),
					Err(e_del_mismatch) => common_response(
						StatusCode::INTERNAL_SERVER_ERROR,
						&format!("Failed to delete OTP: {e_del_mismatch}"),
					),
				},
			},
			Err(err_get) => common_response(StatusCode::BAD_REQUEST, &err_get.to_string()),
		}
	}

	async fn mutation_new_password(
		payload: AuthNewPasswordRequestDto,
		state: &AppState,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = UsersRepository::new(state);
		let user_repo = UsersRepository::new(state);
		let email = match extract_email_token(payload.token.clone()) {
			Some(email) => email,
			None => {
				return common_response(StatusCode::BAD_REQUEST, "Invalid or missing token");
			}
		};
		let user = match user_repo.query_user_by_email(email).await {
			Ok(user) => user,
			Err(_) => return common_response(StatusCode::BAD_REQUEST, "User not found"),
		};
		let password = match hash_password(&payload.password) {
			Ok(p) => p,
			Err(_e) => {
				error!("Failed to hash new password for {}: {}", user.email, _e);
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Failed to hash password",
				);
			}
		};
		let patch = UsersSchema {
			id: user.id.clone(),
			password,
			..UsersSchema::from(user.clone())
		};
		match repo.query_update_user(patch).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(_e) => common_response(StatusCode::BAD_REQUEST, &_e.to_string()),
		}
	}
}

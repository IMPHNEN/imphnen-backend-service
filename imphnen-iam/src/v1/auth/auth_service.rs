use std::pin::Pin;
use std::future::Future;
use imphnen_libs::environment;
use imphnen_libs::AuthRepositoryTrait; // Added this import
use super::{
	AuthLoginRequestDto, AuthLoginResponsetDto, AuthNewPasswordRequestDto,
	AuthRefreshTokenRequestDto, AuthRegisterRequestDto, AuthRepository,
	AuthResendOtpRequestDto, AuthVerifyEmailRequestDto, TokenDto,
};
use crate::{
	AppState, ResponseSuccessDto, RolesEnum, RolesRepository,
	UsersDetailItemDto, UsersRepository, UsersSchema, common_response,
	decode_refresh_token, decode_access_token, encode_access_token, encode_refresh_token,
	encode_reset_password_token, get_iso_date,
	hash_password, send_email, success_response, validate_request, OtpManager,
};
use imphnen_entities::users::UserProfileExtensionDto;
use axum::{http::StatusCode, response::Response};
use crate::{AppError, error_response};

use tracing::error;
use tokio;
use uuid::Uuid;


pub trait AuthServiceTrait: Send + Sync + 'static {
    fn mutation_login(
        payload: AuthLoginRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn mutation_mentor_login(
        payload: AuthLoginRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn mutation_register(
        payload: AuthRegisterRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn mutation_resend_otp(
        payload: AuthResendOtpRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn mutation_refresh_token(
        payload: AuthRefreshTokenRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn mutation_forgot_password(
        payload: AuthResendOtpRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn mutation_verify_email(
        payload: AuthVerifyEmailRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn mutation_new_password(
        payload: AuthNewPasswordRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Response> + Send>>;
}

#[derive(Clone)] // Added Clone derive
pub struct AuthService;


impl AuthServiceTrait for AuthService {
	fn mutation_login(
		payload: AuthLoginRequestDto,
		state: &AppState,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        
        let state = state.to_owned();
        Box::pin(async move {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}

		let user_repo = UsersRepository::new(&state);
		let auth_repo = AuthRepository::new(&state);

		let email = &payload.email;
		let password = &payload.password;

        match auth_repo.validate_credentials(email, password, &state).await {
            Ok(_) => {
                // Credentials are valid, now fetch user details for the response (with Role)
                match user_repo.query_user_by_email(email.to_string()).await {
                    Ok(user) => {
                        if !user.is_active {
                             return error_response(AppError::AuthenticationError("Account not active, please verify your email".into()));
                        }

                        let user_id = user.id.clone();

                        let access_token = match encode_access_token(email.to_string(), user_id.clone()) {
                                            Ok(token) => token,
                                            Err(_e) => {
                                                error!(
                                                    "Failed to generate access token for {}: {}",
                                                    email, _e
                                                );
                                                return error_response(AppError::InternalServerError("Failed to generate access token".into()));
                            }
                        };

                        let refresh_token = match encode_refresh_token(email.to_string(), user_id) {
                                            Ok(token) => token,
                                            Err(_e) => {
                                                error!(
                                                    "Failed to generate refresh token for {}: {}",
                                                    email, _e
                                                );
                                                return error_response(AppError::InternalServerError("Failed to generate refresh token".into()));
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

                        // Only clone user if caching is required
                        // if let Err(err_store) = auth_repo.query_store_user(user.clone()).await {
                        // 	error!(
                        // 		"Failed to store user cache for {}: {}",
                        // 		user.email, err_store
                        // 	);
                        // 	return error_response(AppError::BadRequestError("User already login or failed to cache".into()));
                        // }
                        success_response(response)
                    },
                    Err(err_find) => {
                        error!("User found during validation but failed to fetch details: {}", err_find);
                        error_response(AppError::InternalServerError("Failed to fetch user details".into()))
                    }
                }
            },
            Err(e) => {
                error!("Login failed for {}: {}", email, e);
                error_response(AppError::AuthenticationError("Email or password not correct".into()))
            }
        }
        })
	}

	fn mutation_mentor_login(
		payload: AuthLoginRequestDto,
		state: &AppState,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}

		let user_repo = UsersRepository::new(&state);
		let auth_repo = AuthRepository::new(&state);

        match auth_repo.validate_credentials(&payload.email, &payload.password, &state).await {
            Ok(_) => {
                match user_repo.query_user_by_email(payload.email.clone()).await {
                    Ok(user) => {
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

                        let access_token = match encode_access_token(payload.email.clone(), user.id.clone()) {
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

                        let refresh_token = match encode_refresh_token(payload.email.clone(), user.id.clone()) {
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

                        // if let Err(err_store) = auth_repo.query_store_user(user.clone()).await {
                        // 	error!(
                        // 		"Failed to store user cache for {}: {}",
                        // 		user.email, err_store
                        // 	);
                        // 	return common_response(
                        // 		StatusCode::BAD_REQUEST,
                        // 		"User already login or failed to cache",
                        // 	);
                        // }
                        success_response(response)
                    },
                    Err(err_find) => {
                        common_response(StatusCode::UNAUTHORIZED, &err_find.to_string())
                    }
                }
            },
            Err(e) => {
                error!("Mentor login failed: {}", e);
                common_response(StatusCode::BAD_REQUEST, "Email or password not correct")
            }
        }
        })
	}

	fn mutation_register(
		payload: AuthRegisterRequestDto,
		state: &AppState,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let user_repo = UsersRepository::new(&state);
		let _auth_repo = AuthRepository::new(&state);
		let role_repo = RolesRepository::new(&state);
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
			phone_number: payload.phone_number.clone(),
		};
		let otp = OtpManager::generate_otp();
		// Store OTP (commented out for now)
		// match auth_repo.query_store_otp(new_user.email.clone(), otp.clone()).await {
		// 	Ok(_) => {
				let message = format!("your otp code is {}", otp.code);
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
		// 	}
		// 	Err(err_store) => {
		// 		error!("Failed to store OTP for {}: {}", new_user.email, err_store);
		// 		return common_response(
		// 			StatusCode::INTERNAL_SERVER_ERROR,
		// 			&err_store.to_string(),
		// 		);
		// 	}
		// }
		match user_repo
			.query_create_user(UsersSchema {
				id: Uuid::new_v4().to_string(), // Directly use Uuid
				email: Some(new_user.email.clone()), // email is now Option<String>
				fullname: Some(new_user.fullname.clone()), // fullname is now Option<String>
				password: Some(new_user.password.clone()), // password is now Option<String>
				created_at: get_iso_date(),
				updated_at: get_iso_date(),
				role_id: Some(Uuid::parse_str(&role.id).unwrap_or(Uuid::new_v4())), // Use role.id directly
				is_active: false,
                is_deleted: false,
                profile_extension: Some(UserProfileExtensionDto {
                    phone_number: new_user.phone_number.clone(),
                    ..Default::default()
                }),
                legal_name: None,
                avatar: None,
                mentor_id: None,
			})
			.await
		{
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(err_create) => {
				error!("Failed to create user {}: {}", new_user.email, err_create);
				common_response(StatusCode::INTERNAL_SERVER_ERROR, &err_create.to_string())
			}
		}
        })
	}

	fn mutation_resend_otp(
		payload: AuthResendOtpRequestDto,
		state: &AppState,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let user_repo = UsersRepository::new(&state);
		
		if user_repo.query_user_by_email(payload.email.clone()).await.is_err() {
			return common_response(StatusCode::BAD_REQUEST, "User not found");
		}
		
		// let _auth_repo = AuthRepository::new(&state);
		// let _ = auth_repo.query_get_stored_otp(payload.email.clone()).await;
		let otp = OtpManager::generate_otp();
		let message = format!("Your OTP code is {}", otp.code);
		
		match send_email(&payload.email, "OTP Verification", &message) {
			Ok(_) => common_response(StatusCode::OK, "OTP sent"),
			Err(err_send) => {
				error!(
					"Failed to send OTP email to {}: {}",
					payload.email, err_send
				);
				common_response(StatusCode::BAD_REQUEST, &err_send.to_string())
			}
		}
        })
	}

	fn mutation_refresh_token(
		payload: AuthRefreshTokenRequestDto,
		state: &AppState,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		
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

		let access_token = match encode_access_token(user.email.clone(), user.id.clone()) {
			Ok(token) => token,
			Err(_e) => {
				error!("Failed to generate access token for {}: {}", user.email, _e);
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Failed to generate access token",
				);
			}
		};
		let refresh_token = match encode_refresh_token(user.email.clone(), user.id.clone()) {
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
        })
	}

	fn mutation_forgot_password(
		payload: AuthResendOtpRequestDto,
		state: &AppState,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
            if let Err((status, message)) = validate_request(&payload) {
                return common_response(status, &message);
            }

            tokio::spawn(async move {
                let user_repo = UsersRepository::new(&state);
                if let Ok(user) = user_repo.query_user_by_email(payload.email.clone()).await {
                    let token = match encode_reset_password_token(user.email.clone(), user.id.clone()) {
                        Ok(token) => token,
                        Err(_e) => {
                            error!("Failed to generate reset password token for {}: {}", user.email, _e);
                            return;
                        }
                    };

                    let env = &environment::ENV;
                    let fe_url = env.fe_url.clone();
                    let message = format!(
                        "You have requested a password reset. Please click the link below to continue: {fe_url}/auth/reset-password?token={token}"
                    );

                    if let Err(err_send) = send_email(&payload.email, "Reset Password Request", &message) {
                        error!("Failed to send reset password email to {}: {}", payload.email, err_send);
                    }
                }
            });

            common_response(StatusCode::OK, "If your email is registered, you will receive a password reset link.")
        })
	}

	fn mutation_verify_email(
		payload: AuthVerifyEmailRequestDto,
		state: &AppState,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let user_repo = UsersRepository::new(&state);
		let _auth_repo = AuthRepository::new(&state);
		let email = payload.email.clone();
		let user = match user_repo.query_user_by_email(email.clone()).await {
			Ok(user) => user,
			_ => {
				return common_response(StatusCode::NOT_FOUND, "User not found");
			}
		};

		if user.is_active {
			return common_response(StatusCode::BAD_REQUEST, "User already active");
		}

		        let patch = UsersSchema {
					id: user.id.clone(),
					is_active: true,
		            email: Some(user.email.clone()),
		            fullname: Some(user.fullname),
		            password: Some(user.password.clone()),
		            avatar: user.avatar,
		            is_deleted: user.is_deleted,
		            created_at: user.created_at,
		            updated_at: user.updated_at,
					legal_name: user.legal_name,
                    profile_extension: user.profile_extension.clone(),
		            role_id: Uuid::parse_str(&user.role.id).ok(),
		            mentor_id: user.mentor_id,
				};
		        // Simulate OTP verification and user update success for now.
		        // The actual OTP logic involving AuthRepository needs to be refactored for Postgres.
		        return match user_repo.query_update_user(patch).await {
		            Ok(_) => common_response(StatusCode::OK, "Email verified successfully (simulated)"),
		            Err(err_update) => common_response(StatusCode::BAD_REQUEST, &err_update.to_string()),
		        };
		})
	}

	fn mutation_new_password(
		payload: AuthNewPasswordRequestDto,
		state: &AppState,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let user_repo = UsersRepository::new(&state);
		
		let email = match decode_access_token(&payload.token) {
			Ok(claims) => claims.claims.sub,
			Err(_) => return common_response(StatusCode::BAD_REQUEST, "Invalid or missing token"),
		};
		
		let user = match user_repo.query_user_by_email(email.clone()).await {
			Ok(u) => u,
			Err(e) => return common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		};
		
		let password_hash = match hash_password(&payload.password) {
			Ok(ph) => ph,
			Err(e) => {
				error!("Failed to hash new password for {}: {}", user.email, e);
				return common_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password");
			}
		};
        
        let role_id_uuid = match Uuid::parse_str(&user.role.id) {
            Ok(uuid) => Some(uuid),
            Err(e) => {
                error!("Failed to parse role ID {}: {}", user.role.id, e);
                None
            }
        };

		let patch = UsersSchema {
			id: user.id.clone(),
			password: Some(password_hash),
            email: Some(user.email.clone()),
            fullname: Some(user.fullname),
            avatar: user.avatar,
            is_active: user.is_active,
            is_deleted: user.is_deleted,
            created_at: user.created_at,
            updated_at: user.updated_at,
            profile_extension: user.profile_extension.clone(),
            role_id: role_id_uuid,
			legal_name: user.legal_name,
            mentor_id: user.mentor_id,
		};
		match user_repo.query_update_user(patch).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(_e) => common_response(StatusCode::BAD_REQUEST, &_e.to_string()),
		}
        })
	}
}
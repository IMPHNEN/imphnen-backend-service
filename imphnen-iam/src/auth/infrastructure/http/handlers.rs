use super::dto::{
	AuthLoginRequestDto, AuthLoginResponsetDto, AuthNewPasswordRequestDto,
	AuthRefreshTokenRequestDto, AuthRegisterRequestDto, AuthResendOtpRequestDto,
	AuthVerifyEmailRequestDto, TokenDto,
};
use crate::auth::domain::AuthService;
use crate::auth::domain::types::{
	LoginInput, NewPasswordInput, RefreshTokenInput, RegisterInput, ResendOtpInput,
	VerifyEmailInput,
};
use crate::users::infrastructure::http::dto::UsersDetailItemDto;
use axum::{Extension, response::IntoResponse};
use imphnen_entities::RolesDetailItemDto;
use imphnen_libs::ValidatedJson;
use imphnen_utils::{ApiMessage, ApiSuccess, AppError};
use std::sync::Arc;

fn login_resp_to_dto(
	output: crate::auth::domain::types::LoginOutput,
) -> AuthLoginResponsetDto {
	let u = output.user;
	AuthLoginResponsetDto {
		token: TokenDto {
			access_token: output.token.access_token,
			refresh_token: output.token.refresh_token,
		},
		user: UsersDetailItemDto {
			id: u.id,
			role: RolesDetailItemDto::from(&u.role),
			fullname: u.fullname,
			legal_name: u.legal_name,
			email: u.email,
			avatar: u.avatar,
			is_active: u.is_active,
			profile_extension: u.profile_extension,
			created_at: u.created_at,
			updated_at: u.updated_at,
		},
	}
}

#[utoipa::path(
    post,
    path = "/v1/iam/auth/login",
    request_body = AuthLoginRequestDto,
    responses(
        (status = 200, description = "[PUBLIC] Login successful"),
        (status = 401, description = "[PUBLIC] Login failed")
    ),
    tag = "Authentication"
)]
pub async fn post_login(
	Extension(service): Extension<Arc<dyn AuthService>>,
	ValidatedJson(payload): ValidatedJson<AuthLoginRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	let input = LoginInput {
		email: payload.email,
		password: payload.password,
	};
	let resp = service.login(input).await?;
	Ok(ApiSuccess(login_resp_to_dto(resp)))
}

#[utoipa::path(
    post,
    path = "/v1/iam/auth/login-mentor",
    request_body = AuthLoginRequestDto,
    responses(
        (status = 200, description = "[PUBLIC] Mentor login successful"),
        (status = 401, description = "[PUBLIC] Mentor login failed"),
        (status = 403, description = "[PUBLIC] Forbidden - Not a mentor")
    ),
    tag = "Authentication"
)]
pub async fn post_login_mentor(
	Extension(service): Extension<Arc<dyn AuthService>>,
	ValidatedJson(payload): ValidatedJson<AuthLoginRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	let input = LoginInput {
		email: payload.email,
		password: payload.password,
	};
	let resp = service.login_mentor(input).await?;
	Ok(ApiSuccess(login_resp_to_dto(resp)))
}

#[utoipa::path(
    post,
    path = "/v1/iam/auth/register",
    request_body = AuthRegisterRequestDto,
    responses(
        (status = 201, description = "[PUBLIC] Register successful"),
        (status = 400, description = "[PUBLIC] Register failed")
    ),
    tag = "Authentication"
)]
pub async fn post_register(
	Extension(service): Extension<Arc<dyn AuthService>>,
	ValidatedJson(payload): ValidatedJson<AuthRegisterRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	let input = RegisterInput {
		email: payload.email,
		password: payload.password,
		fullname: payload.fullname,
		phone_number: payload.phone_number,
	};
	service.register(input).await?;
	Ok(ApiMessage::created("Registration successful"))
}

#[utoipa::path(
    post,
    path = "/v1/iam/auth/verify-email",
    request_body = AuthVerifyEmailRequestDto,
    responses(
        (status = 200, description = "[PUBLIC] Verify email successful"),
        (status = 400, description = "[PUBLIC] Verify email failed")
    ),
    tag = "Authentication"
)]
pub async fn post_verify_email(
	Extension(service): Extension<Arc<dyn AuthService>>,
	ValidatedJson(payload): ValidatedJson<AuthVerifyEmailRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	let input = VerifyEmailInput {
		email: payload.email,
		otp: payload.otp,
	};
	service.verify_email(input).await?;
	Ok(ApiMessage::ok("Email verified successfully"))
}

#[utoipa::path(
    post,
    path = "/v1/iam/auth/send-otp",
    request_body = AuthResendOtpRequestDto,
    responses(
        (status = 200, description = "[PUBLIC] Resend OTP successful"),
        (status = 400, description = "[PUBLIC] Resend OTP failed")
    ),
    tag = "Authentication"
)]
pub async fn post_resend_otp(
	Extension(service): Extension<Arc<dyn AuthService>>,
	ValidatedJson(payload): ValidatedJson<AuthResendOtpRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	let input = ResendOtpInput {
		email: payload.email,
	};
	service.resend_otp(input).await?;
	Ok(ApiMessage::ok("OTP sent"))
}

#[utoipa::path(
    post,
    path = "/v1/iam/auth/forgot",
    request_body = AuthResendOtpRequestDto,
    responses(
        (status = 200, description = "[PUBLIC] Forgot password request successful")
    ),
    tag = "Authentication"
)]
pub async fn post_forgot_password(
	Extension(service): Extension<Arc<dyn AuthService>>,
	ValidatedJson(payload): ValidatedJson<AuthResendOtpRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	let input = ResendOtpInput {
		email: payload.email,
	};
	service.forgot_password(input).await?;
	Ok(ApiMessage::ok(
		"If your email is registered, you will receive a password reset link.",
	))
}

#[utoipa::path(
    post,
    path = "/v1/iam/auth/new-password",
    request_body = AuthNewPasswordRequestDto,
    responses(
        (status = 200, description = "[PUBLIC] New password set successfully"),
        (status = 400, description = "[PUBLIC] New password request failed")
    ),
    tag = "Authentication"
)]
pub async fn post_new_password(
	Extension(service): Extension<Arc<dyn AuthService>>,
	ValidatedJson(payload): ValidatedJson<AuthNewPasswordRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	let input = NewPasswordInput {
		token: payload.token,
		password: payload.password,
	};
	service.new_password(input).await?;
	Ok(ApiMessage::ok("Password updated successfully"))
}

#[utoipa::path(
    post,
    path = "/v1/iam/auth/refresh",
    request_body = AuthRefreshTokenRequestDto,
    responses(
        (status = 200, description = "[PUBLIC] Refresh token successful"),
        (status = 401, description = "[PUBLIC] Invalid refresh token")
    ),
    tag = "Authentication"
)]
pub async fn post_refresh_token(
	Extension(service): Extension<Arc<dyn AuthService>>,
	ValidatedJson(payload): ValidatedJson<AuthRefreshTokenRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	let input = RefreshTokenInput {
		refresh_token: payload.refresh_token,
	};
	let tokens = service.refresh_token(input).await?;
	Ok(ApiSuccess(TokenDto {
		access_token: tokens.access_token,
		refresh_token: tokens.refresh_token,
	}))
}

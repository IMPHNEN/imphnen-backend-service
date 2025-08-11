use super::{
	AuthLoginRequestDto, AuthRefreshTokenRequestDto, AuthRegisterRequestDto,
	AuthResendOtpRequestDto, AuthVerifyEmailRequestDto,
};
use crate::{AppState, v1::AuthLoginResponsetDto};
use crate::{AuthNewPasswordRequestDto, MessageResponseDto, ResponseSuccessDto};
use axum::{Extension, Json, response::IntoResponse};
use crate::v1::auth::auth_service::AuthServiceTrait;
use crate::v1::auth::auth_service::AuthService;

#[utoipa::path(
    post,
    path = "/v1/auth/login",
    request_body = AuthLoginRequestDto,
    responses(
        (status = 200, description = "Login successful", body = ResponseSuccessDto<AuthLoginResponsetDto>),
        (status = 401, description = "Login failed", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]
pub async fn post_login(
	Extension(state): Extension<AppState>,
	Json(payload): Json<AuthLoginRequestDto>,
) -> impl IntoResponse {
	AuthService::mutation_login(payload, &state).await
}

#[utoipa::path(
    post,
    path = "/v1/auth/login-mentor",
    request_body = AuthLoginRequestDto,
    responses(
        (status = 200, description = "Mentor login successful", body = ResponseSuccessDto<AuthLoginResponsetDto>),
        (status = 401, description = "Mentor login failed", body = MessageResponseDto),
        (status = 403, description = "Forbidden - Not a mentor", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]
pub async fn post_login_mentor(
	Extension(state): Extension<AppState>,
	Json(payload): Json<AuthLoginRequestDto>,
) -> impl IntoResponse {
	AuthService::mutation_mentor_login(payload, &state).await
}

#[utoipa::path(
    post,
    path = "/v1/auth/register",
    request_body = AuthRegisterRequestDto,
    responses(
        (status = 200, description = "Register successful", body = MessageResponseDto),
        (status = 401, description = "Register failed", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]
pub async fn post_register(
	Extension(state): Extension<AppState>,
	Json(payload): Json<AuthRegisterRequestDto>,
) -> impl IntoResponse {
	AuthService::mutation_register(payload, &state).await
}

#[utoipa::path(
    post,
    path = "/v1/auth/verify-email",
    request_body = AuthVerifyEmailRequestDto,
    responses(
        (status = 200, description = "Verify email successful", body = MessageResponseDto),
        (status = 401, description = "Verify email failed", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]
pub async fn post_verify_email(
	Extension(state): Extension<AppState>,
	Json(payload): Json<AuthVerifyEmailRequestDto>,
) -> impl IntoResponse {
	AuthService::mutation_verify_email(payload, &state).await
}

#[utoipa::path(
    post,
    path = "/v1/auth/send-otp",
    request_body = AuthResendOtpRequestDto,
    responses(
        (status = 200, description = "Resend otp successful", body = MessageResponseDto),
        (status = 401, description = "Resend otp failed", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]
pub async fn post_resend_otp(
	Extension(state): Extension<AppState>,
	Json(payload): Json<AuthResendOtpRequestDto>,
) -> impl IntoResponse {
	AuthService::mutation_resend_otp(payload, &state).await
}

#[utoipa::path(
    post,
    path = "/v1/auth/forgot",
    request_body = AuthResendOtpRequestDto,
    responses(
        (status = 200, description = "Forgot password request successful", body = MessageResponseDto),
        (status = 401, description = "Forgot password request failed", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]
pub async fn post_forgot_password(
	Extension(state): Extension<AppState>,
	Json(payload): Json<AuthResendOtpRequestDto>,
) -> impl IntoResponse {
	AuthService::mutation_forgot_password(payload, &state).await
}

#[utoipa::path(
    post,
    path = "/v1/auth/new-password",
    request_body = AuthNewPasswordRequestDto,
    responses(
        (status = 200, description = "New password request successful", body = MessageResponseDto),
        (status = 401, description = "New password request failed", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]
pub async fn post_new_password(
	Extension(state): Extension<AppState>,
	Json(payload): Json<AuthNewPasswordRequestDto>,
) -> impl IntoResponse {
	AuthService::mutation_new_password(payload, &state).await
}

#[utoipa::path(
    post,
    path = "/v1/auth/refresh",
    request_body = AuthRefreshTokenRequestDto,
    responses(
        (status = 200, description = "Refresh token request successful", body = MessageResponseDto),
        (status = 401, description = "Refresh token request failed", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]
pub async fn post_refresh_token(
	Json(payload): Json<AuthRefreshTokenRequestDto>,
) -> impl IntoResponse {
	AuthService::mutation_refresh_token(payload).await
}

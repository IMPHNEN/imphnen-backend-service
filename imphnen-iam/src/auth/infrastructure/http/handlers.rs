use std::sync::Arc;
use axum::{Extension, response::IntoResponse};
use imphnen_libs::ValidatedJson;
use imphnen_utils::{ApiSuccess, ApiMessage, AppError};
use super::dto::{
    AuthLoginRequestDto, AuthRegisterRequestDto, AuthResendOtpRequestDto,
    AuthVerifyEmailRequestDto, AuthNewPasswordRequestDto, AuthRefreshTokenRequestDto,
};
use crate::auth::domain::AuthService;

#[utoipa::path(
    post,
    path = "/v1/auth/login",
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
    let resp = service.login(payload).await?;
    Ok(ApiSuccess(resp))
}

#[utoipa::path(
    post,
    path = "/v1/auth/login-mentor",
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
    let resp = service.login_mentor(payload).await?;
    Ok(ApiSuccess(resp))
}

#[utoipa::path(
    post,
    path = "/v1/auth/register",
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
    service.register(payload).await?;
    Ok(ApiMessage::created("Registration successful"))
}

#[utoipa::path(
    post,
    path = "/v1/auth/verify-email",
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
    service.verify_email(payload).await?;
    Ok(ApiMessage::ok("Email verified successfully"))
}

#[utoipa::path(
    post,
    path = "/v1/auth/send-otp",
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
    service.resend_otp(payload).await?;
    Ok(ApiMessage::ok("OTP sent"))
}

#[utoipa::path(
    post,
    path = "/v1/auth/forgot",
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
    service.forgot_password(payload).await?;
    Ok(ApiMessage::ok(
        "If your email is registered, you will receive a password reset link.",
    ))
}

#[utoipa::path(
    post,
    path = "/v1/auth/new-password",
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
    service.new_password(payload).await?;
    Ok(ApiMessage::ok("Password updated successfully"))
}

#[utoipa::path(
    post,
    path = "/v1/auth/refresh",
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
    let resp = service.refresh_token(payload).await?;
    Ok(ApiSuccess(resp))
}


use axum::{Extension, Json, response::IntoResponse};
use std::sync::Arc;
use imphnen_utils::response_format::{ApiSuccess, ApiMessage};
use crate::auth::domain::service::HackathonAuthService;
use crate::middleware::hackathon_auth::HackathonAuthUser;
use super::dto::*;

pub async fn signup_handler(
    Extension(service): Extension<Arc<dyn HackathonAuthService>>,
    Json(body): Json<SignupRequest>,
) -> Result<ApiMessage, imphnen_utils::errors::AppError> {
    service.signup(body.email, body.password, body.fullname).await?;
    Ok(ApiMessage::created("Registration successful! Please check your email to activate your account."))
}

pub async fn login_handler(
    Extension(service): Extension<Arc<dyn HackathonAuthService>>,
    Json(body): Json<LoginRequest>,
) -> Result<axum::response::Response, imphnen_utils::errors::AppError> {
    let (tokens, user) = service.login(body.email, body.password).await?;
    Ok(ApiSuccess(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        user,
    }).into_response())
}

pub async fn github_auth_handler(
    Extension(service): Extension<Arc<dyn HackathonAuthService>>,
    Json(body): Json<GitHubAuthRequest>,
) -> Result<axum::response::Response, imphnen_utils::errors::AppError> {
    let (tokens, user) = service.github_auth(body.code).await?;
    Ok(ApiSuccess(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        user,
    }).into_response())
}

pub async fn get_session_handler(
    Extension(service): Extension<Arc<dyn HackathonAuthService>>,
    Extension(auth_user): Extension<HackathonAuthUser>,
) -> Result<axum::response::Response, imphnen_utils::errors::AppError> {
    let user = service.get_session(auth_user.user_id).await?;
    Ok(ApiSuccess(user).into_response())
}

pub async fn forgot_password_handler(
    Extension(service): Extension<Arc<dyn HackathonAuthService>>,
    Json(body): Json<ForgotPasswordRequest>,
) -> Result<ApiMessage, imphnen_utils::errors::AppError> {
    service.forgot_password(body.email).await?;
    Ok(ApiMessage::ok("If an account with that email exists, a password reset link has been sent."))
}

pub async fn reset_password_handler(
    Extension(service): Extension<Arc<dyn HackathonAuthService>>,
    Json(body): Json<ResetPasswordRequest>,
) -> Result<ApiMessage, imphnen_utils::errors::AppError> {
    service.reset_password(body.access_token, body.new_password).await?;
    Ok(ApiMessage::ok("Password has been successfully reset."))
}

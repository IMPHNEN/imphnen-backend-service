use axum::{Extension, Json, response::IntoResponse};
use axum::extract::Query;
use std::sync::Arc;
use serde::Deserialize;
use imphnen_utils::response_format::ApiSuccess;
use imphnen_utils::errors::AppError;
use crate::auth::domain::service::QrAuthService;
use crate::config::QrConfig;
use super::dto::{RegisterRequest, LoginRequest, RefreshRequest, AuthResponse, TokensResponse};

pub async fn register_handler(
    Extension(service): Extension<Arc<dyn QrAuthService>>,
    Json(body): Json<RegisterRequest>,
) -> Result<axum::response::Response, AppError> {
    let (tokens, user) = service.register(body.email, body.password, body.name).await?;
    Ok(ApiSuccess(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        user,
    }).into_response())
}

pub async fn login_handler(
    Extension(service): Extension<Arc<dyn QrAuthService>>,
    Json(body): Json<LoginRequest>,
) -> Result<axum::response::Response, AppError> {
    let (tokens, user) = service.login(body.email, body.password).await?;
    Ok(ApiSuccess(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        user,
    }).into_response())
}

pub async fn google_redirect_handler(
    Extension(config): Extension<Arc<QrConfig>>,
) -> Result<axum::response::Response, AppError> {
    let url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=email+profile",
        config.google_client_id,
        config.google_redirect_url,
    );
    Ok(axum::response::Redirect::temporary(&url).into_response())
}

#[derive(Debug, Deserialize)]
pub struct GoogleCallbackQuery {
    pub code: String,
}

pub async fn google_callback_handler(
    Extension(service): Extension<Arc<dyn QrAuthService>>,
    Query(params): Query<GoogleCallbackQuery>,
) -> Result<axum::response::Response, AppError> {
    let (tokens, user) = service.google_callback(params.code).await?;
    Ok(ApiSuccess(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        user,
    }).into_response())
}

pub async fn refresh_handler(
    Extension(service): Extension<Arc<dyn QrAuthService>>,
    Json(body): Json<RefreshRequest>,
) -> Result<axum::response::Response, AppError> {
    let tokens = service.refresh_token(body.refresh_token).await?;
    Ok(ApiSuccess(TokensResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
    }).into_response())
}

use axum::{Router, routing::post};

pub mod auth_controller;
pub mod auth_dto;
pub mod auth_repository;
pub mod auth_schema;
pub mod auth_service;
pub mod google;

// Export only the essential types and functions from each submodule
pub use auth_dto::{
    AuthLoginRequestDto,
    AuthLoginResponsetDto,
    AuthRegisterRequestDto,
    AuthResendOtpRequestDto,
    AuthVerifyEmailRequestDto,
    AuthNewPasswordRequestDto,
    AuthRefreshTokenRequestDto,
    TokenDto,
    UserCacheSchema,
};

pub use auth_repository::AuthRepository;
pub use imphnen_libs::AuthRepositoryTrait;
pub use auth_schema::AuthOtpSchema;
pub use auth_service::AuthServiceTrait;

// Export controller functions that are used in routing
pub use auth_controller::{
    post_login,
    post_login_mentor,
    post_register,
    post_forgot_password,
    post_new_password,
    post_refresh_token,
    post_resend_otp,
    post_verify_email
};

pub fn auth_router() -> Router {
	Router::new()
        .nest("/google", google::google_oauth_controller::GoogleOauthController::new().get_routes())
		.route("/forgot", post(post_forgot_password))
		.route("/login", post(post_login))
		.route("/login-mentor", post(post_login_mentor))
		.route("/new-password", post(post_new_password))
		.route("/refresh", post(post_refresh_token))
		.route("/register", post(post_register))
		.route("/send-otp", post(post_resend_otp))
		.route("/verify-email", post(post_verify_email))
}

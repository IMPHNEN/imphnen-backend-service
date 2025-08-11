use axum::{Router, routing::post};

pub mod auth_controller;
pub mod auth_dto;
pub mod auth_repository;
pub mod auth_schema;
pub mod auth_service;
pub mod google;

pub use auth_dto::*;
pub use auth_repository::*;
pub use auth_schema::*;
pub use auth_service::*;

pub fn auth_router() -> Router {
	Router::new()
        .nest("/google", google::google_oauth_controller::GoogleOauthController::new().get_routes())
		.route("/forgot", post(auth_controller::post_forgot_password))
		.route("/login", post(auth_controller::post_login))
		.route("/login-mentor", post(auth_controller::post_login_mentor))
		.route("/new-password", post(auth_controller::post_new_password))
		.route("/refresh", post(auth_controller::post_refresh_token))
		.route("/register", post(auth_controller::post_register))
		.route("/send-otp", post(auth_controller::post_resend_otp))
		.route("/verify-email", post(auth_controller::post_verify_email))
}

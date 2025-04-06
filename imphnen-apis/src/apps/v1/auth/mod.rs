use axum::{routing::post, Router};

pub mod auth_controller;
pub mod auth_dto;
pub mod auth_middleware;
pub mod auth_repository;
pub mod auth_schema;
pub mod auth_service;

#[cfg(test)]
pub mod auth_controller_test;
#[cfg(test)]
pub mod auth_repository_test;

pub use auth_dto::*;
pub use auth_repository::*;
pub use auth_schema::*;
pub use auth_service::*;

pub fn auth_router() -> Router {
	Router::new()
		.route("/forgot", post(auth_controller::post_forgot_password))
		.route("/login", post(auth_controller::post_login))
		.route("/new-password", post(auth_controller::post_new_password))
		.route("/refresh", post(auth_controller::post_refresh_token))
		.route("/register", post(auth_controller::post_register))
		.route("/send-otp", post(auth_controller::post_resend_otp))
		.route("/verify-email", post(auth_controller::post_verify_email))
}

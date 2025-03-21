use axum::{routing::post, Router};

pub mod auth_controller;
pub mod auth_dto;
pub mod auth_middleware;
pub mod auth_repository;
pub mod auth_service;

pub use auth_dto::*;
pub use auth_repository::*;
pub use auth_service::*;

pub fn auth_router() -> Router {
	Router::new()
		.route("/login", post(auth_controller::post_login))
		.route("/register", post(auth_controller::post_register))
		.route("/verify", post(auth_controller::post_verify_email))
		.route("/resend", post(auth_controller::post_resend_otp))
		.route("/forgot", post(auth_controller::post_forgot_password))
}

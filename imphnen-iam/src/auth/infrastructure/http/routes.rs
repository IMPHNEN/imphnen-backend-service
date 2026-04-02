use super::handlers::{
	post_forgot_password, post_login, post_login_mentor, post_new_password,
	post_refresh_token, post_register, post_resend_otp, post_verify_email,
};
use crate::auth::application::AuthServiceImpl;
use crate::auth::domain::AuthService;
use crate::roles::infrastructure::persistence::PostgresRoleRepository;
use crate::users::infrastructure::persistence::PostgresUserRepository;
use axum::{Extension, Router, routing::post};
use imphnen_libs::AppState;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub fn auth_public_routes(_db: DatabaseConnection, state: Arc<AppState>) -> Router {
	let user_repo = Arc::new(PostgresUserRepository::new(
		state.postgres_connection.conn.clone(),
	));
	let role_repo = Arc::new(PostgresRoleRepository::new(
		state.postgres_connection.conn.clone(),
	));
	let auth_service: Arc<dyn AuthService> =
		Arc::new(AuthServiceImpl::new(user_repo, role_repo));
	Router::new()
		.route("/auth/login", post(post_login))
		.route("/auth/login-mentor", post(post_login_mentor))
		.route("/auth/register", post(post_register))
		.route("/auth/verify-email", post(post_verify_email))
		.route("/auth/send-otp", post(post_resend_otp))
		.route("/auth/forgot", post(post_forgot_password))
		.route("/auth/new-password", post(post_new_password))
		.route("/auth/refresh", post(post_refresh_token))
		.layer(Extension(auth_service))
		.layer(Extension((*state).clone()))
}

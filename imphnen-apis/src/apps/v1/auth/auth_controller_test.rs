use crate::{
	apps::v1::auth::auth_router, create_mock_app_state, create_test_user,
	encode_refresh_token, AuthOtpSchema, OtpManager, ResourceEnum, RolesRepository,
};
use axum::{http::StatusCode, Extension};
use axum_test::TestServer;
use serde_json::json;
use surrealdb::Uuid;

#[tokio::test]
async fn test_login_should_fail_with_invalid_user() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let payload = json!({
		"email": "nonexistent@example.com",
		"password": "wrongpassword"
	});
	let res = server.post("/v1/auth/login").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
	let body = res.text();
	println!("🧪 Response Body: {}", body);
}

#[tokio::test]
async fn test_login_should_fail_with_wrong_password() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state.clone()));
	let server = TestServer::new(app).unwrap();
	let repo = crate::apps::v1::users::UsersRepository::new(&state);
	let role_repo = RolesRepository::new(&state);
	let role_id = role_repo
		.query_role_by_name("Student".to_string())
		.await
		.unwrap()
		.id;
	let mut user = create_test_user("user@example.com", "User Satu", true, &role_id);
	user.password = crate::hash_password("correctpassword").unwrap();
	repo.query_create_user(user).await.unwrap();
	let payload = json!({
		"email": "user@example.com",
		"password": "wrongpassword"
	});
	let res = server.post("/v1/auth/login").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
	let body = res.text();
	println!("🧪 Wrong password: {}", body);
}

#[tokio::test]
async fn test_login_should_fail_if_user_not_active() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state.clone()));
	let server = TestServer::new(app).unwrap();
	let repo = crate::apps::v1::users::UsersRepository::new(&state);
	let role_repo = RolesRepository::new(&state);
	let role_id = role_repo
		.query_role_by_name("Student".to_string())
		.await
		.unwrap()
		.id;
	let mut user = create_test_user(
		"inactive-again@example.com",
		"Inactive User",
		false,
		&role_id,
	);
	user.password = crate::hash_password("secret").unwrap();
	repo.query_create_user(user).await.unwrap();
	let payload = json!({
		"email": "inactive-again@example.com",
		"password": "secret"
	});
	let res = server.post("/v1/auth/login").json(&payload).await;
	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
	let body = res.text();
	println!("🧪 Not active user: {}", body);
}

#[tokio::test]
async fn test_login_should_succeed() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state.clone()));
	let server = TestServer::new(app).unwrap();
	let repo = crate::apps::v1::users::UsersRepository::new(&state);
	let role_repo = RolesRepository::new(&state);
	let role_id = role_repo
		.query_role_by_name("Student".to_string())
		.await
		.unwrap()
		.id;
	let mut user =
		create_test_user("active@example.com", "Active User", true, &role_id);
	user.password = crate::hash_password("secret").unwrap();
	repo.query_create_user(user).await.unwrap();
	let payload = json!({
		"email": "active@example.com",
		"password": "secret"
	});
	let res = server.post("/v1/auth/login").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::OK);
	let json = res.json::<serde_json::Value>();
	assert!(json["data"]["token"]["access_token"].is_string());
	assert!(json["data"]["user"]["email"] == "active@example.com");
}

#[tokio::test]
async fn test_login_should_fail_if_payload_empty() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let payload = json!({});
	let res = server.post("/v1/auth/login").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_login_should_fail_if_password_empty() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let payload = json!({
			"email": "test@test.com",
			"password": ""
	});
	let res = server.post("/v1/auth/login").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_login_should_fail_if_email_not_valid() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let payload = json!({
			"email": "test",
			"password": "test123"
	});
	let res = server.post("/v1/auth/login").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_register_should_succeed() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state.clone()));
	let server = TestServer::new(app).unwrap();
	let random_email = format!("validuser+{}@example.com", Uuid::new_v4());
	let payload = json!({
		"email": random_email,
		"password": "Validpass1!",
		"fullname": "Valid User",
		"student_type": "regular",
		"phone_number": "0812345678",
		"reffered_by": "Facebook",
		"refferal_code": "KFNB"
	});
	let res = server.post("/v1/auth/register").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_register_should_fail_if_email_already_taken() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state.clone()));
	let server = TestServer::new(app).unwrap();
	let repo = crate::apps::v1::users::UsersRepository::new(&state);
	let role_repo = RolesRepository::new(&state);
	let role_id = role_repo
		.query_role_by_name("Student".to_string())
		.await
		.unwrap()
		.id;
	let mut user =
		create_test_user("duplicate@example.com", "User Exists", false, &role_id);
	user.password = crate::hash_password("secret").unwrap();
	repo.query_create_user(user).await.unwrap();
	let payload = json!({
		"fullname": "User Exists",
		"email": "duplicate@example.com",
		"password": "secret",
		"student_type": "general",
		"phone_number": "081234567890",
		"referral_code": null,
		"referred_by": null
	});
	let res = server.post("/v1/auth/register").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_register_should_fail_if_email_invalid() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let payload = json!({
		"fullname": "Invalid Email",
		"email": "invalid-email",
		"password": "secret",
		"student_type": "general",
		"phone_number": "081234567890",
		"referral_code": null,
		"referred_by": null
	});
	let res = server.post("/v1/auth/register").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_register_should_fail_if_password_empty() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();

	let payload = json!({
		"fullname": "No Password",
		"email": "nopass@example.com",
		"password": "",
		"student_type": "general",
		"phone_number": "081234567890",
		"referral_code": null,
		"referred_by": null
	});
	let res = server.post("/v1/auth/register").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_register_should_fail_if_payload_empty() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let payload = json!({});
	let res = server.post("/v1/auth/register").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_send_otp_should_succeed() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let payload = json!({
		"email": "user@example.com"
	});
	let res = server.post("/v1/auth/send-otp").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::OK);
	let json = res.json::<serde_json::Value>();
	assert_eq!(json["message"], "OTP resent successfully");
}

#[tokio::test]
async fn test_send_otp_should_fail_if_email_empty() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let payload = json!({ "email": "" });
	let res = server.post("/v1/auth/send-otp").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
	let json = res.json::<serde_json::Value>();
	assert_eq!(json["message"], "Email cannot be empty, Email not valid");
}

#[tokio::test]
async fn test_send_otp_should_fail_with_invalid_email_format() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();

	let payload = json!({ "email": "not-an-email" });
	let res = server.post("/v1/auth/send-otp").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
	let json = res.json::<serde_json::Value>();
	assert_eq!(json["message"], "Email not valid");
}

#[tokio::test]
async fn test_send_otp_should_fail_if_payload_empty() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let payload = json!({});
	let res = server.post("/v1/auth/send-otp").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_send_otp_should_fail_if_otp_store_fails() {
	let mut state = create_mock_app_state().await;
	state.surrealdb_mem = surrealdb::Surreal::new::<surrealdb::engine::local::Mem>(())
		.await
		.expect("failed to create mock surrealdb_mem");
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let payload = serde_json::json!({ "email": "fail-store@example.com" });
	let res = server.post("/v1/auth/send-otp").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
	let body = res.text();
	assert!(
		body.contains("Specify a namespace to use") || body.contains("Failed store otp"),
		"Expected namespace setup error or store failure, got: {body}"
	);
}

#[tokio::test]
async fn test_forgot_password_should_succeed() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state.clone()));
	let server = TestServer::new(app).unwrap();
	let role_repo = RolesRepository::new(&state);
	let role_id = role_repo
		.query_role_by_name("Student".to_string())
		.await
		.unwrap()
		.id;
	let mut user =
		create_test_user("forgot@example.com", "Forgot User", true, &role_id);
	user.password = crate::hash_password("secret").unwrap();
	let user_repo = crate::apps::v1::users::UsersRepository::new(&state);
	user_repo.query_create_user(user).await.unwrap();
	let payload = json!({ "email": "forgot@example.com" });
	let res = server.post("/v1/auth/forgot").json(&payload).await;
	dbg!(res.text());
	assert_eq!(res.status_code(), StatusCode::OK);
	let body = res.text();
	assert!(
		body.contains("Reset Password request send"),
		"Unexpected response: {body}"
	);
}

#[tokio::test]
async fn test_forgot_password_should_fail_if_user_not_found() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let payload = json!({ "email": "notfound@example.com" });
	let res = server.post("/v1/auth/forgot").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
	let body = res.text();
	assert!(
		body.contains("User not found"),
		"Expected 'User not found', got: {body}"
	);
}

#[tokio::test]
async fn test_forgot_password_should_fail_if_email_invalid() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let payload = json!({ "email": "" });
	let res = server.post("/v1/auth/forgot").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
	let body = res.text();
	assert!(
		body.contains("Email not valid") || body.contains("User not found"),
		"Expected email error, got: {body}"
	);
}

#[tokio::test]
async fn test_refresh_should_succeed() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let valid_token = encode_refresh_token("refreshme@example.com".into()).unwrap();
	let payload = json!({
		"refresh_token": valid_token
	});
	let res = server.post("/v1/auth/refresh").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::OK);
	let body = res.json::<serde_json::Value>();
	assert!(body["data"]["access_token"].is_string());
	assert!(body["data"]["refresh_token"].is_string());
}

#[tokio::test]
async fn test_refresh_should_fail_with_invalid_token() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let payload = json!({
		"refresh_token": "invalid.token.value"
	});
	let res = server.post("/v1/auth/refresh").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);
	let body = res.text();
	assert!(body.contains("Invalid refresh token"));
}

#[tokio::test]
async fn test_refresh_should_fail_with_empty_payload() {
	let state = create_mock_app_state().await;
	let app = axum::Router::new()
		.nest("/v1/auth", auth_router())
		.layer(Extension(state));
	let server = TestServer::new(app).unwrap();
	let payload = json!({});
	let res = server.post("/v1/auth/refresh").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_verify_email_should_succeed() {
	let state = create_mock_app_state().await;
	let server = TestServer::new(
		axum::Router::new()
			.nest("/v1/auth", auth_router())
			.layer(Extension(state.clone())),
	)
	.unwrap();
	let role_repo = RolesRepository::new(&state);
	let role_id = role_repo
		.query_role_by_name("Student".to_string())
		.await
		.unwrap()
		.id;
	let user = create_test_user("verify@example.com", "Verify User", false, &role_id);
	let otp = OtpManager::generate_otp();
	let user_repo = crate::apps::v1::users::UsersRepository::new(&state);
	let auth_repo = crate::apps::v1::auth::AuthRepository::new(&state);
	user_repo.query_create_user(user.clone()).await.unwrap();
	auth_repo
		.query_store_otp(user.email.clone(), otp)
		.await
		.unwrap();
	let payload = json!({
		"email": user.email,
		"otp": otp
	});
	let res = server.post("/v1/auth/verify-email").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::OK);
	let body = res.text();
	assert!(body.contains("Email verified successfully"));
}

#[tokio::test]
async fn test_verify_email_should_fail_with_wrong_otp() {
	let state = create_mock_app_state().await;
	let server = TestServer::new(
		axum::Router::new()
			.nest("/v1/auth", auth_router())
			.layer(Extension(state.clone())),
	)
	.unwrap();
	let role_repo = RolesRepository::new(&state);
	let role_id = role_repo
		.query_role_by_name("Student".to_string())
		.await
		.unwrap()
		.id;
	let user = create_test_user("wrongotp@example.com", "Wrong OTP", false, &role_id);
	let otp = OtpManager::generate_otp();
	let user_repo = crate::apps::v1::users::UsersRepository::new(&state);
	let auth_repo = crate::apps::v1::auth::AuthRepository::new(&state);
	user_repo.query_create_user(user.clone()).await.unwrap();
	auth_repo
		.query_store_otp(user.email.clone(), otp)
		.await
		.unwrap();
	let payload = json!({
		"email": user.email,
		"otp": 999_999
	});
	let res = server.post("/v1/auth/verify-email").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
	let body = res.text();
	assert!(body.contains("Failed to verify OTP"));
}

#[tokio::test]
async fn test_verify_email_should_fail_if_otp_expired() {
	let state = create_mock_app_state().await;
	let server = TestServer::new(
		axum::Router::new()
			.nest("/v1/auth", auth_router())
			.layer(Extension(state.clone())),
	)
	.unwrap();
	let role_repo = RolesRepository::new(&state);
	let role_id = role_repo
		.query_role_by_name("Student".to_string())
		.await
		.unwrap()
		.id;
	let user = create_test_user("expired@example.com", "Expired OTP", false, &role_id);
	let user_repo = crate::apps::v1::users::UsersRepository::new(&state);
	user_repo.query_create_user(user.clone()).await.unwrap();
	use chrono::{Duration, Utc};
	let expired_otp = crate::apps::v1::auth::AuthOtpSchema {
		otp: 123456,
		expires_at: Utc::now() - Duration::minutes(1),
	};
	let _: Option<AuthOtpSchema> = state
		.surrealdb_mem
		.create((ResourceEnum::OtpCache.to_string(), user.email.clone()))
		.content(expired_otp)
		.await
		.unwrap();
	let payload = json!({
		"email": user.email,
		"otp": 123456
	});
	let res = server.post("/v1/auth/verify-email").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
	let body = res.text();
	assert!(body.contains("OTP expired"));
}

#[tokio::test]
async fn test_verify_email_should_fail_with_empty_payload() {
	let state = create_mock_app_state().await;
	let server = TestServer::new(
		axum::Router::new()
			.nest("/v1/auth", auth_router())
			.layer(Extension(state)),
	)
	.unwrap();
	let res = server.post("/v1/auth/verify-email").json(&json!({})).await;
	assert_eq!(res.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_new_password_should_succeed() {
	let state = create_mock_app_state().await;
	let server = TestServer::new(
		axum::Router::new()
			.nest("/v1/auth", auth_router())
			.layer(Extension(state.clone())),
	)
	.unwrap();
	let email = "resetme@example.com";
	let repo = crate::apps::v1::users::UsersRepository::new(&state);
	let role_repo = RolesRepository::new(&state);
	let role_id = role_repo
		.query_role_by_name("Student".to_string())
		.await
		.unwrap()
		.id;
	let mut user = create_test_user(email, "Reset User", true, &role_id);
	user.password = crate::hash_password("oldpass123!").unwrap();
	repo.query_create_user(user).await.unwrap();
	let token = crate::encode_reset_password_token(email.to_string()).unwrap();
	let payload = json!({
		"token": token,
		"password": "Newpass123!"
	});
	let res = server.post("/v1/auth/new-password").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::OK);
	let body = res.text();
	assert!(body.contains("Success update password user"));
}

#[tokio::test]
async fn test_new_password_should_fail_if_token_is_empty() {
	let state = create_mock_app_state().await;
	let server = TestServer::new(
		axum::Router::new()
			.nest("/v1/auth", auth_router())
			.layer(Extension(state)),
	)
	.unwrap();
	let payload = json!({
		"token": "",
		"password": "Newpass123!"
	});
	let res = server.post("/v1/auth/new-password").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
	let body = res.text();
	assert!(
		body.contains("Invalid or missing token"),
		"Gagal deteksi token kosong: {body}"
	);
}

#[tokio::test]
async fn test_new_password_should_fail_if_password_is_weak() {
	let state = create_mock_app_state().await;
	let server = TestServer::new(
		axum::Router::new()
			.nest("/v1/auth", auth_router())
			.layer(Extension(state)),
	)
	.unwrap();
	let email = "weakpass@example.com";
	let token = crate::encode_reset_password_token(email.to_string()).unwrap();
	let payload = json!({
		"token": token,
		"password": "123"
	});
	let res = server.post("/v1/auth/new-password").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
	let body = res.text();
	assert!(
		body.contains("Password must have at least")
			|| body.contains("Password must include"),
		"Pesan error nggak sesuai: {body}"
	);
}

#[tokio::test]
async fn test_new_password_should_fail_if_user_not_found() {
	let state = create_mock_app_state().await;
	let server = TestServer::new(
		axum::Router::new()
			.nest("/v1/auth", auth_router())
			.layer(Extension(state)),
	)
	.unwrap();
	let token =
		crate::encode_reset_password_token("ghost@example.com".into()).unwrap();
	let payload = json!({
		"token": token,
		"password": "Validpass123!"
	});
	let res = server.post("/v1/auth/new-password").json(&payload).await;
	assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
	let body = res.text();
	assert!(body.contains("User not found"));
}

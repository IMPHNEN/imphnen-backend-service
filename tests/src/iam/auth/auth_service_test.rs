#[cfg(test)]
mod tests {
	use crate::{generate_unique_email, get_role_id, UsersRepository};
	use imphnen_iam::{
		AuthLoginRequestDto, AuthLoginResponsetDto, AuthNewPasswordRequestDto,
		AuthRefreshTokenRequestDto, AuthRegisterRequestDto, AuthResendOtpRequestDto,
		AuthVerifyEmailRequestDto, MessageResponseDto, ResponseSuccessDto, TokenDto,
	};
	use imphnen_utils::{make_thing_from_enum, ResourceEnum as UtilsResourceEnum};
	use axum::{http::StatusCode, response::Response};
	use uuid::Uuid;

	#[tokio::test]
	async fn test_mutation_login_service() {
		let app_state = crate::get_app_state().await;
		let repo = UsersRepository::new(&app_state);
		let role_id = get_role_id("user", &app_state).await;

		// Test data
		let email = generate_unique_email("test_login_service");
		let password = "password123".to_string();

		// Create test user first
		let user_schema = imphnen_iam::UsersSchema {
			id: make_thing_from_enum(UtilsResourceEnum::Users, &Uuid::new_v4().to_string()),
			email: email.clone(),
			fullname: "Test User Service".to_string(),
			password: imphnen_utils::hash_password(&password).unwrap(),
			phone_number: Some("+1234567890".to_string()),
			is_active: true,
			role: role_id,
			..Default::default()
		};

		let create_response = repo.query_create_user(user_schema).await;
		assert!(create_response.is_ok());

		// Try to login
		let login_request = AuthLoginRequestDto {
			email: email.clone(),
			password: password.clone(),
		};

		let response = imphnen_iam::AuthService::mutation_login(login_request, &app_state).await;

	// Verify response
	assert_eq!(response.status(), StatusCode::OK);

	let login_response: ResponseSuccessDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert!(login_response.data.is_some());

		// Clean up
		let user = repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_mutation_login_service_invalid_credentials() {
		let app_state = crate::get_app_state().await;

		// Test data
		let email = generate_unique_email("test_login_invalid_service");
		let password = "wrongpassword".to_string();

		// Try to login with non-existent user
		let login_request = AuthLoginRequestDto {
			email: email.clone(),
			password: password.clone(),
		};

		let response = imphnen_iam::AuthService::mutation_login(login_request, &app_state).await;

	// Verify response
	assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

	let error_response: MessageResponseDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert!(error_response.message.contains("Email or password not correct"));
	}

	#[tokio::test]
	async fn test_mutation_register_service() {
		let app_state = crate::get_app_state().await;

		// Test data
		let email = generate_unique_email("test_register_service");
		let password = "password123".to_string();
		let register_request = AuthRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test User Service".to_string(),
			phone_number: Some("+1234567890".to_string()),
		};

		// Register user
		let response = imphnen_iam::AuthService::mutation_register(register_request, &app_state).await;

	// Verify response
	assert_eq!(response.status(), StatusCode::CREATED);

	let response_data: MessageResponseDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert_eq!(response_data.message, "User registered successfully, please check your email for OTP verification");

		// Verify user was created in database (should be inactive until OTP verification)
		let repo = UsersRepository::new(&app_state);
		let created_user = repo.query_user_by_email(email.clone()).await.unwrap();
		assert_eq!(created_user.email, email);
		assert_eq!(created_user.is_active, false);

		// Clean up
		let _ = repo.query_delete_user(created_user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_mutation_verify_email_service() {
		let app_state = crate::get_app_state().await;
		let repo = UsersRepository::new(&app_state);

		// Test data
		let email = generate_unique_email("test_verify_email_service");
		let password = "password123".to_string();
		let register_request = AuthRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test User Service".to_string(),
			phone_number: Some("+1234567890".to_string()),
		};

		// Register user first
		let register_response = imphnen_iam::AuthService::mutation_register(register_request, &app_state).await;
		assert_eq!(register_response.status(), StatusCode::CREATED);

		// Verify email
		let verify_request = AuthVerifyEmailRequestDto {
			email: email.clone(),
			otp: "123456".to_string(), // Default test OTP
		};

		let response = imphnen_iam::AuthService::mutation_verify_email(verify_request, &app_state).await;

	// Verify response
	assert_eq!(response.status(), StatusCode::OK);

	let response_data: MessageResponseDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert_eq!(response_data.message, "Email verified successfully");

		// Verify user was activated in database
		let updated_user = repo.query_user_by_email(email.clone()).await.unwrap();
		assert_eq!(updated_user.is_active, true);

		// Clean up
		let _ = repo.query_delete_user(updated_user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_mutation_resend_otp_service() {
		let app_state = crate::get_app_state().await;
		let repo = UsersRepository::new(&app_state);

		// Test data
		let email = generate_unique_email("test_resend_otp_service");
		let password = "password123".to_string();
		let register_request = AuthRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test User Service".to_string(),
			phone_number: Some("+1234567890".to_string()),
		};

		// Register user first
		let register_response = imphnen_iam::AuthService::mutation_register(register_request, &app_state).await;
		assert_eq!(register_response.status(), StatusCode::CREATED);

		// Resend OTP
		let resend_request = AuthResendOtpRequestDto {
			email: email.clone(),
		};

		let response = imphnen_iam::AuthService::mutation_resend_otp(resend_request, &app_state).await;

	// Verify response
	assert_eq!(response.status(), StatusCode::OK);

	let response_data: MessageResponseDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert_eq!(response_data.message, "OTP resent successfully");

		// Clean up
		let user = repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_mutation_forgot_password_service() {
		let app_state = crate::get_app_state().await;
		let repo = UsersRepository::new(&app_state);

		// Test data
		let email = generate_unique_email("test_forgot_password_service");
		let password = "password123".to_string();
		let register_request = AuthRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test User Service".to_string(),
			phone_number: Some("+1234567890".to_string()),
		};

		// Register and verify user first
		let register_response = imphnen_iam::AuthService::mutation_register(register_request, &app_state).await;
		assert_eq!(register_response.status(), StatusCode::CREATED);

		let verify_request = AuthVerifyEmailRequestDto {
			email: email.clone(),
			otp: "123456".to_string(),
		};
		let verify_response = imphnen_iam::AuthService::mutation_verify_email(verify_request, &app_state).await;
		assert_eq!(verify_response.status(), StatusCode::OK);

		// Forgot password
		let forgot_request = AuthResendOtpRequestDto {
			email: email.clone(),
		};

		let response = imphnen_iam::AuthService::mutation_forgot_password(forgot_request, &app_state).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		let response_data: MessageResponseDto = crate::common::response_helpers::parse_response(response, 8192).await;
		assert_eq!(
			response_data.message,
			"If your email is registered, you will receive a password reset link."
		);

		// Clean up
		let user = repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_mutation_new_password_service() {
		let app_state = crate::get_app_state().await;
		let repo = UsersRepository::new(&app_state);

		// Test data
		let email = generate_unique_email("test_new_password_service");
		let old_password = "password123".to_string();
		let new_password = "newpassword456".to_string();
		
		// Register and verify user first
		let register_request = AuthRegisterRequestDto {
			email: email.clone(),
			password: old_password.clone(),
			fullname: "Test User Service".to_string(),
			phone_number: Some("+1234567890".to_string()),
		};

		let register_response = imphnen_iam::AuthService::mutation_register(register_request, &app_state).await;
		assert_eq!(register_response.status(), StatusCode::CREATED);

		let verify_request = AuthVerifyEmailRequestDto {
			email: email.clone(),
			otp: "123456".to_string(),
		};
		let verify_response = imphnen_iam::AuthService::mutation_verify_email(verify_request, &app_state).await;
		assert_eq!(verify_response.status(), StatusCode::OK);

		// In a real test, we would extract the password reset token from the email, but for testing
		// we'll simulate this by creating a reset token directly
		let user = repo.query_user_by_email(email.clone()).await.unwrap();
		let reset_token = imphnen_utils::encode_reset_password_token(user.email.clone(), user.id.id.to_raw()).unwrap();

		// Set new password
		let new_password_request = AuthNewPasswordRequestDto {
			token: reset_token,
			password: new_password.clone(),
		};

		let response = imphnen_iam::AuthService::mutation_new_password(new_password_request, &app_state).await;

	// Verify response
	assert_eq!(response.status(), StatusCode::OK);

	let response_data: MessageResponseDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert_eq!(response_data.message, "Password updated successfully");

		// Verify password was updated in database
		let updated_user = repo.query_user_by_email(email.clone()).await.unwrap();
		let is_password_correct = imphnen_utils::verify_password(&new_password, &updated_user.password).unwrap();
		assert!(is_password_correct);

		// Clean up
		let _ = repo.query_delete_user(updated_user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_mutation_refresh_token_service() {
		let app_state = crate::get_app_state().await;
		let repo = UsersRepository::new(&app_state);

		// Test data
		let email = generate_unique_email("test_refresh_token_service");
		let password = "password123".to_string();
		
		// Register and verify user first
		let register_request = AuthRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test User Service".to_string(),
			phone_number: Some("+1234567890".to_string()),
		};

		let register_response = imphnen_iam::AuthService::mutation_register(register_request, &app_state).await;
		assert_eq!(register_response.status(), StatusCode::CREATED);

		let verify_request = AuthVerifyEmailRequestDto {
			email: email.clone(),
			otp: "123456".to_string(),
		};
		let verify_response = imphnen_iam::AuthService::mutation_verify_email(verify_request, &app_state).await;
		assert_eq!(verify_response.status(), StatusCode::OK);

		// Login to get tokens
		let login_request = AuthLoginRequestDto {
			email: email.clone(),
			password: password.clone(),
		};
		
	let login_response = imphnen_iam::AuthService::mutation_login(login_request, &app_state).await;
        
	let login_response_data: ResponseSuccessDto = crate::common::response_helpers::parse_response(login_response, 8192).await;
	let refresh_token = login_response_data.data.as_ref().unwrap().token.refresh_token.clone();

		// Refresh token
		let refresh_request = AuthRefreshTokenRequestDto {
			refresh_token: refresh_token,
		};

		let response = imphnen_iam::AuthService::mutation_refresh_token(refresh_request, &app_state).await;

	// Verify response
	assert_eq!(response.status(), StatusCode::OK);

	let response_data: ResponseSuccessDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert!(response_data.data.is_some());
	let token_data = response_data.data.as_ref().unwrap();
	assert!(token_data.access_token.is_some());
	assert!(token_data.refresh_token.is_some());

		// Clean up
		let user = repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_mutation_mentor_login_service() {
		let app_state = crate::get_app_state().await;
		let repo = UsersRepository::new(&app_state);
		let role_id = get_role_id("mentor", &app_state).await;

		// Test data
		let email = generate_unique_email("test_mentor_login_service");
		let password = "password123".to_string();

		// Create test mentor user first
		let user_schema = imphnen_iam::UsersSchema {
			id: make_thing_from_enum(UtilsResourceEnum::Users, &Uuid::new_v4().to_string()),
			email: email.clone(),
			fullname: "Test Mentor Service".to_string(),
			password: imphnen_utils::hash_password(&password).unwrap(),
			phone_number: Some("+1234567890".to_string()),
			is_active: true,
			role: role_id,
			..Default::default()
		};

		let create_response = repo.query_create_user(user_schema).await;
		assert!(create_response.is_ok());

		// Try to login as mentor
		let login_request = AuthLoginRequestDto {
			email: email.clone(),
			password: password.clone(),
		};

		let response = imphnen_iam::AuthService::mutation_mentor_login(login_request, &app_state).await;

	// Verify response
	assert_eq!(response.status(), StatusCode::OK);

	let login_response: ResponseSuccessDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert!(login_response.data.is_some());

		// Clean up
		let user = repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_mutation_mentor_login_service_non_mentor() {
		let app_state = crate::get_app_state().await;
		let repo = UsersRepository::new(&app_state);
		let role_id = get_role_id("user", &app_state).await;

		// Test data
		let email = generate_unique_email("test_non_mentor_login_service");
		let password = "password123".to_string();

		// Create test user (not mentor) first
		let user_schema = imphnen_iam::UsersSchema {
			id: make_thing_from_enum(UtilsResourceEnum::Users, &Uuid::new_v4().to_string()),
			email: email.clone(),
			fullname: "Test User Service".to_string(),
			password: imphnen_utils::hash_password(&password).unwrap(),
			phone_number: Some("+1234567890".to_string()),
			is_active: true,
			role: role_id,
			..Default::default()
		};

		let create_response = repo.query_create_user(user_schema).await;
		assert!(create_response.is_ok());

		// Try to login as mentor (should fail)
		let login_request = AuthLoginRequestDto {
			email: email.clone(),
			password: password.clone(),
		};

		let response = imphnen_iam::AuthService::mutation_mentor_login(login_request, &app_state).await;

	// Verify response
	assert_eq!(response.status(), StatusCode::FORBIDDEN);

	let error_response: MessageResponseDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert_eq!(error_response.message, "User does not have mentor privileges");

		// Clean up
		let user = repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = repo.query_delete_user(user.id.id.to_raw()).await;
	#[tokio::test]
	async fn test_mutation_login_service_inactive_user() {
		let app_state = crate::get_app_state().await;
		let repo = UsersRepository::new(&app_state);
		let role_id = get_role_id("user", &app_state).await;

		// Test data
		let email = generate_unique_email("test_login_inactive_service");
		let password = "password123".to_string();

		// Create inactive test user
		let user_schema = imphnen_iam::UsersSchema {
			id: make_thing_from_enum(UtilsResourceEnum::Users, &Uuid::new_v4().to_string()),
			email: email.clone(),
			fullname: "Test User Service".to_string(),
			password: imphnen_utils::hash_password(&password).unwrap(),
			phone_number: Some("+1234567890".to_string()),
			is_active: false, // Inactive
			role: role_id,
			..Default::default()
		};

		let create_response = repo.query_create_user(user_schema).await;
		assert!(create_response.is_ok());

		// Try to login
		let login_request = AuthLoginRequestDto {
			email: email.clone(),
			password: password.clone(),
		};

		let response = imphnen_iam::AuthService::mutation_login(login_request, &app_state).await;

	// Verify response
	assert_eq!(response.status(), StatusCode::BAD_REQUEST);

	let error_response: MessageResponseDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert!(error_response.message.contains("Account not active"));

		// Clean up
		let user = repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_mutation_register_service_existing_email() {
		let app_state = crate::get_app_state().await;
		let repo = UsersRepository::new(&app_state);
		let role_id = get_role_id("user", &app_state).await;

		// Test data
		let email = generate_unique_email("test_register_existing_service");
		let password = "password123".to_string();

		// Create existing user
		let user_schema = imphnen_iam::UsersSchema {
			id: make_thing_from_enum(UtilsResourceEnum::Users, &Uuid::new_v4().to_string()),
			email: email.clone(),
			fullname: "Existing User".to_string(),
			password: imphnen_utils::hash_password(&password).unwrap(),
			phone_number: Some("+1234567890".to_string()),
			is_active: true,
			role: role_id,
			..Default::default()
		};

		let create_response = repo.query_create_user(user_schema).await;
		assert!(create_response.is_ok());

		// Try to register with same email
		let register_request = AuthRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "New User".to_string(),
			phone_number: Some("+1234567891".to_string()),
		};

		let response = imphnen_iam::AuthService::mutation_register(register_request, &app_state).await;

	// Verify response
	assert_eq!(response.status(), StatusCode::BAD_REQUEST);

	let error_response: MessageResponseDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert_eq!(error_response.message, "User already exists");

		// Clean up
		let user = repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_mutation_register_service_invalid_data() {
		let app_state = crate::get_app_state().await;

		// Test data with invalid inputs
		let register_request = AuthRegisterRequestDto {
			email: "invalid-email".to_string(), // Invalid email
			password: "123".to_string(), // Too short
			fullname: "".to_string(), // Empty
			phone_number: Some("".to_string()), // Empty
		};

		let response = imphnen_iam::AuthService::mutation_register(register_request, &app_state).await;

		// Verify response - should fail validation
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	async fn test_mutation_verify_email_service_wrong_otp() {
		let app_state = crate::get_app_state().await;
		let repo = UsersRepository::new(&app_state);

		// Test data
		let email = generate_unique_email("test_verify_wrong_otp_service");
		let password = "password123".to_string();
		let register_request = AuthRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test User Service".to_string(),
			phone_number: Some("+1234567890".to_string()),
		};

		// Register user first
		let register_response = imphnen_iam::AuthService::mutation_register(register_request, &app_state).await;
		assert_eq!(register_response.status(), StatusCode::CREATED);

		// Verify email with wrong OTP
		let verify_request = AuthVerifyEmailRequestDto {
			email: email.clone(),
			otp: "999999".to_string(), // Wrong OTP
		};

		let response = imphnen_iam::AuthService::mutation_verify_email(verify_request, &app_state).await;

	// Verify response
	assert_eq!(response.status(), StatusCode::BAD_REQUEST);

	let response_data: MessageResponseDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert_eq!(response_data.message, "Failed to verify OTP");

		// Verify user still inactive
		let user = repo.query_user_by_email(email.clone()).await.unwrap();
		assert_eq!(user.is_active, false);

		// Clean up
		let _ = repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_mutation_verify_email_service_already_active() {
		let app_state = crate::get_app_state().await;
		let repo = UsersRepository::new(&app_state);

		// Test data
		let email = generate_unique_email("test_verify_already_active_service");
		let password = "password123".to_string();
		let register_request = AuthRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test User Service".to_string(),
			phone_number: Some("+1234567890".to_string()),
		};

		// Register and verify user first
		let register_response = imphnen_iam::AuthService::mutation_register(register_request, &app_state).await;
		assert_eq!(register_response.status(), StatusCode::CREATED);

		let verify_request = AuthVerifyEmailRequestDto {
			email: email.clone(),
			otp: "123456".to_string(),
		};
		let verify_response = imphnen_iam::AuthService::mutation_verify_email(verify_request.clone(), &app_state).await;
		assert_eq!(verify_response.status(), StatusCode::OK);

		// Try to verify again
		let response = imphnen_iam::AuthService::mutation_verify_email(verify_request, &app_state).await;

	// Verify response
	assert_eq!(response.status(), StatusCode::BAD_REQUEST);

	let response_data: MessageResponseDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert_eq!(response_data.message, "User already active");

		// Clean up
		let user = repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_mutation_resend_otp_service_non_existent_user() {
		let app_state = crate::get_app_state().await;

		// Test data
		let email = generate_unique_email("test_resend_otp_non_existent_service");

		// Resend OTP for non-existent user
		let resend_request = AuthResendOtpRequestDto {
			email: email.clone(),
		};

		let response = imphnen_iam::AuthService::mutation_resend_otp(resend_request, &app_state).await;

	// Verify response
	assert_eq!(response.status(), StatusCode::BAD_REQUEST);

	let response_data: MessageResponseDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert_eq!(response_data.message, "User not found");
	}

	#[tokio::test]
	async fn test_mutation_forgot_password_service_non_existent_user() {
		let app_state = crate::get_app_state().await;

		// Test data
		let email = generate_unique_email("test_forgot_password_non_existent_service");

		// Forgot password for non-existent user
		let forgot_request = AuthResendOtpRequestDto {
			email: email.clone(),
		};

		let response = imphnen_iam::AuthService::mutation_forgot_password(forgot_request, &app_state).await;

		// Verify response - should still return success for security
		assert_eq!(response.status(), StatusCode::OK);

		let response_data: MessageResponseDto = crate::common::response_helpers::parse_response(response, 8192).await;
		assert_eq!(
			response_data.message,
			"If your email is registered, you will receive a password reset link."
		);
	}

	#[tokio::test]
	async fn test_mutation_new_password_service_invalid_token() {
		let app_state = crate::get_app_state().await;

		// Test data
		let new_password = "newpassword456".to_string();

		// Set new password with invalid token
		let new_password_request = AuthNewPasswordRequestDto {
			token: "invalid_token".to_string(),
			password: new_password.clone(),
		};

		let response = imphnen_iam::AuthService::mutation_new_password(new_password_request, &app_state).await;

	// Verify response
	assert_eq!(response.status(), StatusCode::BAD_REQUEST);

	let response_data: MessageResponseDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert_eq!(response_data.message, "Invalid or missing token");
	}

	#[tokio::test]
	async fn test_mutation_refresh_token_service_invalid_token() {
		let app_state = crate::get_app_state().await;

		// Refresh token with invalid token
		let refresh_request = AuthRefreshTokenRequestDto {
			refresh_token: "invalid_refresh_token".to_string(),
		};

		let response = imphnen_iam::AuthService::mutation_refresh_token(refresh_request, &app_state).await;

	// Verify response
	assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

	let error_response: MessageResponseDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert_eq!(error_response.message, "Invalid refresh token");
	}

	#[tokio::test]
	async fn test_mutation_mentor_login_service_inactive_user() {
		let app_state = crate::get_app_state().await;
		let repo = UsersRepository::new(&app_state);
		let role_id = get_role_id("mentor", &app_state).await;

		// Test data
		let email = generate_unique_email("test_mentor_login_inactive_service");
		let password = "password123".to_string();

		// Create inactive mentor user
		let user_schema = imphnen_iam::UsersSchema {
			id: make_thing_from_enum(UtilsResourceEnum::Users, &Uuid::new_v4().to_string()),
			email: email.clone(),
			fullname: "Test Mentor Service".to_string(),
			password: imphnen_utils::hash_password(&password).unwrap(),
			phone_number: Some("+1234567890".to_string()),
			is_active: false, // Inactive
			role: role_id,
			..Default::default()
		};

		let create_response = repo.query_create_user(user_schema).await;
		assert!(create_response.is_ok());

		// Try to login as mentor
		let login_request = AuthLoginRequestDto {
			email: email.clone(),
			password: password.clone(),
		};

		let response = imphnen_iam::AuthService::mutation_mentor_login(login_request, &app_state).await;

	// Verify response
	assert_eq!(response.status(), StatusCode::BAD_REQUEST);

	let error_response: MessageResponseDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert!(error_response.message.contains("Account not active"));

		// Clean up
		let user = repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = repo.query_delete_user(user.id.id.to_raw()).await;
	}
	}
}
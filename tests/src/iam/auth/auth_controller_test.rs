#[cfg(test)]
mod tests {
	use crate::{generate_unique_email, get_role_id, UsersRepository};
	use axum::{http::StatusCode, response::Response};
	use imphnen_iam::{
		AuthLoginRequestDto, AuthLoginResponsetDto, AuthNewPasswordRequestDto,
		AuthRefreshTokenRequestDto, AuthRegisterRequestDto, AuthResendOtpRequestDto,
		AuthVerifyEmailRequestDto, MessageResponseDto, ResponseSuccessDto, TokenDto,
	};
	use imphnen_utils::{make_thing_from_enum, ResourceEnum as UtilsResourceEnum};
	use uuid::Uuid;

	#[tokio::test]
	async fn test_post_login_controller() {
		let app_state = crate::get_app_state().await;
		let repo = UsersRepository::new(&app_state);
		let role_id = get_role_id("user", &app_state).await;

		// Test data
		let email = generate_unique_email("test_login_controller");
		let password = "password123".to_string();

		// Create test user first
		let user_request = AuthRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test User Controller".to_string(),
			phone_number: Some("+1234567890".to_string()),
		};

		let register_response = imphnen_iam::AuthController::mutation_register(
			&app_state,
			user_request,
		)
		.await;

		assert_eq!(register_response.status(), StatusCode::CREATED);

		// Wait for email verification (simulate OTP verification in test)
		let verify_request = AuthVerifyEmailRequestDto {
			email: email.clone(),
			otp: "123456".to_string(), // Default test OTP
		};

		let verify_response = imphnen_iam::AuthController::mutation_verify_email(
			&app_state,
			verify_request,
		)
		.await;

		assert_eq!(verify_response.status(), StatusCode::OK);

		// Try to login
		let login_request = AuthLoginRequestDto {
			email: email.clone(),
			password: password.clone(),
		};

		let response = imphnen_iam::AuthController::mutation_login(
			&app_state,
			login_request,
		)
		.await;

	// Verify response
	assert_eq!(response.status(), StatusCode::OK);

	let login_response: ResponseSuccessDto = crate::common::response_helpers::parse_response(response, 8192).await;
	let data_val = login_response.data.expect("login should return data");
	// Try to deserialize token structure
	let token_obj: TokenDto = serde_json::from_value(data_val).expect("login data must be TokenDto");
	assert!(!token_obj.access_token.is_empty(), "access_token must be present");

		// Clean up
		let user = repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_post_login_controller_invalid_credentials() {
		let app_state = crate::get_app_state().await;

		// Test data
		let email = generate_unique_email("test_login_invalid");
		let password = "wrongpassword".to_string();

		// Try to login with non-existent user
		let login_request = AuthLoginRequestDto {
			email: email.clone(),
			password: password.clone(),
		};

		let response = imphnen_iam::AuthController::mutation_login(
			&app_state,
			login_request,
		)
		.await;

	// Verify response
	assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

	let error_response: MessageResponseDto = crate::common::response_helpers::parse_response(response, 8192).await;
	assert!(error_response.message.to_lowercase().contains("email or password") || error_response.message.to_lowercase().contains("not correct") || error_response.message.to_lowercase().contains("invalid credentials"));
	}
}
#[cfg(test)]
mod auth_login_tests {
	use crate::generate_unique_email;
	use crate::hash_password;
	use crate::mock_test::setup_all_test_environment;
	use axum::http::StatusCode;
	use imphnen_iam::{
		v1::auth::AuthLoginRequestDto,
		AppState, UsersRepository, UsersSchema,
	};
	use imphnen_iam::v1::auth::auth_service::AuthService;
	use imphnen_iam::v1::auth::AuthServiceTrait;
	use serde_json::Value; // Import the new setup function

	async fn setup_test_environment() -> AppState {
		setup_all_test_environment().await
	}

	async fn create_test_user_with_role(
		state: &AppState,
		email: &str,
		password: &str,
		role_name: &str,
		is_active: bool,
	) -> UsersSchema {
		let role_repo = imphnen_iam::RolesRepository::new(state);

		let role = match role_repo.query_role_by_name(role_name.to_string()).await {
			Ok(role) => role,
			Err(_) => {
				let _ = role_repo
					.query_create_role(imphnen_iam::RolesRequestCreateDto {
						name: role_name.to_string(),
						permissions: vec![],
					})
					.await
					.unwrap();

				role_repo
					.query_role_by_name(role_name.to_string())
					.await
					.unwrap_or_else(|_| {
						panic!("Failed to create {role_name} role");
					})
			}
		};

		let user = UsersSchema {
			id: crate::make_thing("app_users", &uuid::Uuid::new_v4().to_string()),
			email: email.to_string(),
			fullname: "Test User".to_string(),
			legal_name: None,
			password: hash_password(password).unwrap(),
			is_deleted: false,
			avatar: None,
			phone_number: "081234567890".to_string(),
			phone_for_verification: None,
			is_active,
			gender: None,
			birthdate: None,
			domicile: None,
			bio: None,
			last_education: None,
			linkedin_url: None,
			github_url: None,
			cv_url: None,
			portfolio_url: None,
			website_url: None,
			twitter_url: None,
			location: None,
			skills: None,
			experience: None,
			education: None,
			career_status: None,
			role: crate::make_thing("app_roles", &role.id),
			mentor_id: None,
			created_at: imphnen_utils::get_iso_date(),
			updated_at: imphnen_utils::get_iso_date(),
		};
		let user_repo = UsersRepository::new(state);
		user_repo
			.query_create_user(user.clone())
			.await
			.expect("Failed to create test user");

		user
	}

	#[tokio::test]
	async fn test_successful_login_with_valid_credentials() {
		let state = setup_test_environment().await;
		let email = generate_unique_email("test_login_success");
		let password = "password";

		create_test_user_with_role(&state, &email, password, "User", true).await;

		let login_dto = AuthLoginRequestDto {
			email: email.clone(),
			password: password.to_string(),
		};

		let response = AuthService::mutation_login(login_dto, &state).await; // Corrected call
		let (parts, body) = response.into_parts();

		assert_eq!(parts.status, StatusCode::OK);

		let resp = axum::http::Response::from_parts(parts, body);
		let response_json: Value = crate::common::response_helpers::parse_response_value(resp, usize::MAX).await;

		assert!(response_json.get("data").is_some());
		assert!(response_json["data"].get("token").is_some());
		assert!(response_json["data"]["token"].get("access_token").is_some());
		assert!(response_json["data"]["token"].get("refresh_token").is_some());
		assert!(response_json["data"].get("user").is_some());
		assert_eq!(response_json["data"]["user"]["email"], email);
	}

	#[tokio::test]
	async fn test_login_with_invalid_email_format() {
		let state = setup_test_environment().await;

		let login_dto = AuthLoginRequestDto {
			email: "invalid-email".to_string(),
			password: "password".to_string(),
		};

		let response = AuthService::mutation_login(login_dto, &state).await; // Corrected call
	let (parts, body) = response.into_parts();

	assert_eq!(parts.status, StatusCode::BAD_REQUEST);

	let resp = axum::http::Response::from_parts(parts, body);
	let response_json: Value = crate::common::response_helpers::parse_response_value(resp, usize::MAX).await;
	assert_eq!(response_json["message"], "Email not valid");
	}

	#[tokio::test]
	async fn test_login_with_empty_email() {
		let state = setup_test_environment().await;

		let login_dto = AuthLoginRequestDto {
			email: "".to_string(),
			password: "password".to_string(),
		};

		let response = AuthService::mutation_login(login_dto, &state).await; // Corrected call
	let (parts, body) = response.into_parts();

	assert_eq!(parts.status, StatusCode::BAD_REQUEST);
	let resp = axum::http::Response::from_parts(parts, body);
	let response_json: Value = crate::common::response_helpers::parse_response_value(resp, usize::MAX).await;
	let message = response_json["message"].as_str().unwrap();
	assert!(message.contains("Email cannot be empty"));
	assert!(message.contains("Email not valid"));
	}

	#[tokio::test]
	async fn test_login_with_empty_password() {
		let state = setup_test_environment().await;

		let login_dto = AuthLoginRequestDto {
			email: generate_unique_email("test_empty_pass"),
			password: "".to_string(),
		};

		let response = AuthService::mutation_login(login_dto, &state).await; // Corrected call
	let (parts, body) = response.into_parts();

	assert_eq!(parts.status, StatusCode::BAD_REQUEST);
	let resp = axum::http::Response::from_parts(parts, body);
	let response_json: Value = crate::common::response_helpers::parse_response_value(resp, usize::MAX).await;
	assert_eq!(response_json["message"], "Password cannot be empty");
	}

	#[tokio::test]
	async fn test_login_with_wrong_password() {
		let state = setup_test_environment().await;
		let email = generate_unique_email("test_wrong_pass");
		let correct_password = "password";

		create_test_user_with_role(&state, &email, correct_password, "User", true).await;

		let login_dto = AuthLoginRequestDto {
			email: email.clone(),
			password: "WrongPassword123!".to_string(),
		};

		let response = AuthService::mutation_login(login_dto, &state).await; // Corrected call
	let (parts, body) = response.into_parts();

	assert_eq!(parts.status, StatusCode::BAD_REQUEST);
	let resp = axum::http::Response::from_parts(parts, body);
	let response_json: Value = crate::common::response_helpers::parse_response_value(resp, usize::MAX).await;

	assert_eq!(response_json["message"], "Email or password not correct");
	}

	#[tokio::test]
	async fn test_login_with_nonexistent_user() {
		let state = setup_test_environment().await;

		let login_dto = AuthLoginRequestDto {
			email: generate_unique_email("nonexistent"),
			password: "password".to_string(),
		};

		let response = AuthService::mutation_login(login_dto, &state).await; // Corrected call
		let (parts, body) = response.into_parts();

		assert_eq!(parts.status, StatusCode::UNAUTHORIZED);
		let resp = axum::http::Response::from_parts(parts, body);
		let response_json: Value = crate::common::response_helpers::parse_response_value(resp, usize::MAX).await;

		assert!(response_json["message"].to_string().contains("User not found"));
	}

	#[tokio::test]
	async fn test_login_with_inactive_user() {
		let state = setup_test_environment().await;
		let email = generate_unique_email("test_inactive");
		let password = "password";

		create_test_user_with_role(&state, &email, password, "User", false).await;

		let login_dto = AuthLoginRequestDto {
			email: email.clone(),
			password: password.to_string(),
		};

		let response = AuthService::mutation_login(login_dto, &state).await; // Corrected call
		let (parts, body) = response.into_parts();

		assert_eq!(parts.status, StatusCode::BAD_REQUEST);
		let resp = axum::http::Response::from_parts(parts, body);
		let response_json: Value = crate::common::response_helpers::parse_response_value(resp, usize::MAX).await;

		assert_eq!(response_json["message"], "Account not active, please verify your email");
	}

	#[tokio::test]
	async fn test_successful_mentor_login() {
		let state = setup_test_environment().await;
		let email = generate_unique_email("test_mentor_login");
		let password = "password";

		create_test_user_with_role(&state, &email, password, "Mentor", true).await;

		let login_dto = AuthLoginRequestDto {
			email: email.clone(),
			password: password.to_string(),
		};

		let response = AuthService::mutation_mentor_login(login_dto, &state).await; // Corrected call
	let (parts, body) = response.into_parts();

	assert_eq!(parts.status, StatusCode::OK);
	let resp = axum::http::Response::from_parts(parts, body);
	let response_json: Value = crate::common::response_helpers::parse_response_value(resp, usize::MAX).await;

	assert!(response_json.get("data").is_some());
	assert_eq!(response_json["data"]["user"]["role"]["name"], "Mentor");
	}

	#[tokio::test]
	async fn test_mentor_login_with_non_mentor_user() {
		let state = setup_test_environment().await;
		let email = generate_unique_email("test_user_not_mentor");
		let password = "password";

		create_test_user_with_role(&state, &email, password, "User", true).await;

		let login_dto = AuthLoginRequestDto {
			email: email.clone(),
			password: password.to_string(),
		};

		let response = AuthService::mutation_mentor_login(login_dto, &state).await; // Corrected call
		let (parts, body) = response.into_parts();

		assert_eq!(parts.status, StatusCode::FORBIDDEN);
		let resp = axum::http::Response::from_parts(parts, body);
		let response_json: Value = crate::common::response_helpers::parse_response_value(resp, usize::MAX).await;

		assert_eq!(response_json["message"], "User does not have mentor privileges");
	}

	#[tokio::test]
	async fn test_mentor_login_with_inactive_mentor() {
		let state = setup_test_environment().await;
		let email = generate_unique_email("test_inactive_mentor");
		let password = "password";

		create_test_user_with_role(&state, &email, password, "Mentor", false).await;

		let login_dto = AuthLoginRequestDto {
			email: email.clone(),
			password: password.to_string(),
		};

		let response = AuthService::mutation_mentor_login(login_dto, &state).await; // Corrected call
		let (parts, body) = response.into_parts();

		assert_eq!(parts.status, StatusCode::BAD_REQUEST);
		let resp = axum::http::Response::from_parts(parts, body);
		let response_json: Value = crate::common::response_helpers::parse_response_value(resp, usize::MAX).await;

		assert_eq!(response_json["message"], "Account not active, please verify your email");
	}

	#[tokio::test]
	async fn test_login_creates_user_cache() {
		let state = setup_test_environment().await;
		let email = generate_unique_email("test_cache");
		let password = "password";

		create_test_user_with_role(&state, &email, password, "User", true).await;

		let login_dto = AuthLoginRequestDto {
			email: email.clone(),
			password: password.to_string(),
		};

		let response = AuthService::mutation_login(login_dto, &state).await; // Corrected call
		let (parts, _) = response.into_parts();

		assert_eq!(parts.status, StatusCode::OK);

		// Verify user was cached
		let auth_repo = imphnen_iam::AuthRepository::new(state.surrealdb_mem.clone());
		let cached_user = auth_repo.query_get_stored_user(email.clone()).await;
		assert!(cached_user.is_ok());
		assert_eq!(cached_user.unwrap().email, email);
	}

	#[tokio::test]
	async fn test_login_with_special_characters_in_email() {
		let state = setup_test_environment().await;
		let email = generate_unique_email("test+special");
		let password = "password";

		create_test_user_with_role(&state, &email, password, "User", true).await;

		let login_dto = AuthLoginRequestDto {
			email: email.clone(),
			password: password.to_string(),
		};

		let response = AuthService::mutation_login(login_dto, &state).await; // Corrected call
		let (parts, _) = response.into_parts();

		assert_eq!(parts.status, StatusCode::OK);
	}

	#[tokio::test]
	async fn test_login_with_case_sensitive_email() {
		let state = setup_test_environment().await;
		let email = generate_unique_email("test_case");
		let password = "password";

		create_test_user_with_role(&state, &email, password, "User", true).await;

		let login_dto = AuthLoginRequestDto {
			email: email.to_uppercase(),
			password: password.to_string(),
		};

		let response = AuthService::mutation_login(login_dto, &state).await; // Corrected call
		let (parts, body) = response.into_parts();

		// Email should be case-sensitive
		assert_eq!(parts.status, StatusCode::UNAUTHORIZED);
		let resp = axum::http::Response::from_parts(parts, body);
		let response_json: Value = crate::common::response_helpers::parse_response_value(resp, usize::MAX).await;
		assert!(response_json["message"].to_string().contains("User not found"));
	}
}

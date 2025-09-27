#[cfg(test)]
mod tests {
	use crate::{generate_unique_email, get_role_id, UsersRepository};
	use axum::http::StatusCode;
	use imphnen_entities::MessageResponseDto;
	use imphnen_iam::MetaRequestDto;
	use imphnen_iam::v1::users::{UsersCreateRequestDto, UsersSchema, UsersUpdateRequestDto};
	use imphnen_iam::v1::users::users_service::{UsersService, UsersServiceTrait};
	use imphnen_utils::{make_thing_from_enum, ResourceEnum as UtilsResourceEnum};
	use uuid::Uuid;

	#[tokio::test]
	async fn test_get_user_list_service() {
		let app_state = crate::get_app_state().await;

		// Get user list through service
		let meta = MetaRequestDto {
			page: Some(1),
			per_page: Some(10),
			..Default::default()
		};
		let response = UsersService::get_user_list(&app_state, meta).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);
	}

	#[tokio::test]
	async fn test_get_user_by_id_service_invalid_uuid() {
		let app_state = crate::get_app_state().await;

		// Use invalid UUID
		let invalid_id = "invalid-uuid".to_string();

		// Get user by ID through service
		let response = UsersService::get_user_by_id(&app_state, invalid_id).await;

		// Verify response - should fail validation
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	async fn test_get_user_by_id_service_not_found() {
		let app_state = crate::get_app_state().await;

		// Use valid but non-existent UUID
		let non_existent_id = Uuid::new_v4().to_string();

		// Get user by ID through service
		let response = UsersService::get_user_by_id(&app_state, non_existent_id).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn test_create_user_service() {
		let app_state = crate::get_app_state().await;
		let repo = UsersRepository::new(&app_state);
		let role_id = get_role_id("user", &app_state).await;

		// Test data
		let email = generate_unique_email("test_create_user_service");
		let password = "password123".to_string();
		let create_request = UsersCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test User Service".to_string(),
			phone_number: "+1234567890".to_string(),
			role_id: role_id.id.to_raw(),
			is_active: true,
			avatar: None,
		};

		// Create user through service
		let response = UsersService::create_user(&app_state, create_request).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::CREATED);

		// Verify user was created in database
		let created_user = repo.query_user_by_email(email.clone()).await.unwrap();
		assert_eq!(created_user.email, email);

		// Clean up
		let _ = repo.query_delete_user(created_user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_create_user_service_existing_email() {
		let app_state = crate::get_app_state().await;
		let repo = UsersRepository::new(&app_state);
		let role_id = get_role_id("user", &app_state).await;

		// Test data
		let email = generate_unique_email("test_create_existing_service");
		let password = "password123".to_string();

		// Create existing user
		let user_schema = UsersSchema {
			id: make_thing_from_enum(UtilsResourceEnum::Users, &Uuid::new_v4().to_string()),
			fullname: "Existing User".to_string(),
			legal_name: None,
			email: email.clone(),
			password: imphnen_utils::hash_password(&password).unwrap(),
			avatar: None,
			phone_number: "+1234567890".to_string(),
			phone_for_verification: None,
			is_active: true,
			is_deleted: false,
			mentor_id: None,
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
			role: role_id.clone(),
			created_at: "2023-01-01T00:00:00Z".to_string(),
			updated_at: "2023-01-01T00:00:00Z".to_string(),
		};

		let create_response = repo.query_create_user(user_schema).await;
		assert!(create_response.is_ok());

		// Try to create again with same email
		let create_request = UsersCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "New User".to_string(),
			phone_number: "+1234567891".to_string(),
			role_id: role_id.id.to_raw(),
			is_active: true,
			avatar: None,
		};

		let response = UsersService::create_user(&app_state, create_request).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);

		let body = response.into_body();
		let body_bytes = axum::body::to_bytes(body, 1024).await.unwrap();
		let error_response: MessageResponseDto = serde_json::from_slice(&body_bytes).unwrap();
		assert_eq!(error_response.message, "Email not valid");

		// Clean up
		let user = repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_update_user_service_invalid_uuid() {
		let app_state = crate::get_app_state().await;

		// Use invalid UUID
		let invalid_id = "invalid-uuid".to_string();

		// Prepare update request
		let update_request = UsersUpdateRequestDto {
			fullname: Some("Updated Name".to_string()),
			phone_number: None,
			is_active: None,
			avatar: None,
			bio: None,
			birthdate: None,
			gender: None,
			domicile: None,
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
			email: None,
			password: None,
			legal_name: None,
			phone_for_verification: None,
			role_id: None,
		};

		// Update user through service
		let response = UsersService::update_user(&app_state, invalid_id, update_request).await;

		// Verify response - should fail validation
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	async fn test_update_user_service_not_found() {
		let app_state = crate::get_app_state().await;

		// Use valid but non-existent UUID
		let non_existent_id = Uuid::new_v4().to_string();

		// Prepare update request
		let update_request = UsersUpdateRequestDto {
			fullname: Some("Updated Name".to_string()),
			phone_number: None,
			is_active: None,
			avatar: None,
			bio: None,
			birthdate: None,
			gender: None,
			domicile: None,
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
			email: None,
			password: None,
			legal_name: None,
			phone_for_verification: None,
			role_id: None,
		};

		// Update user through service
		let response = UsersService::update_user(&app_state, non_existent_id, update_request).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn test_delete_user_service_invalid_uuid() {
		let app_state = crate::get_app_state().await;

		// Use invalid UUID
		let invalid_id = "invalid-uuid".to_string();

		// Delete user through service
		let response = UsersService::delete_user(&app_state, invalid_id).await;

		// Verify response - should fail validation
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	async fn test_delete_user_service_not_found() {
		let app_state = crate::get_app_state().await;

		// Use valid but non-existent UUID
		let non_existent_id = Uuid::new_v4().to_string();

		// Delete user through service
		let response = UsersService::delete_user(&app_state, non_existent_id).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	async fn test_get_user_by_mentor_id_service_invalid_uuid() {
		let app_state = crate::get_app_state().await;

		// Use invalid UUID
		let invalid_id = "invalid-uuid".to_string();

		// Get user by mentor ID through service
		let response = UsersService::get_user_by_mentor_id(&app_state, invalid_id).await;

		// Verify response - should fail validation
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn test_get_user_by_mentor_id_service_not_found() {
		let app_state = crate::get_app_state().await;

		// Use valid but non-existent UUID
		let non_existent_id = Uuid::new_v4().to_string();

		// Get user by mentor ID through service
		let response = UsersService::get_user_by_mentor_id(&app_state, non_existent_id).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}
}
#[cfg(test)]
mod tests {
	use crate::{generate_unique_email, get_role_id, setup_postgres_test_environment, UsersRepository};
	use axum::{http::StatusCode, response::Response};
	use imphnen_entities::{AppState, MetaRequestDto, ResponseSuccessDto, ResponseListSuccessDto};
	use imphnen_gacha::{
		gacha_credits_controller::GachaCreditsController,
		gacha_credits_dto::{GachaCreditsCreateRequestDto, GachaCreditsUpdateRequestDto},
	};
	use serde_json::json;
	use imphnen_iam::users_service::UsersService;
	use imphnen_utils::{generate_otp, hash_password, get_iso_date};
	use sea_orm::EntityTrait;
	use uuid::Uuid;

	#[tokio::test]
	async fn test_create_gacha_credits() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);

		// Create test user first
		let email = generate_unique_email("test_gacha_credits");
		let password = "Password123!".to_string();
		
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Gacha Credits".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};

		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Test data
		let gacha_credits_dto = GachaCreditsCreateRequestDto {
			user_id: user.id.to_string(),
			amount: 100,
			description: Some("Test Gacha Credits".to_string()),
			transaction_id: Some("TXN-123456".to_string()),
			status: "active".to_string(),
		};

		// Create gacha credits
		let response = GachaCreditsController::create_gacha_credits(&app_state, gacha_credits_dto.clone()).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);
		
		// Parse and verify JSON content
		let response_body: serde_json::Value = response.json().await.unwrap();
		assert!(response_body["message"].is_string(), "Success message should be a string");

		// Clean up
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_get_gacha_credits_list() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);

		// Create test user first
		let email = generate_unique_email("test_gacha_credits_list");
		let password = "Password123!".to_string();
		
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Gacha Credits List".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};

		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Create test gacha credits
		let gacha_credits_dto = GachaCreditsCreateRequestDto {
			user_id: user.id.to_string(),
			amount: 100,
			description: Some("Test Gacha Credits List".to_string()),
			transaction_id: Some("TXN-123456".to_string()),
			status: "active".to_string(),
		};

		let _ = GachaCreditsController::create_gacha_credits(&app_state, gacha_credits_dto).await;

		// Get gacha credits list
		let meta = MetaRequestDto {
			limit: 10,
			page: 1,
			search: None,
			sort: None,
			filter: None,
		};

		let response = GachaCreditsController::get_gacha_credits_list(&app_state, meta).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);
		
		// Parse and verify JSON content
		let response_body: ResponseListSuccessDto<serde_json::Value> = response.json().await.unwrap();
		assert!(!response_body.data.is_null(), "Response data should not be null");
		
		// Verify all fields in response are not empty
		let credits_array = response_body.data.as_array().unwrap();
		for credit in credits_array {
			assert!(credit["id"].is_string() && !credit["id"].as_str().unwrap().is_empty(), "Response credit.id should not be empty");
			assert!(credit["user"].is_object(), "Response credit.user should be an object");
			assert!(credit["available_rolls"].is_i64(), "Response credit.available_rolls should be present");
			assert!(credit["is_deleted"].is_bool(), "Response credit.is_deleted should be present");
			assert!(credit["created_at"].is_string(), "Response credit.created_at should be present");
			assert!(credit["updated_at"].is_string(), "Response credit.updated_at should be present");
		}

		// Clean up
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_get_gacha_credits_by_id() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let gacha_credits_repo = imphnen_gacha::GachaCreditsRepository::new(&app_state);

		// Create test user first
		let email = generate_unique_email("test_gacha_credits_by_id");
		let password = "Password123!".to_string();
		
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Gacha Credits By ID".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};

		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Create test gacha credits
		let gacha_credits_dto = GachaCreditsCreateRequestDto {
			user_id: user.id.id.to_raw(),
			amount: 100,
			description: Some("Test Gacha Credits By ID".to_string()),
			transaction_id: Some("TXN-123456".to_string()),
			status: "active".to_string(),
		};

		let create_response = GachaCreditsController::create_gacha_credits(&app_state, gacha_credits_dto).await;
		let gacha_credits = gacha_credits_repo.query_gacha_credits_by_user_id(user.id.to_string(), false).await.unwrap();
		let gacha_credits_id = gacha_credits.id.id.to_raw();

		// Get gacha credits by ID
		let response = GachaCreditsController::get_gacha_credits_by_id(&app_state, gacha_credits_id).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);
		
		// Parse and verify JSON content
		let response_body: ResponseSuccessDto<serde_json::Value> = response.json().await.unwrap();
		assert!(!response_body.data.is_null(), "Response data should not be null");
		
		// Verify all fields in response are not empty
		let credit = response_body.data.as_object().unwrap();
		assert!(credit["id"].is_string() && !credit["id"].as_str().unwrap().is_empty(), "Response credit.id should not be empty");
		assert!(credit["user"].is_object(), "Response credit.user should be an object");
		assert!(credit["available_rolls"].is_i64(), "Response credit.available_rolls should be present");
		assert!(credit["is_deleted"].is_bool(), "Response credit.is_deleted should be present");
		assert!(credit["created_at"].is_string(), "Response credit.created_at should be present");
		assert!(credit["updated_at"].is_string(), "Response credit.updated_at should be present");

		// Clean up
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_update_gacha_credits() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let gacha_credits_repo = imphnen_gacha::GachaCreditsRepository::new(&app_state);

		// Create test user first
		let email = generate_unique_email("test_update_gacha_credits");
		let password = "Password123!".to_string();
		
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Update Gacha Credits".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};

		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Create test gacha credits
		let gacha_credits_dto = GachaCreditsCreateRequestDto {
			user_id: user.id.to_string(),
			amount: 100,
			description: Some("Test Update Gacha Credits".to_string()),
			transaction_id: Some("TXN-123456".to_string()),
			status: "active".to_string(),
		};

		let _ = GachaCreditsController::create_gacha_credits(&app_state, gacha_credits_dto).await;
		let gacha_credits = gacha_credits_repo.query_gacha_credits_by_user_id(user.id.to_string(), false).await.unwrap();
		let gacha_credits_id = gacha_credits.id.id.to_raw();

		// Prepare update request
		let update_dto = GachaCreditsUpdateRequestDto {
			amount: Some(200),
			description: Some("Updated Test Gacha Credits".to_string()),
			status: Some("used".to_string()),
		};

		// Update gacha credits
		let response = GachaCreditsController::update_gacha_credits(&app_state, gacha_credits_id, update_dto).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);
		
		// Parse and verify JSON content
		let response_body: serde_json::Value = response.json().await.unwrap();
		assert!(response_body["message"].is_string(), "Success message should be a string");
		
		// Parse and verify JSON content
		let response_body: serde_json::Value = response.json().await.unwrap();
		assert!(response_body["message"].is_string(), "Success message should be a string");
		
		// Parse and verify JSON content
		let response_body: serde_json::Value = response.json().await.unwrap();
		assert!(response_body["message"].is_string(), "Success message should be a string");

		// Clean up
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_delete_gacha_credits() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let gacha_credits_repo = imphnen_gacha::GachaCreditsRepository::new(&app_state);

		// Create test user first
		let email = generate_unique_email("test_delete_gacha_credits");
		let password = "Password123!".to_string();
		
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Delete Gacha Credits".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};

		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Create test gacha credits
		let gacha_credits_dto = GachaCreditsCreateRequestDto {
			user_id: user.id.to_string(),
			amount: 100,
			description: Some("Test Delete Gacha Credits".to_string()),
			transaction_id: Some("TXN-123456".to_string()),
			status: "active".to_string(),
		};

		let _ = GachaCreditsController::create_gacha_credits(&app_state, gacha_credits_dto).await;
		let gacha_credits = gacha_credits_repo.query_gacha_credits_by_user_id(user.id.to_string(), false).await.unwrap();
		let gacha_credits_id = gacha_credits.id.id.to_raw();

		// Delete gacha credits
		let response = GachaCreditsController::delete_gacha_credits(&app_state, gacha_credits_id).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Clean up
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}
}
#[cfg(test)]
mod tests {
	use crate::{generate_unique_email, get_role_id, setup_all_test_environment, UsersRepository};
	use axum::{http::StatusCode, response::Response};
	use imphnen_entities::{AppState, MetaRequestDto};
	use imphnen_gacha::{
		gacha_credits_service::GachaCreditsService,
		gacha_credits_dto::{GachaCreditsCreateRequestDto, GachaCreditsUpdateRequestDto},
		GachaCreditsRepository
	};
	use imphnen_iam::users_service::UsersService;
	use imphnen_utils::{hash_password};
	use surrealdb::Uuid;

	#[tokio::test]
	async fn test_create_gacha_credits_service() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let gacha_credits_repo = GachaCreditsRepository::new(&app_state);

		// Create test user first
		let email = generate_unique_email("test_create_gacha_credits_service");
		let password = "Password123!".to_string();
		
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Gacha Credits Service".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};

		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Test data
		let gacha_credits_dto = GachaCreditsCreateRequestDto {
			user_id: user.id.id.to_raw(),
			amount: 100,
			description: Some("Test Gacha Credits Service".to_string()),
			transaction_id: Some("TXN-123456".to_string()),
			status: "active".to_string(),
		};

		// Create gacha credits via service
		let response = GachaCreditsService::create_gacha_credits(&app_state, gacha_credits_dto.clone()).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Verify gacha credits was created in database
		let gacha_credits = gacha_credits_repo.query_gacha_credits_by_user_id(user.id.id.to_raw(), false).await;
		assert!(gacha_credits.is_ok());
		assert_eq!(gacha_credits.unwrap().amount, 100);
		assert_eq!(gacha_credits.unwrap().status, "active".to_string());

		// Clean up
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_get_gacha_credits_list_service() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let gacha_credits_repo = GachaCreditsRepository::new(&app_state);

		// Create test user first
		let email = generate_unique_email("test_gacha_credits_list_service");
		let password = "Password123!".to_string();
		
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Gacha Credits List Service".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};

		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Create test gacha credits
		let gacha_credits_dto = GachaCreditsCreateRequestDto {
			user_id: user.id.id.to_raw(),
			amount: 100,
			description: Some("Test Gacha Credits List Service".to_string()),
			transaction_id: Some("TXN-123456".to_string()),
			status: "active".to_string(),
		};

		let _ = GachaCreditsService::create_gacha_credits(&app_state, gacha_credits_dto).await;

		// Get gacha credits list via service
		let meta = MetaRequestDto {
			limit: 10,
			page: 1,
			search: None,
			sort: None,
			filter: None,
		};

		let response = GachaCreditsService::get_gacha_credits_list(&app_state, meta).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Clean up
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_get_gacha_credits_by_id_service() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let gacha_credits_repo = GachaCreditsRepository::new(&app_state);

		// Create test user first
		let email = generate_unique_email("test_gacha_credits_by_id_service");
		let password = "Password123!".to_string();
		
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Gacha Credits By ID Service".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};

		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Create test gacha credits
		let gacha_credits_dto = GachaCreditsCreateRequestDto {
			user_id: user.id.id.to_raw(),
			amount: 100,
			description: Some("Test Gacha Credits By ID Service".to_string()),
			transaction_id: Some("TXN-123456".to_string()),
			status: "active".to_string(),
		};

		let _ = GachaCreditsService::create_gacha_credits(&app_state, gacha_credits_dto).await;
		let gacha_credits = gacha_credits_repo.query_gacha_credits_by_user_id(user.id.id.to_raw(), false).await.unwrap();
		let gacha_credits_id = gacha_credits.id.id.to_raw();

		// Get gacha credits by ID via service
		let response = GachaCreditsService::get_gacha_credits_by_id(&app_state, gacha_credits_id).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Clean up
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_update_gacha_credits_service() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let gacha_credits_repo = GachaCreditsRepository::new(&app_state);

		// Create test user first
		let email = generate_unique_email("test_update_gacha_credits_service");
		let password = "Password123!".to_string();
		
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Update Gacha Credits Service".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};

		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Create test gacha credits
		let gacha_credits_dto = GachaCreditsCreateRequestDto {
			user_id: user.id.id.to_raw(),
			amount: 100,
			description: Some("Test Update Gacha Credits Service".to_string()),
			transaction_id: Some("TXN-123456".to_string()),
			status: "active".to_string(),
		};

		let _ = GachaCreditsService::create_gacha_credits(&app_state, gacha_credits_dto).await;
		let gacha_credits = gacha_credits_repo.query_gacha_credits_by_user_id(user.id.id.to_raw(), false).await.unwrap();
		let gacha_credits_id = gacha_credits.id.id.to_raw();

		// Prepare update request
		let update_dto = GachaCreditsUpdateRequestDto {
			amount: Some(200),
			description: Some("Updated Test Gacha Credits Service".to_string()),
			status: Some("used".to_string()),
		};

		// Update gacha credits via service
		let response = GachaCreditsService::update_gacha_credits(&app_state, gacha_credits_id, update_dto).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Verify gacha credits was updated
		let updated_gacha_credits = gacha_credits_repo.query_gacha_credits_by_id(&gacha_credits.id, false).await.unwrap();
		assert_eq!(updated_gacha_credits.amount, 200);
		assert_eq!(updated_gacha_credits.status, "used".to_string());

		// Clean up
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_delete_gacha_credits_service() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let gacha_credits_repo = GachaCreditsRepository::new(&app_state);

		// Create test user first
		let email = generate_unique_email("test_delete_gacha_credits_service");
		let password = "Password123!".to_string();
		
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Delete Gacha Credits Service".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};

		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Create test gacha credits
		let gacha_credits_dto = GachaCreditsCreateRequestDto {
			user_id: user.id.id.to_raw(),
			amount: 100,
			description: Some("Test Delete Gacha Credits Service".to_string()),
			transaction_id: Some("TXN-123456".to_string()),
			status: "active".to_string(),
		};

		let _ = GachaCreditsService::create_gacha_credits(&app_state, gacha_credits_dto).await;
		let gacha_credits = gacha_credits_repo.query_gacha_credits_by_user_id(user.id.id.to_raw(), false).await.unwrap();
		let gacha_credits_id = gacha_credits.id.id.to_raw();

		// Delete gacha credits via service
		let response = GachaCreditsService::delete_gacha_credits(&app_state, gacha_credits_id).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Verify gacha credits was deleted
		let deleted_gacha_credits = gacha_credits_repo.query_gacha_credits_by_id(&gacha_credits.id, false).await;
		assert!(deleted_gacha_credits.is_err());

		// Clean up
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}
}
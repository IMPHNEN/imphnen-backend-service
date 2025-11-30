#[cfg(test)]
mod tests {
	use crate::{generate_unique_email, get_role_id, setup_all_test_environment, UsersRepository};
	use imphnen_entities::AppState;
	use imphnen_gacha::v1::gacha_credits::gacha_credits_repository::GachaCreditRepository;
	use imphnen_gacha::v1::gacha_credits::gacha_credits_dto::GachaCreditRequestDto;
	use imphnen_iam::users_service::UsersService;

	#[tokio::test]
	async fn test_query_by_user_id_no_credits() {
		let app_state = setup_all_test_environment().await;
		let credits_repo = GachaCreditRepository::new(&app_state);

		// Test with non-existent user
		let result = credits_repo.query_by_user_id("nonexistent".to_string()).await;
		assert!(result.is_ok());
		assert!(result.unwrap().is_none());
	}

	#[tokio::test]
	async fn test_add_credit_new_user() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let credits_repo = GachaCreditRepository::new(&app_state);

		// Create test user
		let email = generate_unique_email("test_add_credit_new");
		let password = "Password123!".to_string();
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Add Credit New".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};
		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Add credit for new user
		let credit_dto = GachaCreditRequestDto {
			user_id: user.id.id.to_raw(),
			amount: 50,
		};
		let result = credits_repo.query_add_credit(credit_dto).await;
		assert!(result.is_ok());

		// Verify credit was added
		let credit = credits_repo.query_by_user_id(user.id.id.to_raw()).await.unwrap().unwrap();
		assert_eq!(credit.available_rolls, 50);

		// Clean up
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_add_credit_existing_user() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let credits_repo = GachaCreditRepository::new(&app_state);

		// Create test user
		let email = generate_unique_email("test_add_credit_existing");
		let password = "Password123!".to_string();
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Add Credit Existing".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};
		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Add initial credit
		let credit_dto1 = GachaCreditRequestDto {
			user_id: user.id.id.to_raw(),
			amount: 20,
		};
		let _ = credits_repo.query_add_credit(credit_dto1).await;

		// Add more credit
		let credit_dto2 = GachaCreditRequestDto {
			user_id: user.id.id.to_raw(),
			amount: 30,
		};
		let result = credits_repo.query_add_credit(credit_dto2).await;
		assert!(result.is_ok());

		// Verify credit was accumulated
		let credit = credits_repo.query_by_user_id(user.id.id.to_raw()).await.unwrap().unwrap();
		assert_eq!(credit.available_rolls, 50);

		// Clean up
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_consume_credit_success() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let credits_repo = GachaCreditRepository::new(&app_state);

		// Create test user
		let email = generate_unique_email("test_consume_credit_success");
		let password = "Password123!".to_string();
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Consume Credit Success".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};
		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Add credit
		let credit_dto = GachaCreditRequestDto {
			user_id: user.id.id.to_raw(),
			amount: 10,
		};
		let _ = credits_repo.query_add_credit(credit_dto).await;

		// Consume credit
		let result = credits_repo.query_consume_credit(user.id.id.to_raw()).await;
		assert!(result.is_ok());

		// Verify credit was consumed
		let credit = credits_repo.query_by_user_id(user.id.id.to_raw()).await.unwrap().unwrap();
		assert_eq!(credit.available_rolls, 9);

		// Clean up
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_consume_credit_no_credits() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let credits_repo = GachaCreditRepository::new(&app_state);

		// Create test user
		let email = generate_unique_email("test_consume_credit_no_credits");
		let password = "Password123!".to_string();
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Consume Credit No Credits".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};
		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Try to consume credit without any
		let result = credits_repo.query_consume_credit(user.id.id.to_raw()).await;
		assert!(result.is_ok()); // Should succeed but do nothing

		// Verify no credit record was created
		let credit = credits_repo.query_by_user_id(user.id.id.to_raw()).await.unwrap();
		assert!(credit.is_none());

		// Clean up
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_consume_credit_insufficient_credits() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let credits_repo = GachaCreditRepository::new(&app_state);

		// Create test user
		let email = generate_unique_email("test_consume_credit_insufficient");
		let password = "Password123!".to_string();
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Consume Credit Insufficient".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};
		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Add 1 credit
		let credit_dto = GachaCreditRequestDto {
			user_id: user.id.id.to_raw(),
			amount: 1,
		};
		let _ = credits_repo.query_add_credit(credit_dto).await;

		// Consume first credit
		let _ = credits_repo.query_consume_credit(user.id.id.to_raw()).await;

		// Try to consume again (should fail)
		let result = credits_repo.query_consume_credit(user.id.id.to_raw()).await;
		assert!(result.is_err());

		// Verify credit is 0
		let credit = credits_repo.query_by_user_id(user.id.id.to_raw()).await.unwrap().unwrap();
		assert_eq!(credit.available_rolls, 0);

		// Clean up
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_add_credit_zero_amount() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let credits_repo = GachaCreditRepository::new(&app_state);

		// Create test user
		let email = generate_unique_email("test_add_credit_zero");
		let password = "Password123!".to_string();
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Add Credit Zero".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};
		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Add zero credit
		let credit_dto = GachaCreditRequestDto {
			user_id: user.id.id.to_raw(),
			amount: 0,
		};
		let result = credits_repo.query_add_credit(credit_dto).await;
		assert!(result.is_ok());

		// Verify credit was added with 0
		let credit = credits_repo.query_by_user_id(user.id.id.to_raw()).await.unwrap().unwrap();
		assert_eq!(credit.available_rolls, 0);

		// Clean up
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_add_credit_negative_amount() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let credits_repo = GachaCreditRepository::new(&app_state);

		// Create test user
		let email = generate_unique_email("test_add_credit_negative");
		let password = "Password123!".to_string();
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Add Credit Negative".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};
		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Add negative credit (should still work as i32 allows negative)
		let credit_dto = GachaCreditRequestDto {
			user_id: user.id.id.to_raw(),
			amount: -10,
		};
		let result = credits_repo.query_add_credit(credit_dto).await;
		assert!(result.is_ok());

		// Verify negative credit was added
		let credit = credits_repo.query_by_user_id(user.id.id.to_raw()).await.unwrap().unwrap();
		assert_eq!(credit.available_rolls, -10);

		// Clean up
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}
}
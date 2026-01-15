#[cfg(test)]
mod tests {
	use crate::{generate_unique_email, get_role_id, setup_all_test_environment, UsersRepository};
	use axum::{http::{HeaderMap, StatusCode}, response::Response};
	use imphnen_entities::{AppState, MetaRequestDto};
	use imphnen_gacha::v1::gacha_rolls::gacha_rolls_service::GachaRollService;
	use imphnen_gacha::v1::gacha_rolls::gacha_rolls_dto::GachaRollRequestDto;
	use imphnen_gacha::v1::gacha_items::gacha_items_service::GachaItemService;
	use imphnen_gacha::v1::gacha_items::gacha_items_dto::GachaItemRequestDto;
	use imphnen_gacha::GachaRollRepository;
	use imphnen_iam::users_service::UsersService;
	use imphnen_utils::hash_password;
	use serde_json;

	#[tokio::test]
	async fn test_get_gacha_roll_by_id_service() {
		let app_state = setup_all_test_environment().await;
		let roll_repo = GachaRollRepository::new(&app_state);

		// Create test item first
		let item_dto = GachaItemRequestDto {
			name: "Test Item".to_string(),
			image_url: "https://example.com/item.png".to_string(),
		};
		let _ = GachaItemService::create_gacha_item(&app_state, item_dto).await;
		let item = roll_repo.query_all_active_rolls().await.unwrap().into_iter().find(|r| r.item.name == "Test Item").unwrap().item.clone();

		// Create test roll
		let roll_dto = GachaRollRequestDto {
			item_id: item.id.clone(),
			weight: 1.0,
			quantity: 10,
		};
		let _ = GachaRollService::create_gacha_roll(&app_state, roll_dto).await;
		let roll = roll_repo.query_all_active_rolls().await.unwrap().into_iter().find(|r| r.item.name == "Test Item").unwrap();

		// Test get by id
		let response = GachaRollService::get_gacha_roll_by_id(&app_state, roll.id.clone()).await;

		// Verify response (status + body)
		assert_eq!(response.status(), StatusCode::OK);
		let v = crate::common::response_helpers::parse_response_value(response, 4096).await;
		let data = v.get("data").expect("response should contain data");
		assert_eq!(data["item"]["name"].as_str().unwrap(), "Test Item");

		// Clean up
		let _ = roll_repo.query_soft_delete_gacha_roll(roll.id.clone()).await;
	}

	#[tokio::test]
	async fn test_get_gacha_roll_by_id_not_found() {
		let app_state = setup_all_test_environment().await;

		// Test get by non-existent id
		let response = GachaRollService::get_gacha_roll_by_id(&app_state, "nonexistent".to_string()).await;

		// Verify response (status + body)
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in NOT_FOUND response");
	}

	#[tokio::test]
	async fn test_create_gacha_roll_service() {
		let app_state = setup_all_test_environment().await;
		let roll_repo = GachaRollRepository::new(&app_state);

		// Create test item first
		let item_dto = GachaItemRequestDto {
			name: "Test Item Create".to_string(),
			image_url: "https://example.com/item.png".to_string(),
		};
		let _ = GachaItemService::create_gacha_item(&app_state, item_dto).await;
		let item = roll_repo.query_all_active_rolls().await.unwrap().into_iter().find(|r| r.item.name == "Test Item Create").unwrap().item.clone();

		// Test data
		let roll_dto = GachaRollRequestDto {
			item_id: item.id.clone(),
			weight: 1.0,
			quantity: 10,
		};

		// Create roll via service
		let response = GachaRollService::create_gacha_roll(&app_state, roll_dto.clone()).await;

		// Verify response (status + body)
		assert_eq!(response.status(), StatusCode::CREATED);
		let v = crate::common::response_helpers::parse_response_value(response, 4096).await;
		let data = v.get("data").expect("response should contain data");
		assert_eq!(data["item"]["name"].as_str().unwrap(), "Test Item Create");
		assert_eq!(data["weight"].as_f64().unwrap(), 1.0);
		assert_eq!(data["quantity"].as_i64().unwrap(), 10);

		// Verify roll was created in database
		let rolls = roll_repo.query_all_active_rolls().await.unwrap();
		assert!(rolls.iter().any(|r| r.item.name == "Test Item Create" && r.weight == 1.0 && r.quantity == 10));

		// Clean up
		for roll in rolls {
			let _ = roll_repo.query_soft_delete_gacha_roll(roll.id.clone()).await;
		}
	}

	#[tokio::test]
	async fn test_create_gacha_roll_invalid_input() {
		let app_state = setup_all_test_environment().await;

		// Test with empty item_id
		let roll_dto = GachaRollRequestDto {
			item_id: "".to_string(),
			weight: 1.0,
			quantity: 10,
		};

		let response = GachaRollService::create_gacha_roll(&app_state, roll_dto).await;

		// Verify response (status + body)
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in BAD_REQUEST response");
	}

	#[tokio::test]
	async fn test_create_gacha_roll_zero_quantity() {
		let app_state = setup_all_test_environment().await;

		// Create test item first
		let item_dto = GachaItemRequestDto {
			name: "Test Item Zero".to_string(),
			image_url: "https://example.com/item.png".to_string(),
		};
		let _ = GachaItemService::create_gacha_item(&app_state, item_dto).await;
		let roll_repo = GachaRollRepository::new(&app_state);
		let item = roll_repo.query_all_active_rolls().await.unwrap().into_iter().find(|r| r.item.name == "Test Item Zero").unwrap().item.clone();

		// Test with quantity = 0
		let roll_dto = GachaRollRequestDto {
			item_id: item.id.clone(),
			weight: 1.0,
			quantity: 0,
		};

		let response = GachaRollService::create_gacha_roll(&app_state, roll_dto).await;

		// Verify response - should fail validation (status + body)
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in BAD_REQUEST response");
	}

	#[tokio::test]
	async fn test_execute_roll_once_happy_path() {
		let app_state = setup_all_test_environment().await;
		let roll_repo = GachaRollRepository::new(&app_state);
		let user_repo = UsersRepository::new(&app_state);

		// Create test user
		let email = generate_unique_email("test_execute_roll_once");
		let password = "Password123!".to_string();
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Execute Roll".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};
		let _ = UsersService::create_user(&app_state, user_dto).await;
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();

		// Create test item
		let item_dto = GachaItemRequestDto {
			name: "Test Item Roll".to_string(),
			image_url: "https://example.com/item.png".to_string(),
		};
		let _ = GachaItemService::create_gacha_item(&app_state, item_dto).await;

		// Create test roll
		let rolls = roll_repo.query_all_active_rolls().await.unwrap();
		let item = rolls.iter().find(|r| r.item.name == "Test Item Roll").unwrap().item.clone();
		let roll_dto = GachaRollRequestDto {
			item_id: item.id.clone(),
			weight: 1.0,
			quantity: 10,
		};
	let _ = GachaRollService::create_gacha_roll(&app_state, roll_dto).await;

	// Create auth header
	let mut headers = HeaderMap::new();
	headers.insert("Authorization", format!("Bearer {email}").parse().unwrap());

	// Execute roll once
	let response = GachaRollService::execute_roll_once(headers, &app_state).await;

		// Verify response (status + body)
		assert_eq!(response.status(), StatusCode::OK);
		let v = crate::common::response_helpers::parse_response_value(response, 4096).await;
		assert!(v.get("data").is_some(), "expected data in OK response");

		// Clean up
		let _ = user_repo.query_delete_user(user.id.clone()).await;
		let rolls = roll_repo.query_all_active_rolls().await.unwrap();
		for roll in rolls {
			let _ = roll_repo.query_soft_delete_gacha_roll(roll.id.clone()).await;
		}
	}

	#[tokio::test]
	async fn test_execute_roll_once_no_active_rolls() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);

		// Create test user
		let email = generate_unique_email("test_no_active_rolls");
		let password = "Password123!".to_string();
		let user_dto = imphnen_iam::users_dto::UserCreateRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test No Active Rolls".to_string(),
			phone_number: Some("1234567890".to_string()),
			role_id: get_role_id(&app_state, "user").await.unwrap(),
		};
	let _ = UsersService::create_user(&app_state, user_dto).await;

	// Create auth header
	let mut headers = HeaderMap::new();
	headers.insert("Authorization", format!("Bearer {email}").parse().unwrap());

	// Execute roll once with no active rolls
		let response = GachaRollService::execute_roll_once(headers, &app_state).await;

		// Verify response (status + body)
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in NOT_FOUND response");

		// Clean up
		let user = user_repo.query_user_by_email(email).await.unwrap();
		let _ = user_repo.query_delete_user(user.id.clone()).await;
	}

	#[tokio::test]
	async fn test_execute_roll_once_unauthorized() {
		let app_state = setup_all_test_environment().await;

		// Execute roll once without auth header
		let headers = HeaderMap::new();
		let response = GachaRollService::execute_roll_once(headers, &app_state).await;

		// Verify response (status + body)
		assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in UNAUTHORIZED response");
	}

	#[tokio::test]
	async fn test_soft_delete_gacha_roll_service() {
		let app_state = setup_all_test_environment().await;
		let roll_repo = GachaRollRepository::new(&app_state);

		// Create test item and roll first
		let item_dto = GachaItemRequestDto {
			name: "Test Item Delete".to_string(),
			image_url: "https://example.com/item.png".to_string(),
		};
		let _ = GachaItemService::create_gacha_item(&app_state, item_dto).await;
		let rolls = roll_repo.query_all_active_rolls().await.unwrap();
		let item = rolls.iter().find(|r| r.item.name == "Test Item Delete").unwrap().item.clone();
		let roll_dto = GachaRollRequestDto {
			item_id: item.id.clone(),
			weight: 1.0,
			quantity: 10,
		};
		let _ = GachaRollService::create_gacha_roll(&app_state, roll_dto).await;
		let roll = roll_repo.query_all_active_rolls().await.unwrap().into_iter().find(|r| r.item.name == "Test Item Delete").unwrap();

		// Soft delete roll
		let response = GachaRollService::soft_delete_gacha_roll(&app_state, roll.id.clone()).await;

		// Verify response (status + body)
		assert_eq!(response.status(), StatusCode::OK);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some() || v.get("data").is_some(), "expected message or data in OK response");

		// Verify roll is deleted
		let deleted_roll = roll_repo.query_gacha_roll_by_id(roll.id.clone()).await;
		assert!(deleted_roll.is_err());
	}

	#[tokio::test]
	async fn test_soft_delete_gacha_roll_not_found() {
		let app_state = setup_all_test_environment().await;

		// Try to delete non-existent roll
		let response = GachaRollService::soft_delete_gacha_roll(&app_state, "nonexistent".to_string()).await;

		// Verify response (status + body)
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in NOT_FOUND response");
	}
}
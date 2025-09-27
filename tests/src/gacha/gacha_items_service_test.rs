#[cfg(test)]
mod tests {
	use crate::setup_all_test_environment;
	use axum::http::StatusCode;
	use imphnen_entities::{AppState, MetaRequestDto};
	use imphnen_gacha::v1::gacha_items::gacha_items_service::GachaItemService;
	use imphnen_gacha::v1::gacha_items::gacha_items_dto::{GachaItemRequestDto, GachaItemUpdateRequestDto};
	use imphnen_gacha::GachaItemRepository;

	#[tokio::test]
	async fn test_get_gacha_item_list_service() {
		let app_state = setup_all_test_environment().await;
		let item_repo = GachaItemRepository::new(&app_state);

		// Create test item first
		let item_dto = GachaItemRequestDto {
			name: "Test Item List".to_string(),
			image_url: "https://example.com/item.png".to_string(),
		};
		let _ = GachaItemService::create_gacha_item(&app_state, item_dto).await;

		// Get item list
		let meta = MetaRequestDto {
			limit: 10,
			page: 1,
			search: None,
			sort: None,
			filter: None,
		};

		let response = GachaItemService::get_gacha_item_list(&app_state, meta).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Clean up
		let items = item_repo.query_gacha_item_list(MetaRequestDto::default()).await.unwrap().data;
		for item in items {
			let _ = item_repo.query_delete_gacha_item(item.id.id.to_raw()).await;
		}
	}

	#[tokio::test]
	async fn test_get_gacha_item_by_id_service() {
		let app_state = setup_all_test_environment().await;
		let item_repo = GachaItemRepository::new(&app_state);

		// Create test item
		let item_dto = GachaItemRequestDto {
			name: "Test Item By ID".to_string(),
			image_url: "https://example.com/item.png".to_string(),
		};
		let _ = GachaItemService::create_gacha_item(&app_state, item_dto).await;
		let item = item_repo.query_gacha_item_list(MetaRequestDto::default()).await.unwrap().data.into_iter().find(|i| i.name == "Test Item By ID").unwrap();

		// Get item by id
		let response = GachaItemService::get_gacha_item_by_id(&app_state, item.id.id.to_raw()).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Clean up
		let _ = item_repo.query_delete_gacha_item(item.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_get_gacha_item_by_id_not_found() {
		let app_state = setup_all_test_environment().await;

		// Get non-existent item
		let response = GachaItemService::get_gacha_item_by_id(&app_state, "nonexistent".to_string()).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn test_create_gacha_item_service() {
		let app_state = setup_all_test_environment().await;
		let item_repo = GachaItemRepository::new(&app_state);

		// Test data
		let item_dto = GachaItemRequestDto {
			name: "Test Item Create".to_string(),
			image_url: "https://example.com/item.png".to_string(),
		};

		// Create item
		let response = GachaItemService::create_gacha_item(&app_state, item_dto.clone()).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::CREATED);

		// Verify item was created
		let items = item_repo.query_gacha_item_list(MetaRequestDto::default()).await.unwrap().data;
		assert!(items.iter().any(|i| i.name == "Test Item Create"));

		// Clean up
		for item in items {
			let _ = item_repo.query_delete_gacha_item(item.id.id.to_raw()).await;
		}
	}

	#[tokio::test]
	async fn test_create_gacha_item_invalid_input() {
		let app_state = setup_all_test_environment().await;

		// Test with empty name
		let item_dto = GachaItemRequestDto {
			name: "".to_string(),
			image_url: "https://example.com/item.png".to_string(),
		};

		let response = GachaItemService::create_gacha_item(&app_state, item_dto).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	async fn test_create_gacha_item_empty_image_url() {
		let app_state = setup_all_test_environment().await;

		// Test with empty image_url
		let item_dto = GachaItemRequestDto {
			name: "Test Item".to_string(),
			image_url: "".to_string(),
		};

		let response = GachaItemService::create_gacha_item(&app_state, item_dto).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	async fn test_update_gacha_item_service() {
		let app_state = setup_all_test_environment().await;
		let item_repo = GachaItemRepository::new(&app_state);

		// Create test item
		let item_dto = GachaItemRequestDto {
			name: "Test Item Update".to_string(),
			image_url: "https://example.com/item.png".to_string(),
		};
		let _ = GachaItemService::create_gacha_item(&app_state, item_dto).await;
		let item = item_repo.query_gacha_item_list(MetaRequestDto::default()).await.unwrap().data.into_iter().find(|i| i.name == "Test Item Update").unwrap();

		// Update item
		let update_dto = GachaItemUpdateRequestDto {
			name: Some("Updated Test Item".to_string()),
			image_url: Some("https://example.com/updated.png".to_string()),
		};

		let response = GachaItemService::update_gacha_item(&app_state, update_dto, item.id.id.to_raw()).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Verify item was updated
		let updated_item = item_repo.query_gacha_item_by_id(item.id.id.to_raw()).await.unwrap();
		assert_eq!(updated_item.name, "Updated Test Item");

		// Clean up
		let _ = item_repo.query_delete_gacha_item(item.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_update_gacha_item_not_found() {
		let app_state = setup_all_test_environment().await;

		// Update non-existent item
		let update_dto = GachaItemUpdateRequestDto {
			name: Some("Updated Name".to_string()),
			image_url: None,
		};

		let response = GachaItemService::update_gacha_item(&app_state, update_dto, "nonexistent".to_string()).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn test_update_gacha_item_invalid_input() {
		let app_state = setup_all_test_environment().await;
		let item_repo = GachaItemRepository::new(&app_state);

		// Create test item
		let item_dto = GachaItemRequestDto {
			name: "Test Item Invalid Update".to_string(),
			image_url: "https://example.com/item.png".to_string(),
		};
		let _ = GachaItemService::create_gacha_item(&app_state, item_dto).await;
		let item = item_repo.query_gacha_item_list(MetaRequestDto::default()).await.unwrap().data.into_iter().find(|i| i.name == "Test Item Invalid Update").unwrap();

		// Update with empty name
		let update_dto = GachaItemUpdateRequestDto {
			name: Some("".to_string()),
			image_url: None,
		};

		let response = GachaItemService::update_gacha_item(&app_state, update_dto, item.id.id.to_raw()).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);

		// Clean up
		let _ = item_repo.query_delete_gacha_item(item.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_delete_gacha_item_service() {
		let app_state = setup_all_test_environment().await;
		let item_repo = GachaItemRepository::new(&app_state);

		// Create test item
		let item_dto = GachaItemRequestDto {
			name: "Test Item Delete".to_string(),
			image_url: "https://example.com/item.png".to_string(),
		};
		let _ = GachaItemService::create_gacha_item(&app_state, item_dto).await;
		let item = item_repo.query_gacha_item_list(MetaRequestDto::default()).await.unwrap().data.into_iter().find(|i| i.name == "Test Item Delete").unwrap();

		// Delete item
		let response = GachaItemService::delete_gacha_item(&app_state, item.id.id.to_raw()).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Verify item was deleted
		let deleted_item = item_repo.query_gacha_item_by_id(item.id.id.to_raw()).await;
		assert!(deleted_item.is_err());

		// Clean up - already deleted
	}

	#[tokio::test]
	async fn test_delete_gacha_item_not_found() {
		let app_state = setup_all_test_environment().await;

		// Delete non-existent item
		let response = GachaItemService::delete_gacha_item(&app_state, "nonexistent".to_string()).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}
}
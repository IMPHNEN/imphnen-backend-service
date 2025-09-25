use imphnen_gacha::v1::gacha_items::gacha_items_service::{self, GachaItemsRepository};
use imphnen_gacha::v1::gacha_items::gacha_items_dto::{CreateGachaItemDto, GachaItemResponse};
use mockall::mock;
use std::sync::Arc;

mock! {
    pub GachaItemsRepositoryMock {}
    #[async_trait]
    impl GachaItemsRepository for GachaItemsRepositoryMock {
        async fn create(&self, item: &CreateGachaItemDto) -> Result<GachaItemResponse, String>;
        async fn find_by_id(&self, item_id: &str) -> Result<Option<GachaItemResponse>, String>;
        async fn find_all(&self) -> Result<Vec<GachaItemResponse>, String>;
        async fn update(&self, item_id: &str, item: &CreateGachaItemDto) -> Result<GachaItemResponse, String>;
        async fn delete(&self, item_id: &str) -> Result<(), String>;
    }
}

#[tokio::test]
async fn test_create_item_happy_path() {
    let mock_repo = MockGachaItemsRepositoryMock::new();
    let service = gacha_items_service::GachaItemsService::new(Arc::new(mock_repo));
    
    let create_dto = CreateGachaItemDto { name: "Sword".to_string(), rarity: "rare".to_string(), image_url: "https://example.com/sword.png".to_string(), value: 100 };
    let expected = GachaItemResponse { id: "item123".to_string(), name: "Sword".to_string(), rarity: "rare".to_string(), image_url: "https://example.com/sword.png".to_string(), value: 100, created_at: "2024-01-01T00:00:00Z".to_string() };

    mock_repo.expect_create().withf(|i| i.name == create_dto.name && i.rarity == create_dto.rarity).returning(|_| Ok(expected.clone()));
    
    let result = service.create_item(&create_dto).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, expected.id);
}

#[tokio::test]
async fn test_create_item_duplicate_name_error() {
    let mock_repo = MockGachaItemsRepositoryMock::new();
    let service = gacha_items_service::GachaItemsService::new(Arc::new(mock_repo));
    
    let create_dto = CreateGachaItemDto { name: "Sword".to_string(), rarity: "rare".to_string(), image_url: "https://example.com/sword.png".to_string(), value: 100 };
    let error_msg = "Item with this name already exists";

    mock_repo.expect_create().withf(|i| i.name == "Sword").returning(|_| Err(error_msg.to_string()));
    
    let result = service.create_item(&create_dto).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), error_msg);
}

#[tokio::test]
async fn test_get_all_items_empty_case() {
    let mock_repo = MockGachaItemsRepositoryMock::new();
    let service = gacha_items_service::GachaItemsService::new(Arc::new(mock_repo));

    mock_repo.expect_find_all().returning(|| Ok(Vec::new()));
    
    let result = service.get_all_items().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

// Additional tests for get by id, update, delete operations...
use axum_test::TestServer;
use imphnen_gacha::v1::gacha_items::gacha_items_controller::{self, GachaItemsService};
use imphnen_gacha::v1::gacha_items::gacha_items_dto::{CreateGachaItemDto, GachaItemResponse};
use mockall::mock;
use tower::ServiceBuilder;
use tower::timeout::TimeoutLayer;
use std::time::Duration;

mock! {
    pub GachaItemsServiceMock {}
    #[async_trait]
    impl GachaItemsService for GachaItemsServiceMock {
        async fn create_item(&self, item: &CreateGachaItemDto) -> Result<GachaItemResponse, String>;
        async fn get_item(&self, item_id: &str) -> Result<GachaItemResponse, String>;
        async fn get_all_items(&self) -> Result<Vec<GachaItemResponse>, String>;
        async fn update_item(&self, item_id: &str, item: &CreateGachaItemDto) -> Result<GachaItemResponse, String>;
        async fn delete_item(&self, item_id: &str) -> Result<(), String>;
    }
}

#[tokio::test]
async fn test_create_item_happy_path() {
    let mock_service = MockGachaItemsServiceMock::new();
    let create_dto = CreateGachaItemDto { name: "Sword".to_string(), rarity: "rare".to_string(), image_url: "https://example.com/sword.png".to_string(), value: 100 };
    let expected = GachaItemResponse { id: "item123".to_string(), name: "Sword".to_string(), rarity: "rare".to_string(), image_url: "https://example.com/sword.png".to_string(), value: 100, created_at: "2024-01-01T00:00:00Z".to_string() };

    mock_service.expect_create_item().withf(|i| i.name == create_dto.name && i.rarity == create_dto.rarity).returning(|_| Ok(expected.clone()));
    
    let app = ServiceBuilder::new().layer(TimeoutLayer::new(Duration::from_secs(10))).service(gacha_items_controller::router(mock_service));
    let server = TestServer::new(app).unwrap();
    
    let response = server.post("/gacha/items").json(&create_dto).await;
    assert_eq!(response.status(), 201);
    let body: GachaItemResponse = response.json().await.unwrap();
    
    // Verify all fields in response are not empty
    assert!(!body.id.is_empty(), "GachaItemResponse.id should not be empty");
    assert!(!body.name.is_empty(), "GachaItemResponse.name should not be empty");
    assert!(!body.rarity.is_empty(), "GachaItemResponse.rarity should not be empty");
    assert!(!body.image_url.is_empty(), "GachaItemResponse.image_url should not be empty");
    assert!(body.value > 0, "GachaItemResponse.value should be positive");
    assert!(!body.created_at.is_empty(), "GachaItemResponse.created_at should not be empty");
    assert_eq!(body.id, expected.id);
}

#[tokio::test]
async fn test_get_all_items_happy_path() {
    let mock_service = MockGachaItemsServiceMock::new();
    let expected_items = vec![
        GachaItemResponse { id: "item123".to_string(), name: "Sword".to_string(), rarity: "rare".to_string(), image_url: "https://example.com/sword.png".to_string(), value: 100, created_at: "2024-01-01T00:00:00Z".to_string() },
        GachaItemResponse { id: "item456".to_string(), name: "Shield".to_string(), rarity: "common".to_string(), image_url: "https://example.com/shield.png".to_string(), value: 50, created_at: "2024-01-01T00:00:00Z".to_string() }
    ];

    mock_service.expect_get_all_items().returning(|| Ok(expected_items.clone()));
    
    let app = ServiceBuilder::new().layer(TimeoutLayer::new(Duration::from_secs(10))).service(gacha_items_controller::router(mock_service));
    let server = TestServer::new(app).unwrap();
    
    let response = server.get("/gacha/items").await;
    assert_eq!(response.status(), 200);
    let body: Vec<GachaItemResponse> = response.json().await.unwrap();
    assert_eq!(body.len(), 2);
    
    // Verify all fields in all responses are not empty
    for item in &body {
        assert!(!item.id.is_empty(), "GachaItemResponse.id should not be empty");
        assert!(!item.name.is_empty(), "GachaItemResponse.name should not be empty");
        assert!(!item.rarity.is_empty(), "GachaItemResponse.rarity should not be empty");
        assert!(!item.image_url.is_empty(), "GachaItemResponse.image_url should not be empty");
        assert!(item.value > 0, "GachaItemResponse.value should be positive");
        assert!(!item.created_at.is_empty(), "GachaItemResponse.created_at should not be empty");
    }
    assert_eq!(body[0].id, "item123");
    assert_eq!(body[1].id, "item456");
}

// Additional tests for error cases, get by id, update, delete...
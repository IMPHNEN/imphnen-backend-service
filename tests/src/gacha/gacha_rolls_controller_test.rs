use axum_test::TestServer;
use imphnen_gacha::v1::gacha_rolls::gacha_rolls_controller::{self, GachaRollsService};
use imphnen_gacha::v1::gacha_rolls::gacha_rolls_dto::{CreateGachaRollDto, GachaRollResponse};
use mockall::mock;
use tower::ServiceBuilder;
use tower::timeout::TimeoutLayer;
use std::time::Duration;

mock! {
    pub GachaRollsServiceMock {}
    #[async_trait]
    impl GachaRollsService for GachaRollsServiceMock {
        async fn create_roll(&self, user_id: &str, credits_used: i32) -> Result<GachaRollResponse, String>;
        async fn get_roll(&self, roll_id: &str) -> Result<GachaRollResponse, String>;
        async fn get_user_rolls(&self, user_id: &str) -> Result<Vec<GachaRollResponse>, String>;
        async fn get_all_rolls(&self) -> Result<Vec<GachaRollResponse>, String>;
        async fn update_roll(&self, roll_id: &str, status: &str) -> Result<GachaRollResponse, String>;
        async fn delete_roll(&self, roll_id: &str) -> Result<(), String>;
    }
}

#[tokio::test]
async fn test_create_roll_happy_path() {
    let mock_service = MockGachaRollsServiceMock::new();
    let create_dto = CreateGachaRollDto { user_id: "user123".to_string(), credits_used: 10 };
    let expected = GachaRollResponse { id: "roll789".to_string(), user_id: "user123".to_string(), credits_used: 10, items_won: vec!["item456".to_string()], status: "completed".to_string(), created_at: "2024-01-01T00:00:00Z".to_string() };

    mock_service.expect_create_roll().withf(|u, c| u == &create_dto.user_id && c == &create_dto.credits_used).returning(|_, _| Ok(expected.clone()));
    
    let app = ServiceBuilder::new().layer(TimeoutLayer::new(Duration::from_secs(10))).service(gacha_rolls_controller::router(mock_service));
    let server = TestServer::new(app).unwrap();
    
    let response = server.post("/gacha/rolls").json(&create_dto).await;
    assert_eq!(response.status(), 201);
    let body: GachaRollResponse = response.json().await.unwrap();
    assert_eq!(body.id, expected.id);
}

#[tokio::test]
async fn test_get_user_rolls_happy_path() {
    let mock_service = MockGachaRollsServiceMock::new();
    let expected_rolls = vec![
        GachaRollResponse { id: "roll123".to_string(), user_id: "user123".to_string(), credits_used: 10, items_won: vec!["item456".to_string()], status: "completed".to_string(), created_at: "2024-01-01T00:00:00Z".to_string() },
        GachaRollResponse { id: "roll456".to_string(), user_id: "user123".to_string(), credits_used: 5, items_won: vec!["item789".to_string()], status: "completed".to_string(), created_at: "2024-01-02T00:00:00Z".to_string() }
    ];

    mock_service.expect_get_user_rolls().withf(|u| u == "user123").returning(|| Ok(expected_rolls.clone()));
    
    let app = ServiceBuilder::new().layer(TimeoutLayer::new(Duration::from_secs(10))).service(gacha_rolls_controller::router(mock_service));
    let server = TestServer::new(app).unwrap();
    
    let response = server.get("/gacha/rolls/user/user123").await;
    assert_eq!(response.status(), 200);
    let body: Vec<GachaRollResponse> = response.json().await.unwrap();
    assert_eq!(body.len(), 2);
    assert_eq!(body[0].id, "roll123");
    assert_eq!(body[1].id, "roll456");
}

// Additional tests for error cases, get by id, update, delete, get all rolls...
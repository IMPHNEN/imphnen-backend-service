use imphnen_gacha::v1::gacha_rolls::gacha_rolls_service::{self, GachaRollsRepository, GachaCreditsRepository};
use imphnen_gacha::v1::gacha_rolls::gacha_rolls_dto::{CreateGachaRollDto, GachaRollResponse};
use mockall::mock;
use std::sync::Arc;

mock! {
    pub GachaRollsRepositoryMock {}
    #[async_trait]
    impl GachaRollsRepository for GachaRollsRepositoryMock {
        async fn create(&self, roll: &CreateGachaRollDto) -> Result<GachaRollResponse, String>;
        async fn find_by_id(&self, roll_id: &str) -> Result<Option<GachaRollResponse>, String>;
        async fn find_by_user(&self, user_id: &str) -> Result<Vec<GachaRollResponse>, String>;
        async fn find_all(&self) -> Result<Vec<GachaRollResponse>, String>;
        async fn update(&self, roll_id: &str, status: &str) -> Result<GachaRollResponse, String>;
        async fn delete(&self, roll_id: &str) -> Result<(), String>;
    }
}

mock! {
    pub GachaCreditsRepositoryMock {}
    #[async_trait]
    impl GachaCreditsRepository for GachaCreditsRepositoryMock {
        async fn deduct_credits(&self, user_id: &str, amount: i32) -> Result<(), String>;
        async fn add_credits(&self, user_id: &str, amount: i32) -> Result<(), String>;
        async fn get_balance(&self, user_id: &str) -> Result<i32, String>;
    }
}

#[tokio::test]
async fn test_create_roll_happy_path() {
    let mock_roll_repo = MockGachaRollsRepositoryMock::new();
    let mock_credits_repo = MockGachaCreditsRepositoryMock::new();
    let service = gacha_rolls_service::GachaRollsService::new(Arc::new(mock_roll_repo), Arc::new(mock_credits_repo));
    
    let create_dto = CreateGachaRollDto { user_id: "user123".to_string(), credits_used: 10 };
    let expected = GachaRollResponse { id: "roll789".to_string(), user_id: "user123".to_string(), credits_used: 10, items_won: vec!["item456".to_string()], status: "completed".to_string(), created_at: "2024-01-01T00:00:00Z".to_string() };

    mock_credits_repo.expect_deduct_credits().withf(|u, a| u == "user123" && a == 10).returning(|_, _| Ok(()));
    mock_roll_repo.expect_create().withf(|r| r.user_id == create_dto.user_id && r.credits_used == create_dto.credits_used).returning(|_| Ok(expected.clone()));
    
    let result = service.create_roll(&create_dto).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, expected.id);
}

#[tokio::test]
async fn test_create_roll_insufficient_credits() {
    let mock_roll_repo = MockGachaRollsRepositoryMock::new();
    let mock_credits_repo = MockGachaCreditsRepositoryMock::new();
    let service = gacha_rolls_service::GachaRollsService::new(Arc::new(mock_roll_repo), Arc::new(mock_credits_repo));
    
    let create_dto = CreateGachaRollDto { user_id: "user123".to_string(), credits_used: 100 };
    let error_msg = "Insufficient credits";

    mock_credits_repo.expect_deduct_credits().withf(|u, a| u == "user123" && a == 100).returning(|_, _| Err(error_msg.to_string()));
    
    let result = service.create_roll(&create_dto).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), error_msg);
}

#[tokio::test]
async fn test_get_user_rolls_empty_case() {
    let mock_roll_repo = MockGachaRollsRepositoryMock::new();
    let mock_credits_repo = MockGachaCreditsRepositoryMock::new();
    let service = gacha_rolls_service::GachaRollsService::new(Arc::new(mock_roll_repo), Arc::new(mock_credits_repo));

    mock_roll_repo.expect_find_by_user().withf(|u| u == "user123").returning(|| Ok(Vec::new()));
    
    let result = service.get_user_rolls("user123").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

// Additional tests for get by id, update, delete, get all rolls operations...
use imphnen_gacha::v1::gacha_credits::gacha_credits_repository::{self, GachaCreditsRepository};
use sea_orm::{Database, DatabaseConnection, EntityTrait, MockDatabase};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_credits_repository_crud_operations() {
    // Setup in-memory SeaORM database for testing
    let db = MockDatabase::new().await;
    let repo = gacha_credits_repository::GachaCreditsRepository::new(Arc::new(db));

    // Test initial balance (should be 0 for new user)
    let initial_balance = repo.get_balance("user123").await.unwrap();
    assert_eq!(initial_balance, 0);

    // Test add credits
    let add_result = repo.add_credits("user123", 100).await;
    assert!(add_result.is_ok());
    
    // Verify balance after add
    let balance_after_add = repo.get_balance("user123").await.unwrap();
    assert_eq!(balance_after_add, 100);

    // Test deduct credits
    let deduct_result = repo.deduct_credits("user123", 30).await;
    assert!(deduct_result.is_ok());
    
    // Verify balance after deduct
    let balance_after_deduct = repo.get_balance("user123").await.unwrap();
    assert_eq!(balance_after_deduct, 70);

    // Test add multiple times
    repo.add_credits("user123", 50).await.unwrap();
    let final_balance = repo.get_balance("user123").await.unwrap();
    assert_eq!(final_balance, 120);
}

#[tokio::test]
async fn test_credits_repository_error_cases() {
    let db = MockDatabase::new().await;
    let repo = gacha_credits_repository::GachaCreditsRepository::new(Arc::new(db));

    // Test deduct more than available
    let deduct_result = repo.deduct_credits("user123", 50).await;
    assert!(deduct_result.is_err());
    assert_eq!(deduct_result.unwrap_err(), "Insufficient credits");

    // Test add negative amount (should be invalid)
    let add_negative_result = repo.add_credits("user123", -10).await;
    assert!(add_negative_result.is_err());
    assert_eq!(add_negative_result.unwrap_err(), "Cannot add negative credits");

    // Test deduct negative amount (should be invalid)
    let deduct_negative_result = repo.deduct_credits("user123", -5).await;
    assert!(deduct_negative_result.is_err());
    assert_eq!(deduct_negative_result.unwrap_err(), "Cannot deduct negative credits");
}

#[tokio::test]
async fn test_credits_repository_edge_cases() {
    let db = MockDatabase::new().await;
    let repo = gacha_credits_repository::GachaCreditsRepository::new(Arc::new(db));

    // Test add zero credits
    let add_zero_result = repo.add_credits("user123", 0).await;
    assert!(add_zero_result.is_ok());
    
    // Balance should still be 0
    let balance = repo.get_balance("user123").await.unwrap();
    assert_eq!(balance, 0);

    // Test deduct zero credits
    let deduct_zero_result = repo.deduct_credits("user123", 0).await;
    assert!(deduct_zero_result.is_ok());
    
    // Balance should still be 0
    let balance_after = repo.get_balance("user123").await.unwrap();
    assert_eq!(balance_after, 0);

    // Test multiple users
    repo.add_credits("user456", 200).await.unwrap();
    repo.add_credits("user789", 150).await.unwrap();
    
    let user456_balance = repo.get_balance("user456").await.unwrap();
    let user789_balance = repo.get_balance("user789").await.unwrap();
    
    assert_eq!(user456_balance, 200);
    assert_eq!(user789_balance, 150);
}
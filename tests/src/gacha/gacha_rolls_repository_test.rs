use imphnen_gacha::v1::gacha_rolls::gacha_rolls_repository::{self, GachaRollsRepository};
use imphnen_gacha::v1::gacha_rolls::gacha_rolls_dto::{CreateGachaRollDto, GachaRollResponse};
use imphnen_entities::seaorm::gacha::gacha_rolls::{Entity as GachaRollsEntity, ActiveModel as GachaRollActiveModel, Column as GachaRollColumn};
use sea_orm::{DatabaseConnection, EntityTrait, MockDatabase, ActiveValue, Set};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_rolls_repository_crud_operations() {
    // Setup in-memory SeaORM database for testing
    let db = MockDatabase::new().await;
    let repo = gacha_rolls_repository::GachaRollsRepository::new(Arc::new(db));
    
    // Create a test gacha roll directly in the mock database
    let test_roll = GachaRollActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        user_id: Set("user123".to_string()),
        gacha_id: Set("gacha123".to_string()),
        item_id: Set("item123".to_string()),
        quantity: Set(1),
        weight: Set(1.0),
        is_deleted: Set(false),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };
    
    let created_roll = GachaRollsEntity::insert(test_roll).exec(&db).await.unwrap();
    let roll_id = created_roll.last_insert_id.to_string();

    // Test find by id
    let found = repo.query_gacha_roll_by_id(roll_id.clone()).await.unwrap();
    assert_eq!(found.user_id, "user123");

    // Test find all active rolls
    let all_rolls = repo.query_all_active_rolls().await.unwrap();
    assert_eq!(all_rolls.len(), 1);
    assert_eq!(all_rolls[0].user_id, "user123");

    // Test soft delete
    let delete_result = repo.query_soft_delete_gacha_roll(roll_id.clone()).await;
    assert!(delete_result.is_ok());
    
    // Verify deletion
    let result = repo.query_gacha_roll_by_id(roll_id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_rolls_repository_error_cases() {
    let db = MockDatabase::new().await;
    let repo = gacha_rolls_repository::GachaRollsRepository::new(Arc::new(db));

    // Test find by non-existent id
    let result = repo.query_gacha_roll_by_id("non_existent_id").await;
    assert!(result.is_err());

    // Test soft delete non-existent roll
    let result = repo.query_soft_delete_gacha_roll("non_existent_id").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_rolls_repository_create_with_items() {
    let db = MockDatabase::new().await;
    let repo = gacha_rolls_repository::GachaRollsRepository::new(Arc::new(db));
    
    // Create a roll with pre-defined items_won
    let mut create_dto = CreateGachaRollDto { user_id: "user123".to_string(), credits_used: 15 };
    create_dto.items_won = Some(vec!["item456".to_string(), "item789".to_string()]);

    let created = repo.create(&create_dto).await.unwrap();
    assert!(!created.id.is_empty());
    assert_eq!(created.items_won.len(), 2);
    assert_eq!(created.items_won[0], "item456");
    assert_eq!(created.items_won[1], "item789");

    // Verify items are stored correctly
    let found = repo.find_by_id(&created.id).await.unwrap().unwrap();
    assert_eq!(found.items_won.len(), 2);
    assert_eq!(found.items_won[0], "item456");
    assert_eq!(found.items_won[1], "item789");
}
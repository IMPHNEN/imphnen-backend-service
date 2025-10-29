use imphnen_gacha::v1::gacha_rolls::gacha_rolls_repository::{self, GachaRollsRepository};
use imphnen_gacha::v1::gacha_rolls::gacha_rolls_dto::{CreateGachaRollDto, GachaRollResponse};
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use surrealdb::opt::auth::Root;
use std::sync::Arc;

#[tokio::test]
async fn test_rolls_repository_crud_operations() {
    // Setup in-memory SurrealDB for testing
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.signin(Root { username: "root", password: "root" }).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    
    let repo = gacha_rolls_repository::GachaRollsRepository::new(Arc::new(db));
    let create_dto = CreateGachaRollDto { user_id: "user123".to_string(), credits_used: 10 };

    // Test create
    let created = repo.create(&create_dto).await.unwrap();
    assert!(!created.id.is_empty());
    assert_eq!(created.user_id, "user123");
    assert_eq!(created.credits_used, 10);
    assert_eq!(created.items_won.len(), 0); // Default empty array

    // Test find by id
    let found = repo.find_by_id(&created.id).await.unwrap().unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.user_id, "user123");

    // Test find by user
    let user_rolls = repo.find_by_user("user123").await.unwrap();
    assert_eq!(user_rolls.len(), 1);
    assert_eq!(user_rolls[0].id, created.id);

    // Test find all
    let all_rolls = repo.find_all().await.unwrap();
    assert_eq!(all_rolls.len(), 1);
    assert_eq!(all_rolls[0].id, created.id);

    // Test update status
    let updated = repo.update(&created.id, "completed").await.unwrap();
    assert_eq!(updated.status, "completed");
    assert_eq!(updated.id, created.id);

    // Test delete
    let delete_result = repo.delete(&created.id).await;
    assert!(delete_result.is_ok());
    
    // Verify deletion
    let deleted = repo.find_by_id(&created.id).await.unwrap();
    assert!(deleted.is_none());
}

#[tokio::test]
async fn test_rolls_repository_error_cases() {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.signin(Root { username: "root", password: "root" }).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    
    let repo = gacha_rolls_repository::GachaRollsRepository::new(Arc::new(db));

    // Test find by non-existent id
    let result = repo.find_by_id("non_existent_id").await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());

    // Test update non-existent roll
    let result = repo.update("non_existent_id", "completed").await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Roll not found");

    // Test delete non-existent roll
    let result = repo.delete("non_existent_id").await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Roll not found");
}

#[tokio::test]
async fn test_rolls_repository_create_with_items() {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.signin(Root { username: "root", password: "root" }).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    
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
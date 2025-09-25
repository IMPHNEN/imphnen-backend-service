use imphnen_gacha::v1::gacha_items::gacha_items_repository::{self, GachaItemsRepository};
use imphnen_gacha::v1::gacha_items::gacha_items_dto::{CreateGachaItemDto, GachaItemResponse};
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use surrealdb::opt::auth::Root;
use std::sync::Arc;

#[tokio::test]
async fn test_items_repository_crud_operations() {
    // Setup in-memory SurrealDB for testing
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.signin(Root { username: "root", password: "root" }).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    
    let repo = gacha_items_repository::GachaItemsRepository::new(Arc::new(db));
    let create_dto = CreateGachaItemDto { name: "Sword".to_string(), rarity: "rare".to_string(), image_url: "https://example.com/sword.png".to_string(), value: 100 };

    // Test create
    let created = repo.create(&create_dto).await.unwrap();
    assert!(!created.id.is_empty());
    assert_eq!(created.name, "Sword");
    assert_eq!(created.rarity, "rare");

    // Test find by id
    let found = repo.find_by_id(&created.id).await.unwrap().unwrap();
    assert_eq!(found.id, created.id);

    // Test find all
    let all_items = repo.find_all().await.unwrap();
    assert_eq!(all_items.len(), 1);
    assert_eq!(all_items[0].id, created.id);

    // Test update
    let updated_dto = CreateGachaItemDto { name: "Magic Sword".to_string(), rarity: "epic".to_string(), image_url: "https://example.com/magic_sword.png".to_string(), value: 200 };
    let updated = repo.update(&created.id, &updated_dto).await.unwrap();
    assert_eq!(updated.name, "Magic Sword");
    assert_eq!(updated.rarity, "epic");
    assert_eq!(updated.value, 200);

    // Test delete
    let delete_result = repo.delete(&created.id).await;
    assert!(delete_result.is_ok());
    
    // Verify deletion
    let deleted = repo.find_by_id(&created.id).await.unwrap();
    assert!(deleted.is_none());
}

#[tokio::test]
async fn test_items_repository_error_cases() {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.signin(Root { username: "root", password: "root" }).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    
    let repo = gacha_items_repository::GachaItemsRepository::new(Arc::new(db));

    // Test find by non-existent id
    let result = repo.find_by_id("non_existent_id").await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());

    // Test update non-existent item
    let update_dto = CreateGachaItemDto { name: "Test".to_string(), rarity: "common".to_string(), image_url: "https://example.com/test.png".to_string(), value: 10 };
    let result = repo.update("non_existent_id", &update_dto).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Item not found");

    // Test delete non-existent item
    let result = repo.delete("non_existent_id").await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Item not found");
}

#[tokio::test]
async fn test_items_repository_duplicate_name_error() {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.signin(Root { username: "root", password: "root" }).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    
    let repo = gacha_items_repository::GachaItemsRepository::new(Arc::new(db));
    
    // Create first item
    let create_dto1 = CreateGachaItemDto { name: "Sword".to_string(), rarity: "rare".to_string(), image_url: "https://example.com/sword.png".to_string(), value: 100 };
    repo.create(&create_dto1).await.unwrap();
    
    // Try to create item with same name
    let create_dto2 = CreateGachaItemDto { name: "Sword".to_string(), rarity: "common".to_string(), image_url: "https://example.com/sword2.png".to_string(), value: 50 };
    let result = repo.create(&create_dto2).await;
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Item with this name already exists");
}
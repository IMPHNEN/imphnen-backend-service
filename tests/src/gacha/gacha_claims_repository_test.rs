use imphnen_gacha::v1::gacha_claims::gacha_claims_repository::{self, GachaClaimsRepository};
use imphnen_gacha::v1::gacha_claims::gacha_claims_dto::{CreateGachaClaimDto, GachaClaimResponse};
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use surrealdb::opt::auth::Root;
use std::sync::Arc;

#[tokio::test]
async fn test_claims_repository_crud_operations() {
    // Setup in-memory SurrealDB for testing
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.signin(Root { username: "root", password: "root" }).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    
    let repo = gacha_claims_repository::GachaClaimsRepository::new(Arc::new(db));
    let create_dto = CreateGachaClaimDto { user_id: "user123".to_string(), item_id: "item456".to_string() };

    // Test create
    let created = repo.create(&create_dto).await.unwrap();
    assert!(!created.id.is_empty());
    assert_eq!(created.user_id, "user123");
    assert_eq!(created.item_id, "item456");

    // Test find by id
    let found = repo.find_by_id(&created.id).await.unwrap().unwrap();
    assert_eq!(found.id, created.id);

    // Test find by user
    let user_claims = repo.find_by_user("user123").await.unwrap();
    assert_eq!(user_claims.len(), 1);
    assert_eq!(user_claims[0].id, created.id);

    // Test update
    let updated = repo.update(&created.id, "approved").await.unwrap();
    assert_eq!(updated.status, "approved");

    // Test delete
    let delete_result = repo.delete(&created.id).await;
    assert!(delete_result.is_ok());
    
    // Verify deletion
    let deleted = repo.find_by_id(&created.id).await.unwrap();
    assert!(deleted.is_none());
}

#[tokio::test]
async fn test_claims_repository_error_cases() {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.signin(Root { username: "root", password: "root" }).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    
    let repo = gacha_claims_repository::GachaClaimsRepository::new(Arc::new(db));

    // Test find by non-existent id
    let result = repo.find_by_id("non_existent_id").await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());

    // Test update non-existent claim
    let result = repo.update("non_existent_id", "approved").await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Claim not found");

    // Test delete non-existent claim
    let result = repo.delete("non_existent_id").await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Claim not found");
}
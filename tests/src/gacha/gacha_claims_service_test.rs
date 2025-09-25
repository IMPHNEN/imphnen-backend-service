use imphnen_gacha::v1::gacha_claims::gacha_claims_service::{self, GachaClaimsRepository};
use imphnen_gacha::v1::gacha_claims::gacha_claims_dto::{CreateGachaClaimDto, GachaClaimResponse};
use mockall::mock;
use std::sync::Arc;

mock! {
    pub GachaClaimsRepositoryMock {}
    #[async_trait]
    impl GachaClaimsRepository for GachaClaimsRepositoryMock {
        async fn create(&self, claim: &CreateGachaClaimDto) -> Result<GachaClaimResponse, String>;
        async fn find_by_id(&self, claim_id: &str) -> Result<Option<GachaClaimResponse>, String>;
        async fn find_by_user(&self, user_id: &str) -> Result<Vec<GachaClaimResponse>, String>;
        async fn update(&self, claim_id: &str, status: &str) -> Result<GachaClaimResponse, String>;
        async fn delete(&self, claim_id: &str) -> Result<(), String>;
    }
}

#[tokio::test]
async fn test_create_claim_happy_path() {
    let mock_repo = MockGachaClaimsRepositoryMock::new();
    let service = gacha_claims_service::GachaClaimsService::new(Arc::new(mock_repo));
    
    let create_dto = CreateGachaClaimDto { user_id: "user123".to_string(), item_id: "item456".to_string() };
    let expected = GachaClaimResponse { id: "claim789".to_string(), user_id: "user123".to_string(), item_id: "item456".to_string(), status: "pending".to_string(), created_at: "2024-01-01T00:00:00Z".to_string() };

    mock_repo.expect_create().withf(|c| c.user_id == create_dto.user_id && c.item_id == create_dto.item_id).returning(|_| Ok(expected.clone()));
    
    let result = service.create_claim(&create_dto).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, expected.id);
}

#[tokio::test]
async fn test_create_claim_error_case() {
    let mock_repo = MockGachaClaimsRepositoryMock::new();
    let service = gacha_claims_service::GachaClaimsService::new(Arc::new(mock_repo));
    
    let create_dto = CreateGachaClaimDto { user_id: "user123".to_string(), item_id: "invalid_item".to_string() };
    let error_msg = "Item not found";

    mock_repo.expect_create().withf(|c| c.item_id == "invalid_item").returning(|_| Err(error_msg.to_string()));
    
    let result = service.create_claim(&create_dto).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), error_msg);
}

// Additional tests for get, update, delete operations...
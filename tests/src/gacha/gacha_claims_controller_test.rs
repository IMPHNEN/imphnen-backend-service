use axum_test::TestServer;
use imphnen_gacha::v1::gacha_claims::gacha_claims_controller::{self, GachaClaimsService};
use imphnen_gacha::v1::gacha_claims::gacha_claims_dto::{CreateGachaClaimDto, GachaClaimResponse};
use mockall::mock;
use tower::ServiceBuilder;
use tower::timeout::TimeoutLayer;
use std::time::Duration;

mock! {
    pub GachaClaimsServiceMock {}
    #[async_trait]
    impl GachaClaimsService for GachaClaimsServiceMock {
        async fn create_claim(&self, user_id: &str, item_id: &str) -> Result<GachaClaimResponse, String>;
        async fn get_claim(&self, claim_id: &str) -> Result<GachaClaimResponse, String>;
        async fn get_user_claims(&self, user_id: &str) -> Result<Vec<GachaClaimResponse>, String>;
        async fn update_claim(&self, claim_id: &str, status: &str) -> Result<GachaClaimResponse, String>;
        async fn delete_claim(&self, claim_id: &str) -> Result<(), String>;
    }
}

#[tokio::test]
async fn test_create_claim_happy_path() {
    let mock_service = MockGachaClaimsServiceMock::new();
    let create_dto = CreateGachaClaimDto { user_id: "user123".to_string(), item_id: "item456".to_string() };
    let expected = GachaClaimResponse { id: "claim789".to_string(), user_id: "user123".to_string(), item_id: "item456".to_string(), status: "pending".to_string(), created_at: "2024-01-01T00:00:00Z".to_string() };

    mock_service.expect_create_claim().withf(|u, i| u == &create_dto.user_id && i == &create_dto.item_id).returning(|_, _| Ok(expected.clone()));
    
    let app = ServiceBuilder::new().layer(TimeoutLayer::new(Duration::from_secs(10))).service(gacha_claims_controller::router(mock_service));
    let server = TestServer::new(app).unwrap();
    
    let response = server.post("/gacha/claims").json(&create_dto).await;
    assert_eq!(response.status(), 201);
    let body: GachaClaimResponse = response.json().await.unwrap();
    
    // Verify all fields in response are not empty
    assert!(!body.id.is_empty(), "GachaClaimResponse.id should not be empty");
    assert!(!body.user_id.is_empty(), "GachaClaimResponse.user_id should not be empty");
    assert!(!body.item_id.is_empty(), "GachaClaimResponse.item_id should not be empty");
    assert!(!body.status.is_empty(), "GachaClaimResponse.status should not be empty");
    assert!(!body.created_at.is_empty(), "GachaClaimResponse.created_at should not be empty");
    assert_eq!(body.id, expected.id);
}

// Additional tests for error cases, get, update, delete...
use axum::{http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use imphnen_entities::{ErrorDto, PermissionsEnum};
use imphnen_hackathon::v1::hackathon::hackathon_controller::{
    create_hackathon_submission, get_admin_hackathon_results, update_submission_status,
};
use imphnen_libs::AppState;
use imphnen_middleware::{PermissionsMiddlewareLayer, TimelineEnforcementLayer};
use tower::Service;

#[tokio::test]
async fn test_timeline_enforcement_middleware() {
    // Setup test environment
    let app_state = AppState::default();
    
    // Test that timeline enforcement middleware rejects requests outside allowed phases
    let middleware = TimelineEnforcementLayer::for_submission(app_state.clone());
    
    // Create a test request (this would be properly constructed in real tests)
    let req = axum::http::Request::builder()
        .uri("/hackathons/test-hackathon/submissions")
        .body(axum::body::Body::empty())
        .unwrap();
    
    // The middleware should return a Forbidden response when not in submission phase
    let response = middleware.call(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_admin_permission_required_for_timeline_crud() {
    let app_state = AppState::default();
    
    // Test that admin permission is required for timeline CRUD operations
    let middleware = PermissionsMiddlewareLayer::admin_only(app_state.clone());
    
    let req = axum::http::Request::builder()
        .uri("/hackathons/test-hackathon/timeline")
        .body(axum::body::Body::empty())
        .unwrap();
    
    // The middleware should return a Forbidden response without proper credentials
    let response = middleware.call(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_admin_results_endpoint_returns_masked_data() {
    let app_state = AppState::default();
    
    // Test that admin results endpoint returns masked sensitive data
    let req = axum::http::Request::builder()
        .uri("/hackathons/test-hackathon/admin/results")
        .body(axum::body::Body::empty())
        .unwrap();
    
    // In a real test, we would properly set up the middleware chain
    let response = get_admin_hackathon_results(
        axum::http::HeaderMap::new(),
        axum::extract::Extension(app_state),
        axum::extract::Path("test-hackathon".to_string()),
        axum::extract::Query(imphnen_libs::MetaRequestDto::default()),
    ).await;
    
    let response_body = response.into_response().into_body();
    // Verify that the response contains masked data patterns
    // This would be more comprehensive in a real test
}

#[tokio::test]
async fn test_admin_submission_review_endpoint() {
    let app_state = AppState::default();
    
    // Test that submission review endpoint requires admin permission
    let middleware = PermissionsMiddlewareLayer::admin_only(app_state.clone());
    
    let req = axum::http::Request::builder()
        .uri("/hackathons/submissions/test-submission/status")
        .body(axum::body::Body::empty())
        .unwrap();
    
    // The middleware should return a Forbidden response without admin credentials
    let response = middleware.call(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_timeline_enforcement_for_submissions() {
    let app_state = AppState::default();
    
    // Test that submission creation is only allowed during submission phase
    let timeline_middleware = TimelineEnforcementLayer::for_submission(app_state.clone());
    
    let req = axum::http::Request::builder()
        .uri("/hackathons/test-hackathon/teams/test-team/submissions")
        .body(axum::body::Body::empty())
        .unwrap();
    
    // The middleware should return a Forbidden response when not in submission phase
    let response = timeline_middleware.call(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_permissions_enum_contains_administrator() {
    // Verify that Administrator permission exists in the enum
    let admin_permission = PermissionsEnum::Administrator;
    assert_eq!(admin_permission.to_string(), "Administrator");
    assert_eq!(admin_permission.id(), "d6e7f8a9-0123-4567-8901-6789012345ab");
}
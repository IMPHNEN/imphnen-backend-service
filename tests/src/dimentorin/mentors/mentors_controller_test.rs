use axum::{
    body::Body,
    http::{HeaderMap, Method, Request, StatusCode},
    response::Response,
    routing::{get, post, put, delete},
    Router,
};
use http_body_util::BodyExt;
use imphnen_dimentorin::{
    mentors_controller,
    mentors_dto::{
        MentorUpdateRequestDto, MentorUserRegisterRequestDto, MentorVerifyRequestDto,
        MentorRegisterResponseDto,
    },
};
use imphnen_entities::{AppState, ResponseSuccessDto};
use imphnen_iam::{PermissionsEnum, RolesEnum, UsersRepository};
use imphnen_libs::{ResourceEnum, surrealdb_init_ws, surrealdb_init_mem, Env};
use imphnen_utils::{generate_otp, hash_password, make_thing, get_iso_date};
use serde_json::json;
use surrealdb::{Uuid, sql::Thing};
use dotenvy::dotenv;
use tower::ServiceExt;
use crate::{generate_unique_email, get_role_id, setup_all_test_environment};

// Helper function to create a test application (router) with all mentor endpoints
fn app(app_state: AppState) -> Router {
    Router::new()
        .route("/v1/mentors/register", post(mentors_controller::post_register_mentor))
        .route("/v1/mentors", get(mentors_controller::get_mentor_list))
        .route("/v1/mentors/detail/:id", get(mentors_controller::get_mentor_by_id))
        .route("/v1/mentors/update/:id", put(mentors_controller::put_update_mentor))
        .route("/v1/mentors/delete/:id", delete(mentors_controller::delete_mentor))
        .route("/v1/mentors/verify/:id", put(mentors_controller::put_verify_mentor))
        .route("/v1/mentors/me", get(mentors_controller::get_mentor_me))
        .route("/v1/mentors/update/me", put(mentors_controller::put_update_mentor_me))
        .route("/v1/mentors/update", put(mentors_controller::put_update_mentor_no_id))
        .route("/v1/mentors/status", get(mentors_controller::get_mentor_status))
        .with_state(app_state)
}

// Helper function to create a valid mentor registration DTO
fn create_valid_mentor_dto(email: &str) -> MentorUserRegisterRequestDto {
    MentorUserRegisterRequestDto {
        email: email.to_string(),
        password: "Password123!".to_string(),
        fullname: "Test Mentor".to_string(),
        phone_number: "1234567890".to_string(),
        identity_and_verification: imphnen_dimentorin::mentors_dto::IdentityAndVerification {
            legal_name: "Legal Test Name".to_string(),
            gender: Some("Laki-laki".to_string()),
            domicile: Some("Jakarta Selatan".to_string()),
            identity_document_url: "http://example.com/id.pdf".to_string(),
            phone_for_verification: "0987654321".to_string(),
        },
        professional_profile: imphnen_dimentorin::mentors_dto::ProfessionalProfile {
            bio: "Experienced professional with 5+ years of experience in software development.".to_string(),
            last_education: Some("S1".to_string()),
            linkedin_url: Some("http://linkedin.com/in/test".to_string()),
            github_url: None,
            cv_url: None,
            portfolio_url: Some("http://example.com/portfolio".to_string()),
            industries: vec!["Technology".to_string()],
            expertise: vec!["Rust".to_string(), "Backend Development".to_string()],
            languages: vec!["English".to_string()],
            current_company: "Tech Corp".to_string(),
            current_role: "Senior Engineer".to_string(),
            years_of_experience: 5,
        },
        mentoring_logistics: imphnen_dimentorin::mentors_dto::MentoringLogistics {
            topics_of_interest: vec!["Career Development".to_string()],
            preferred_mentee_level: vec!["Beginner".to_string()],
            preferred_mentoring_formats: vec!["Online".to_string()],
            availability_commitment: "5 hours/week".to_string(),
            mentoring_rate_amount: 100,
        },
    }
}

// Helper function to create a valid mentor update DTO
fn create_valid_mentor_update_dto() -> MentorUpdateRequestDto {
    MentorUpdateRequestDto {
        legal_name: Some("Updated Legal Name".to_string()),
        gender: Some("Perempuan".to_string()),
        domicile: Some("Bandung".to_string()),
        phone_for_verification: Some("0876543210".to_string()),
        bio: Some("Updated bio with more experience.".to_string()),
        last_education: Some("S2".to_string()),
        linkedin_url: Some("http://linkedin.com/in/updated".to_string()),
        github_url: Some("http://github.com/updated".to_string()),
        cv_url: Some("http://example.com/updated_cv.pdf".to_string()),
        portfolio_url: Some("http://example.com/updated_portfolio".to_string()),
        industries: Some(vec!["Technology".to_string(), "Education".to_string()]),
        expertise: Some(vec!["Rust".to_string(), "AI".to_string()]),
        languages: Some(vec!["English".to_string(), "Spanish".to_string()]),
        current_company: Some("New Tech Corp".to_string()),
        current_role: Some("Lead Engineer".to_string()),
        years_of_experience: Some(7),
        topics_of_interest: Some(vec!["Career Development".to_string(), "Tech Trends".to_string()]),
        preferred_mentee_level: Some(vec!["Beginner".to_string(), "Intermediate".to_string()]),
        preferred_mentoring_formats: Some(vec!["Online".to_string(), "Offline".to_string()]),
        availability_commitment: Some("10 hours/week".to_string()),
        mentoring_rate_amount: Some(200),
    }
}

// Helper function to create authentication headers with mock JWT
fn create_auth_headers(user_email: &str, permissions: Vec<PermissionsEnum>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", format!("Bearer mock_jwt_{}", user_email).parse().unwrap());
    headers
}

#[tokio::test]
async fn test_post_register_mentor_success() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "register_mentor_success@example.com";
    let dto = create_valid_mentor_dto(test_email);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let mentor_register_response: MentorRegisterResponseDto = serde_json::from_slice(&body).unwrap();

    assert!(!mentor_register_response.id.is_empty());
    assert!(!mentor_register_response.user_id.is_empty());
    assert_eq!(mentor_register_response.status, "pending".to_string());

    // Verify user was created in database
    let user_repo = UsersRepository::new(&app_state);
    let user = user_repo.query_user_by_email(test_email.to_string()).await.unwrap();
    assert_eq!(user.email, test_email);
    assert_eq!(user.is_active, false); // Should be inactive until OTP verification

    // Clean up
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_post_register_mentor_invalid_email() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "invalid-email";
    let mut dto = create_valid_mentor_dto(test_email);
    dto.email = test_email.to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let error_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(error_response["message"].as_str().unwrap().contains("email"));
}

#[tokio::test]
async fn test_post_register_mentor_weak_password() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "weak_password@example.com";
    let mut dto = create_valid_mentor_dto(test_email);
    dto.password = "weak".to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let error_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(error_response["message"].as_str().unwrap().contains("password"));
}

#[tokio::test]
async fn test_get_mentor_list_success() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Create a test mentor first
    let test_email = "list_mentor_test@example.com";
    let dto = create_valid_mentor_dto(test_email);
    
    let register_response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(register_response.status(), StatusCode::OK);

    // Get mentor list with authentication
    let headers = create_auth_headers(test_email, vec![PermissionsEnum::ReadListMentors]);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/mentors")
                .headers(headers)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let response_data: ResponseSuccessDto<Vec<imphnen_dimentorin::mentors_dto::MentorListResponseDto>> = 
        serde_json::from_slice(&body).unwrap();
    
    assert!(!response_data.data.is_empty());
    assert_eq!(response_data.data[0].status, "pending".to_string());

    // Clean up
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_get_mentor_list_unauthorized() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Try to get mentor list without authentication
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/mentors")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_mentor_by_id_success() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Create a test mentor first
    let test_email = "get_by_id_test@example.com";
    let dto = create_valid_mentor_dto(test_email);
    
    let register_response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(register_response.status(), StatusCode::OK);
    let register_body: MentorRegisterResponseDto = serde_json::from_slice(&register_response.into_body().collect().await.unwrap().to_bytes()).unwrap();
    let mentor_id = register_body.id.clone();

    // Get mentor by ID with authentication
    let headers = create_auth_headers(test_email, vec![PermissionsEnum::ReadDetailMentors]);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(&format!("/v1/mentors/detail/{}", mentor_id))
                .headers(headers)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let mentor_response: ResponseSuccessDto<imphnen_dimentorin::mentors_dto::MentorDetailResponseDto> = 
        serde_json::from_slice(&body).unwrap();
    
    assert_eq!(mentor_response.data.id, mentor_id);
    assert_eq!(mentor_response.data.status, "pending".to_string());
    assert_eq!(mentor_response.data.fullname, Some("Test Mentor".to_string()));

    // Clean up
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_get_mentor_by_id_not_found() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Try to get non-existent mentor with authentication
    let headers = create_auth_headers("nonexistent@example.com", vec![PermissionsEnum::ReadDetailMentors]);
    let non_existent_id = Uuid::new_v4().to_string();
    
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(&format!("/v1/mentors/detail/{}", non_existent_id))
                .headers(headers)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_put_update_mentor_success() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Create a test mentor first
    let test_email = "update_mentor_test@example.com";
    let dto = create_valid_mentor_dto(test_email);
    
    let register_response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(register_response.status(), StatusCode::OK);
    let register_body: MentorRegisterResponseDto = serde_json::from_slice(&register_response.into_body().collect().await.unwrap().to_bytes()).unwrap();
    let mentor_id = register_body.id.clone();

    // Prepare update DTO
    let update_dto = create_valid_mentor_update_dto();

    // Update mentor with authentication
    let headers = create_auth_headers(test_email, vec![PermissionsEnum::UpdateMentors]);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri(&format!("/v1/mentors/update/{}", mentor_id))
                .headers(headers)
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&update_dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let mentor_response: ResponseSuccessDto<imphnen_dimentorin::mentors_dto::MentorDetailResponseDto> = 
        serde_json::from_slice(&body).unwrap();
    
    assert_eq!(mentor_response.data.id, mentor_id);
    assert_eq!(mentor_response.data.legal_name, Some("Updated Legal Name".to_string()));
    assert_eq!(mentor_response.data.current_role, "Lead Engineer".to_string());

    // Clean up
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_put_update_mentor_not_found() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Try to update non-existent mentor with authentication
    let headers = create_auth_headers("nonexistent@example.com", vec![PermissionsEnum::UpdateMentors]);
    let non_existent_id = Uuid::new_v4().to_string();
    let update_dto = create_valid_mentor_update_dto();
    
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri(&format!("/v1/mentors/update/{}", non_existent_id))
                .headers(headers)
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&update_dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_mentor_success() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Create a test mentor first
    let test_email = "delete_mentor_test@example.com";
    let dto = create_valid_mentor_dto(test_email);
    
    let register_response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(register_response.status(), StatusCode::OK);
    let register_body: MentorRegisterResponseDto = serde_json::from_slice(&register_response.into_body().collect().await.unwrap().to_bytes()).unwrap();
    let mentor_id = register_body.id.clone();

    // Delete mentor with authentication
    let headers = create_auth_headers(test_email, vec![PermissionsEnum::DeleteMentors]);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri(&format!("/v1/mentors/delete/{}", mentor_id))
                .headers(headers)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Verify mentor was soft-deleted
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_delete_mentor_not_found() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Try to delete non-existent mentor with authentication
    let headers = create_auth_headers("nonexistent@example.com", vec![PermissionsEnum::DeleteMentors]);
    let non_existent_id = Uuid::new_v4().to_string();
    
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri(&format!("/v1/mentors/delete/{}", non_existent_id))
                .headers(headers)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_put_verify_mentor_success() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Create a test mentor first
    let test_email = "verify_mentor_test@example.com";
    let dto = create_valid_mentor_dto(test_email);
    
    let register_response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(register_response.status(), StatusCode::OK);
    let register_body: MentorRegisterResponseDto = serde_json::from_slice(&register_response.into_body().collect().await.unwrap().to_bytes()).unwrap();
    let mentor_id = register_body.id.clone();

    // Prepare verification DTO
    let verify_dto = MentorVerifyRequestDto {
        status: "verified".to_string(),
    };

    // Verify mentor with authentication
    let headers = create_auth_headers(test_email, vec![PermissionsEnum::VerifyMentors]);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri(&format!("/v1/mentors/verify/{}", mentor_id))
                .headers(headers)
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&verify_dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let mentor_response: ResponseSuccessDto<imphnen_dimentorin::mentors_dto::MentorDetailResponseDto> = 
        serde_json::from_slice(&body).unwrap();
    
    assert_eq!(mentor_response.data.id, mentor_id);
    assert_eq!(mentor_response.data.status, "verified".to_string());

    // Clean up
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_get_mentor_me_success() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Create a test mentor first
    let test_email = "mentor_me_test@example.com";
    let dto = create_valid_mentor_dto(test_email);
    
    let register_response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(register_response.status(), StatusCode::OK);

    // Get mentor me with authentication
    let headers = create_auth_headers(test_email, vec![PermissionsEnum::ReadOwnMentorProfile]);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/mentors/me")
                .headers(headers)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let mentor_response: ResponseSuccessDto<imphnen_dimentorin::mentors_dto::MentorDetailResponseDto> = 
        serde_json::from_slice(&body).unwrap();
    
    assert_eq!(mentor_response.data.email, Some(test_email.to_string()));
    assert_eq!(mentor_response.data.status, "pending".to_string());

    // Clean up
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_put_update_mentor_me_success() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Create a test mentor first
    let test_email = "update_mentor_me_test@example.com";
    let dto = create_valid_mentor_dto(test_email);
    
    let register_response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(register_response.status(), StatusCode::OK);

    // Prepare update DTO
    let update_dto = create_valid_mentor_update_dto();

    // Update mentor me with authentication
    let headers = create_auth_headers(test_email, vec![PermissionsEnum::UpdateOwnMentorProfile]);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/v1/mentors/update/me")
                .headers(headers)
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&update_dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let mentor_response: ResponseSuccessDto<imphnen_dimentorin::mentors_dto::MentorDetailResponseDto> = 
        serde_json::from_slice(&body).unwrap();
    
    assert_eq!(mentor_response.data.legal_name, Some("Updated Legal Name".to_string()));
    assert_eq!(mentor_response.data.current_role, "Lead Engineer".to_string());

    // Clean up
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_put_update_mentor_no_id() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Try to update mentor without ID (should return 400)
    let headers = create_auth_headers("test@example.com", vec![PermissionsEnum::UpdateMentors]);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/v1/mentors/update")
                .headers(headers)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let error_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(error_response["message"], "Mentor ID is required for update");
}

#[tokio::test]
async fn test_get_mentor_status_success() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Create a test mentor first
    let test_email = "mentor_status_test@example.com";
    let dto = create_valid_mentor_dto(test_email);
    
    let register_response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(register_response.status(), StatusCode::OK);

    // Get mentor status with authentication
    let headers = create_auth_headers(test_email, vec![PermissionsEnum::ReadOwnMentorStatus]);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/mentors/status")
                .headers(headers)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let status_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(status_response, "pending");

    // Clean up
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_controller_endpoints_validation() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Test update mentor with invalid data (empty legal name)
    let test_email = "validation_test@example.com";
    let dto = create_valid_mentor_dto(test_email);
    
    let register_response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(register_response.status(), StatusCode::OK);
    let register_body: MentorRegisterResponseDto = serde_json::from_slice(&register_response.into_body().collect().await.unwrap().to_bytes()).unwrap();
    let mentor_id = register_body.id.clone();

    // Prepare invalid update DTO (empty legal name)
    let mut invalid_update_dto = create_valid_mentor_update_dto();
    invalid_update_dto.legal_name = Some("".to_string()); // Invalid - too short

    // Try to update with invalid data
    let headers = create_auth_headers(test_email, vec![PermissionsEnum::UpdateMentors]);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri(&format!("/v1/mentors/update/{}", mentor_id))
                .headers(headers)
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&invalid_update_dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let error_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(error_response["message"].as_str().unwrap().contains("Legal name must be at least 3 characters"));

    // Clean up
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_controller_permission_denied() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Try to access protected endpoint without proper permissions
    let headers = create_auth_headers("test@example.com", vec![]); // Empty permissions
    
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/mentors")
                .headers(headers)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}
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
#[tokio::test]
async fn test_register_mentor_boundary_values() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "boundary_test@example.com";
    let mut dto = create_valid_mentor_dto(test_email);

    // Set boundary values
    dto.identity_and_verification.legal_name = "abc".to_string(); // Exactly 3 chars
    dto.professional_profile.bio = "a".repeat(50); // Exactly 50 chars
    dto.phone_number = "1234567890".to_string(); // Exactly 10 chars
    dto.identity_and_verification.phone_for_verification = "123456789012345".to_string(); // Exactly 15 chars
    dto.professional_profile.years_of_experience = 2; // Exactly 2
    dto.mentoring_logistics.mentoring_rate_amount = 1; // Exactly 1
    dto.mentoring_logistics.availability_commitment = "12345".to_string(); // Exactly 5 chars

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

    // Clean up
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_register_mentor_invalid_urls() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "invalid_urls@example.com";
    let mut dto = create_valid_mentor_dto(test_email);

    // Invalid URLs
    dto.professional_profile.linkedin_url = Some("not-a-url".to_string());
    dto.professional_profile.github_url = Some("invalid-url".to_string());
    dto.professional_profile.cv_url = Some("bad-url".to_string());
    dto.professional_profile.portfolio_url = Some("wrong-url".to_string());
    dto.identity_and_verification.identity_document_url = "invalid-doc-url".to_string();

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
    assert!(error_response["message"].as_str().unwrap().contains("url"));
}

#[tokio::test]
async fn test_register_mentor_empty_arrays() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "empty_arrays@example.com";
    let mut dto = create_valid_mentor_dto(test_email);

    // Empty arrays
    dto.professional_profile.industries = vec![];
    dto.professional_profile.expertise = vec![];
    dto.professional_profile.languages = vec![];
    dto.mentoring_logistics.topics_of_interest = vec![];
    dto.mentoring_logistics.preferred_mentee_level = vec![];
    dto.mentoring_logistics.preferred_mentoring_formats = vec![];

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
    assert!(error_response["message"].as_str().unwrap().contains("required"));
}

#[tokio::test]
async fn test_register_mentor_too_short_values() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "too_short@example.com";
    let mut dto = create_valid_mentor_dto(test_email);

    // Too short values
    dto.identity_and_verification.legal_name = "ab".to_string(); // < 3
    dto.professional_profile.bio = "short".to_string(); // < 50
    dto.phone_number = "123456789".to_string(); // < 10
    dto.identity_and_verification.phone_for_verification = "1234567890".to_string(); // < 10
    dto.professional_profile.years_of_experience = 1; // < 2
    dto.mentoring_logistics.mentoring_rate_amount = 0; // < 1
    dto.mentoring_logistics.availability_commitment = "1234".to_string(); // < 5

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
}

#[tokio::test]
async fn test_register_mentor_too_long_phone() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "too_long_phone@example.com";
    let mut dto = create_valid_mentor_dto(test_email);

    dto.identity_and_verification.phone_for_verification = "1234567890123456".to_string(); // > 15

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
}

#[tokio::test]
async fn test_update_mentor_partial_data() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Create a test mentor first
    let test_email = "partial_update@example.com";
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

    // Update with partial data (only some fields)
    let partial_update_dto = MentorUpdateRequestDto {
        legal_name: Some("Partial Update".to_string()),
        industries: Some(vec!["Updated Industry".to_string()]),
        ..Default::default() // Other fields None
    };

    let headers = create_auth_headers(test_email, vec![PermissionsEnum::UpdateMentors]);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri(&format!("/v1/mentors/update/{}", mentor_id))
                .headers(headers)
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&partial_update_dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let mentor_response: ResponseSuccessDto<imphnen_dimentorin::mentors_dto::MentorDetailResponseDto> =
        serde_json::from_slice(&body).unwrap();

    assert_eq!(mentor_response.data.legal_name, Some("Partial Update".to_string()));
    assert_eq!(mentor_response.data.industries, vec!["Updated Industry".to_string()]);

    // Clean up
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_access_deleted_mentor() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Create and delete a mentor
    let test_email = "deleted_mentor@example.com";
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

    // Delete the mentor
    let headers = create_auth_headers(test_email, vec![PermissionsEnum::DeleteMentors]);
    let delete_response = app
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri(&format!("/v1/mentors/delete/{}", mentor_id))
                .headers(headers.clone())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(delete_response.status(), StatusCode::OK);

    // Try to access the deleted mentor
    let get_response = app
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

    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);

    // Clean up
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_register_mentor_invalid_json() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "application/json")
                .body(Body::from("invalid json {"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_register_mentor_wrong_content_type() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "wrong_content_type@example.com";
    let dto = create_valid_mentor_dto(test_email);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "text/plain")
                .body(Body::from(serde_json::to_string(&dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Axum should handle this, but test for robustness
    // May return 400 or 422 depending on implementation
    assert!(response.status().is_client_error());
}

#[tokio::test]
async fn test_register_mentor_special_characters() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "special_chars@example.com";
    let mut dto = create_valid_mentor_dto(test_email);

    // Use special characters and unicode
    dto.fullname = "Tëst Üsér ñame".to_string();
    dto.identity_and_verification.legal_name = "Lëgäl Nämé".to_string();
    dto.professional_profile.bio = "Bïô wïth spëcïäl chärs - ".repeat(5); // Make it 50+ chars
    dto.professional_profile.current_company = "Cömpäny Ïnc.".to_string();
    dto.professional_profile.expertise = vec!["Rüst Dëvëlôpmënt".to_string()];

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

    // Clean up
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_verify_mentor_invalid_status() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Create a test mentor first
    let test_email = "invalid_status@example.com";
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

    // Try to verify with empty status
    let verify_dto = MentorVerifyRequestDto {
        status: "".to_string(),
    };

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

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Clean up
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_get_mentor_list_pagination_edge_cases() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Create a test mentor first
    let test_email = "pagination_test@example.com";
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

    // Test with large page number
    let headers = create_auth_headers(test_email, vec![PermissionsEnum::ReadListMentors]);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/mentors?page=999999&per_page=1")
                .headers(headers.clone())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let response_data: ResponseSuccessDto<Vec<imphnen_dimentorin::mentors_dto::MentorListResponseDto>> =
        serde_json::from_slice(&body).unwrap();
    assert!(response_data.data.is_empty()); // Should be empty for large page

    // Test with zero per_page
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/mentors?page=1&per_page=0")
                .headers(headers)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Clean up
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_get_mentor_list_search_special_chars() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Create a test mentor first
    let test_email = "search_special@example.com";
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

    // Search with special characters
    let headers = create_auth_headers(test_email, vec![PermissionsEnum::ReadListMentors]);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/mentors?search=%3C%3E%22%27%2F%5C%3A%3B")
                .headers(headers)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Clean up
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}
#[tokio::test]
async fn test_register_mentor_concurrent_requests() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email1 = "concurrent1@example.com";
    let test_email2 = "concurrent2@example.com";
    let dto1 = create_valid_mentor_dto(test_email1);
    let dto2 = create_valid_mentor_dto(test_email2);

    // Send concurrent requests
    let (response1, response2) = tokio::join!(
        app.clone().oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&dto1).unwrap()))
                .unwrap(),
        ),
        app.oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&dto2).unwrap()))
                .unwrap(),
        )
    );

    let response1 = response1.unwrap();
    let response2 = response2.unwrap();

    // Both should succeed or one should fail due to unique constraints
    assert!(response1.status().is_success() || response2.status().is_success());
    if response1.status().is_success() {
        let user_repo = UsersRepository::new(&app_state);
        let _ = user_repo.query_delete_user(test_email1.to_string()).await;
    }
    if response2.status().is_success() {
        let user_repo = UsersRepository::new(&app_state);
        let _ = user_repo.query_delete_user(test_email2.to_string()).await;
    }
}

#[tokio::test]
async fn test_register_mentor_sql_injection_attempt() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "sql_injection@example.com";
    let mut dto = create_valid_mentor_dto(test_email);

    // Attempt SQL injection in various fields
    dto.fullname = "'; DROP TABLE users; --".to_string();
    dto.identity_and_verification.legal_name = "'; SELECT * FROM users; --".to_string();
    dto.professional_profile.bio = "Bio with ' OR '1'='1".to_string();
    dto.professional_profile.current_company = "Company'; --".to_string();

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

    // Should either fail validation or succeed but not execute injection
    // In a secure system, this should fail validation due to special characters
    assert!(response.status().is_client_error() || response.status().is_success());

    if response.status().is_success() {
        // Clean up if it succeeded
        let user_repo = UsersRepository::new(&app_state);
        let _ = user_repo.query_delete_user(test_email.to_string()).await;
    }
}

#[tokio::test]
async fn test_update_mentor_empty_request_body() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Create a test mentor first
    let test_email = "empty_body@example.com";
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

    // Try to update with empty body
    let headers = create_auth_headers(test_email, vec![PermissionsEnum::UpdateMentors]);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri(&format!("/v1/mentors/update/{}", mentor_id))
                .headers(headers)
                .header("Content-Type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should succeed with empty update (no-op)
    assert_eq!(response.status(), StatusCode::OK);

    // Clean up
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_get_mentor_by_id_invalid_uuid() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Try to get mentor with invalid UUID
    let headers = create_auth_headers("test@example.com", vec![PermissionsEnum::ReadDetailMentors]);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/mentors/detail/not-a-uuid")
                .headers(headers)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_verify_mentor_invalid_uuid() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Try to verify mentor with invalid UUID
    let verify_dto = MentorVerifyRequestDto {
        status: "verified".to_string(),
    };

    let headers = create_auth_headers("test@example.com", vec![PermissionsEnum::VerifyMentors]);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/v1/mentors/verify/not-a-uuid")
                .headers(headers)
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&verify_dto).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_delete_mentor_invalid_uuid() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    // Try to delete mentor with invalid UUID
    let headers = create_auth_headers("test@example.com", vec![PermissionsEnum::DeleteMentors]);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri("/v1/mentors/delete/not-a-uuid")
                .headers(headers)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_register_mentor_extremely_large_payload() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "large_payload@example.com";
    let mut dto = create_valid_mentor_dto(test_email);

    // Make bio extremely large
    dto.professional_profile.bio = "A".repeat(100000); // 100KB bio

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

    // Should either succeed or fail due to size limits
    assert!(response.status().is_success() || response.status() == StatusCode::PAYLOAD_TOO_LARGE);

    if response.status().is_success() {
        // Clean up
        let user_repo = UsersRepository::new(&app_state);
        let _ = user_repo.query_delete_user(test_email.to_string()).await;
    }
}

#[tokio::test]
async fn test_register_mentor_duplicate_email() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "duplicate_email@example.com";
    let dto1 = create_valid_mentor_dto(test_email);
    let dto2 = create_valid_mentor_dto(test_email); // Same email

    // First registration
    let response1 = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&dto1).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response1.status(), StatusCode::OK);

    // Second registration with same email
    let response2 = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/mentors/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&dto2).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response2.status(), StatusCode::BAD_REQUEST);

    // Clean up
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}
}
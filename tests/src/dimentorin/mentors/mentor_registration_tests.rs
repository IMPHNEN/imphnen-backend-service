use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    response::Response,
    routing::post,
    Router,
};
use http_body_util::BodyExt; // for `buffer` method
use imphnen_dimentorin::{
    mentors_controller,
    mentors_dto::{MentorUserRegisterRequestDto, MentorRegisterResponseDto},
};
use imphnen_entities::{AppState, SurrealMemClient, SurrealWsClient};
use imphnen_iam::{RolesRepository, UsersRepository};
use imphnen_iam::{RolesEnum, UsersSchema, RolesSchema};
use imphnen_utils::{hash_password, make_thing, get_iso_date};
use surrealdb::{Uuid, sql::Thing};
use imphnen_libs::{ResourceEnum, surrealdb_init_ws, surrealdb_init_mem, Env};
use dotenvy::dotenv;
use tower::ServiceExt; // Added ServiceExt for .oneshot()
use crate::mock_test::setup_all_test_environment; // Import the new setup function

// Helper function to create a test AppState
async fn setup_app_state() -> AppState {
    dotenv().ok();
    // Use the surrealdb initialization functions from imphnen_libs
    let surrealdb_ws = surrealdb_init_ws().await.expect("Failed to initialize SurrealDB WS client");
    let surrealdb_mem = surrealdb_init_mem().await.expect("Failed to initialize SurrealDB MEM client");
    
    let app_state = AppState {
        surrealdb_ws,
        surrealdb_mem,
    };

    // Manually seed roles since seed_roles is not directly callable as a repository method
    let db = &app_state.surrealdb_ws;
    let roles_to_seed = vec![
        (RolesEnum::Admin.to_string(), Vec::new()),
        (RolesEnum::User.to_string(), Vec::new()),
        (RolesEnum::Staff.to_string(), Vec::new()),
        (RolesEnum::Mentor.to_string(), Vec::new()),
    ];

    for (name, permissions) in roles_to_seed {
        // Check if role already exists to avoid errors on rerun
        let existing_role: Option<RolesSchema> = db.query(format!("SELECT * FROM ONLY role WHERE name = '{}'", name)).await.unwrap().take(0).unwrap_or(None);
        if existing_role.is_none() {
            let role_id = Uuid::new_v4().to_string();
            let role = RolesSchema {
                id: make_thing(&ResourceEnum::Roles.to_string(), &role_id),
                name: name.clone(),
                is_deleted: false,
                permissions,
                created_at: Some(get_iso_date()),
                updated_at: Some(get_iso_date()),
            };
            db.create::<RolesSchema>((ResourceEnum::Roles.to_string().as_str(), role_id)).content(role).await.unwrap(); // Corrected syntax
        }
    }

    app_state
}

// Helper function to create a test application (router)
fn app(app_state: AppState) -> Router {
    Router::new()
        .route("/v1/mentors/register", post(mentors_controller::post_register_mentor))
        .with_state(app_state)
}

#[tokio::test]
async fn test_register_new_user_as_mentor_success() {
    let app_state = setup_all_test_environment().await; // Use the new setup function
    let app = app(app_state.clone());

    let test_email = "newmentor@example.com";
    let test_password = "Password123!";
    let test_fullname = "New Mentor User";
    let test_phone = "1234567890";

    // Clean up before test
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;

    let dto = MentorUserRegisterRequestDto {
        email: test_email.to_string(),
        password: test_password.to_string(),
        fullname: test_fullname.to_string(),
        phone_number: test_phone.to_string(),
        identity_and_verification: imphnen_dimentorin::mentors_dto::IdentityAndVerification {
            legal_name: "Legal Name".to_string(),
            gender: Some("Laki-laki".to_string()),
            domicile: Some("Jakarta Selatan".to_string()),
            identity_document_url: "http://example.com/id.pdf".to_string(),
            phone_for_verification: "0987654321".to_string(),
        },
        professional_profile: imphnen_dimentorin::mentors_dto::ProfessionalProfile {
            bio: "Experienced professional seeking to mentor others in software development.".to_string(),
            last_education: Some("S1".to_string()),
            linkedin_url: Some("http://linkedin.com/in/mentor".to_string()),
            github_url: None,
            cv_url: None,
            portfolio_url: Some("http://example.com/portfolio".to_string()),
            industries: vec!["Technology".to_string()],
            expertise: vec!["Rust".to_string(), "Debugging".to_string()],
            languages: vec!["English".to_string()],
            current_company: "Acme Corp".to_string(),
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
    };

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

    let mentor_register_response: MentorRegisterResponseDto =
        crate::common::response_helpers::parse_response(response, 8192).await;

    assert!(!mentor_register_response.id.is_empty());
    assert!(!mentor_register_response.user_id.is_empty());
    assert_eq!(mentor_register_response.email, Some(test_email.to_string()));
    assert_eq!(mentor_register_response.status, "pending".to_string()); // New mentors should be pending verification

    // Verify user created and has mentor role, and is inactive
    let user = user_repo.query_user_by_email(test_email.to_string()).await.unwrap();
    assert_eq!(user.email, test_email);
    assert_eq!(user.is_active, false); // Should be inactive awaiting OTP
    let role_repo = RolesRepository::new(&app_state); // Use RolesRepository to get role
    let mentor_role = role_repo.query_role_by_name(RolesEnum::Mentor.to_string()).await.unwrap();
    assert_eq!(user.role.to_raw(), mentor_role.id.to_raw()); // Compare raw IDs

    // Clean up after test
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_register_existing_user_as_mentor_success() {
    let app_state = setup_all_test_environment().await; // Use the new setup function
    let app = app(app_state.clone());

    let test_email = "existinguser_mentor@example.com";
    let test_password = "Password123!";
    let test_fullname = "Existing User Becoming Mentor";
    let test_phone = "1122334455";

    // Clean up and create an existing user
    let user_repo = UsersRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;

    // Manually create a user with 'User' role first
    let role_repo = RolesRepository::new(&app_state); // Use RolesRepository to get role
    let user_role_id_item = role_repo.query_role_by_name(RolesEnum::User.to_string()).await.unwrap(); // Get RolesDetailItemDto
    let user_role_id_thing = make_thing(&ResourceEnum::Roles.to_string(), &user_role_id_item.id); // Convert to Thing
    let hashed_password = hash_password(test_password).unwrap();
    user_repo.query_create_user(imphnen_iam::UsersSchema {
        id: make_thing(&ResourceEnum::Users.to_string(), &Uuid::new_v4().to_string()),
        email: test_email.to_string(),
        password: hashed_password,
        fullname: "Original User Name".to_string(),
        phone_number: "0000000000".to_string(),
        is_active: true, // Initially active
        role: user_role_id_thing,
        created_at: get_iso_date(),
        updated_at: get_iso_date(),
        ..Default::default()
    }).await.unwrap();

    let dto = MentorUserRegisterRequestDto {
        email: test_email.to_string(),
        password: test_password.to_string(), // Keep password same for simplicity in test
        fullname: test_fullname.to_string(),
        phone_number: test_phone.to_string(),
        identity_and_verification: imphnen_dimentorin::mentors_dto::IdentityAndVerification {
            legal_name: "Legal Name Updated".to_string(),
            gender: Some("Perempuan".to_string()),
            domicile: Some("Surabaya".to_string()),
            identity_document_url: "http://example.com/id_updated.pdf".to_string(),
            phone_for_verification: "0987654322".to_string(),
        },
        professional_profile: imphnen_dimentorin::mentors_dto::ProfessionalProfile {
            bio: "Existing user now a mentor.".to_string(),
            last_education: Some("S2".to_string()),
            linkedin_url: Some("http://linkedin.com/in/mentor_existing".to_string()),
            github_url: None,
            cv_url: None,
            portfolio_url: Some("http://example.com/portfolio_existing".to_string()),
            industries: vec!["Education".to_string()],
            expertise: vec!["Marketing".to_string()],
            languages: vec!["Indonesian".to_string()],
            current_company: "New Company".to_string(),
            current_role: "Manager".to_string(),
            years_of_experience: 10,
        },
        mentoring_logistics: imphnen_dimentorin::mentors_dto::MentoringLogistics {
            topics_of_interest: vec!["Business Strategy".to_string()],
            preferred_mentee_level: vec!["Experienced".to_string()],
            preferred_mentoring_formats: vec!["Offline".to_string()],
            availability_commitment: "10 hours/month".to_string(),
            mentoring_rate_amount: 200,
        },
    };

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

    let mentor_register_response: MentorRegisterResponseDto =
        crate::common::response_helpers::parse_response(response, 8192).await;

    assert!(!mentor_register_response.id.is_empty());
    assert!(!mentor_register_response.user_id.is_empty());
    assert_eq!(mentor_register_response.email, Some(test_email.to_string()));
    assert_eq!(mentor_register_response.status, "pending".to_string()); // Should be pending verification

    // Verify user updated and has mentor role, and is inactive
    let user = user_repo.query_user_by_email(test_email.to_string()).await.unwrap();
    assert_eq!(user.email, test_email);
    assert_eq!(user.fullname, test_fullname);
    assert_eq!(user.phone_number, test_phone);
    assert_eq!(user.is_active, false); // Should be inactive awaiting OTP
    let role_repo = RolesRepository::new(&app_state); // Use RolesRepository to get role
    let mentor_role = role_repo.query_role_by_name(RolesEnum::Mentor.to_string()).await.unwrap();
    assert_eq!(user.role.to_raw(), mentor_role.id.to_raw()); // Compare raw IDs

    // Clean up after test
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
}

#[tokio::test]
async fn test_register_mentor_already_has_profile() {
    let app_state = setup_all_test_environment().await; // Use the new setup function
    let app = app(app_state.clone());

    let test_email = "existing_mentor_profile@example.com";
    let test_password = "Password123!";
    let test_fullname = "Existing Mentor Profile";
    let test_phone = "5551234567";

    // Clean up and create a user that is already a mentor
    let user_repo = UsersRepository::new(&app_state);
    let mentor_repo = imphnen_dimentorin::mentors_repository::MentorsRepository::new(&app_state);
    let _ = user_repo.query_delete_user(test_email.to_string()).await;

    // Manually create a user with 'Mentor' role
    let role_repo = RolesRepository::new(&app_state); // Use RolesRepository to get role
    let mentor_role_thing = role_repo.query_role_by_name(RolesEnum::Mentor.to_string()).await.unwrap().id;
    let hashed_password = imphnen_utils::hash_password(test_password).unwrap();
    let user_id = imphnen_utils::make_thing(&ResourceEnum::Users.to_string(), &Uuid::new_v4().to_string());
    user_repo.query_create_user(imphnen_iam::UsersSchema {
        id: user_id.clone(),
        email: test_email.to_string(),
        password: hashed_password,
        fullname: test_fullname.to_string(),
        phone_number: test_phone.to_string(),
        is_active: true,
        role: mentor_role_thing.clone(),
        created_at: imphnen_utils::get_iso_date(),
        updated_at: imphnen_utils::get_iso_date(),
        // Removed mentor_user_id
    }).await.unwrap();

    // Manually create a mentor profile for this user
    let existing_mentor_profile_id = imphnen_utils::make_thing(&ResourceEnum::Mentors.to_string(), &Uuid::new_v4().to_string());
    mentor_repo.query_create_mentor(imphnen_dimentorin::mentors_schema::MentorSchema {
        id: existing_mentor_profile_id.clone(),
        user_id: Some(user_id.clone()),
        email: Some(test_email.to_string()),
        legal_name: "Existing Mentor Legal Name".to_string(),
        gender: Some("Laki-laki".to_string()),
        domicile: Some("Bandung".to_string()),
        identity_document_url: "http://example.com/existing_id.pdf".to_string(),
        phone_for_verification: "1234567890".to_string(),
        bio: "Already an existing mentor in the system.".to_string(),
        last_education: Some("S3".to_string()),
        linkedin_url: None,
        github_url: None,
        cv_url: None,
        portfolio_url: Some("http://example.com/existing_portfolio".to_string()),
        industries: vec!["Finance".to_string()],
        expertise: vec!["Investments".to_string()],
        languages: vec!["English".to_string()],
        current_company: "Finance Corp".to_string(),
        current_role: "Analyst".to_string(),
        years_of_experience: 7,
        topics_of_interest: vec!["Stocks".to_string()],
        preferred_mentee_level: vec!["All".to_string()],
        preferred_mentoring_formats: vec!["Any".to_string()],
        availability_commitment: "Flexible".to_string(),
        mentoring_rate: imphnen_dimentorin::mentors_dto::MentoringRate::default(),
        status: "Approved".to_string(),
        is_deleted: false,
        created_at: imphnen_utils::get_iso_date(),
        updated_at: imphnen_utils::get_iso_date(),
    }).await.unwrap();

    // Update user to link mentor profile
    let user_after_mentor_creation_dto = user_repo.query_user_by_email(test_email.to_string()).await.unwrap();
    let mut user_after_mentor_creation_schema = UsersSchema::from(user_after_mentor_creation_dto);
    user_after_mentor_creation_schema = user_after_mentor_creation_schema.update_mentor_id(Some(existing_mentor_profile_id.clone().to_raw()));
    user_repo.query_update_user(user_after_mentor_creation_schema).await.unwrap();

    let dto = MentorUserRegisterRequestDto {
        email: test_email.to_string(),
        password: test_password.to_string(),
        fullname: test_fullname.to_string(),
        phone_number: test_phone.to_string(),
        identity_and_verification: imphnen_dimentorin::mentors_dto::IdentityAndVerification {
            legal_name: "Legal Name".to_string(),
            gender: Some("Perempuan".to_string()),
            domicile: Some("Yogyakarta".to_string()),
            identity_document_url: "http://example.com/id.pdf".to_string(),
            phone_for_verification: "0987654321".to_string(),
        },
        professional_profile: imphnen_dimentorin::mentors_dto::ProfessionalProfile {
            bio: "Experienced professional seeking to mentor others in software development.".to_string(),
            last_education: Some("SMA".to_string()),
            linkedin_url: Some("http://linkedin.com/in/mentor".to_string()),
            github_url: None,
            cv_url: None,
            portfolio_url: Some("http://example.com/portfolio_new".to_string()),
            industries: vec!["Technology".to_string()],
            expertise: vec!["Rust".to_string(), "Debugging".to_string()],
            languages: vec!["English".to_string()],
            current_company: "Acme Corp".to_string(),
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
    };

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

    assert_eq!(response.status(), StatusCode::CONFLICT); // Expecting conflict

    // Clean up after test
    let _ = user_repo.query_delete_user(test_email.to_string()).await;
    let _ = mentor_repo.query_delete_mentor(&existing_mentor_profile_id.to_raw()).await;
}

#[tokio::test]
async fn test_register_mentor_invalid_email_format() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "invalid-email"; // Invalid email format
    let test_password = "Password123!";
    let test_fullname = "Test User";
    let test_phone = "1234567890";

    let dto = MentorUserRegisterRequestDto {
        email: test_email.to_string(),
        password: test_password.to_string(),
        fullname: test_fullname.to_string(),
        phone_number: test_phone.to_string(),
        identity_and_verification: imphnen_dimentorin::mentors_dto::IdentityAndVerification {
            legal_name: "Legal Name".to_string(),
            gender: Some("Laki-laki".to_string()),
            domicile: Some("Jakarta Selatan".to_string()),
            identity_document_url: "http://example.com/id.pdf".to_string(),
            phone_for_verification: "0987654321".to_string(),
        },
        professional_profile: imphnen_dimentorin::mentors_dto::ProfessionalProfile {
            bio: "Experienced professional seeking to mentor others in software development.".to_string(),
            last_education: Some("S1".to_string()),
            linkedin_url: Some("http://linkedin.com/in/mentor".to_string()),
            github_url: None,
            cv_url: None,
            portfolio_url: Some("http://example.com/portfolio".to_string()),
            industries: vec!["Technology".to_string()],
            expertise: vec!["Rust".to_string(), "Debugging".to_string()],
            languages: vec!["English".to_string()],
            current_company: "Acme Corp".to_string(),
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
    };

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
    let error_response: serde_json::Value =
        crate::common::response_helpers::parse_response_value(response, 8192).await;
    assert!(error_response["message"].as_str().unwrap().contains("email"));
}

#[tokio::test]
async fn test_register_mentor_weak_password() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "weakpass@example.com";
    let test_password = "weak"; // Weak password
    let test_fullname = "Test User";
    let test_phone = "1234567890";

    let dto = MentorUserRegisterRequestDto {
        email: test_email.to_string(),
        password: test_password.to_string(),
        fullname: test_fullname.to_string(),
        phone_number: test_phone.to_string(),
        identity_and_verification: imphnen_dimentorin::mentors_dto::IdentityAndVerification {
            legal_name: "Legal Name".to_string(),
            gender: Some("Laki-laki".to_string()),
            domicile: Some("Jakarta Selatan".to_string()),
            identity_document_url: "http://example.com/id.pdf".to_string(),
            phone_for_verification: "0987654321".to_string(),
        },
        professional_profile: imphnen_dimentorin::mentors_dto::ProfessionalProfile {
            bio: "Experienced professional seeking to mentor others in software development.".to_string(),
            last_education: Some("S1".to_string()),
            linkedin_url: Some("http://linkedin.com/in/mentor".to_string()),
            github_url: None,
            cv_url: None,
            portfolio_url: Some("http://example.com/portfolio".to_string()),
            industries: vec!["Technology".to_string()],
            expertise: vec!["Rust".to_string(), "Debugging".to_string()],
            languages: vec!["English".to_string()],
            current_company: "Acme Corp".to_string(),
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
    };

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
    let v = crate::common::response_helpers::parse_response_value(response, 8192).await;
    assert!(v.get("message").and_then(|m| m.as_str()).map(|s| s.contains("password")).unwrap_or(false));
}

#[tokio::test]
async fn test_register_mentor_missing_fullname() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "missingfullname@example.com";
    let test_password = "Password123!";
    let test_phone = "1234567890";

    let dto = MentorUserRegisterRequestDto {
        email: test_email.to_string(),
        password: test_password.to_string(),
        fullname: "".to_string(), // Missing fullname
        phone_number: test_phone.to_string(),
        identity_and_verification: imphnen_dimentorin::mentors_dto::IdentityAndVerification {
            legal_name: "Legal Name".to_string(),
            gender: Some("Laki-laki".to_string()),
            domicile: Some("Jakarta Selatan".to_string()),
            identity_document_url: "http://example.com/id.pdf".to_string(),
            phone_for_verification: "0987654321".to_string(),
        },
        professional_profile: imphnen_dimentorin::mentors_dto::ProfessionalProfile {
            bio: "Experienced professional seeking to mentor others in software development.".to_string(),
            last_education: Some("S1".to_string()),
            linkedin_url: Some("http://linkedin.com/in/mentor".to_string()),
            github_url: None,
            cv_url: None,
            portfolio_url: Some("http://example.com/portfolio".to_string()),
            industries: vec!["Technology".to_string()],
            expertise: vec!["Rust".to_string(), "Debugging".to_string()],
            languages: vec!["English".to_string()],
            current_company: "Acme Corp".to_string(),
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
    };

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
    let v = crate::common::response_helpers::parse_response_value(response, 8192).await;
    assert!(v.get("message").and_then(|m| m.as_str()).map(|s| s.contains("fullname")).unwrap_or(false));
}

#[tokio::test]
async fn test_register_mentor_missing_phone_number() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "missingphone@example.com";
    let test_password = "Password123!";
    let test_fullname = "Test User";

    let dto = MentorUserRegisterRequestDto {
        email: test_email.to_string(),
        password: test_password.to_string(),
        fullname: test_fullname.to_string(),
        phone_number: "".to_string(), // Missing phone number
        identity_and_verification: imphnen_dimentorin::mentors_dto::IdentityAndVerification {
            legal_name: "Legal Name".to_string(),
            gender: Some("Laki-laki".to_string()),
            domicile: Some("Jakarta Selatan".to_string()),
            identity_document_url: "http://example.com/id.pdf".to_string(),
            phone_for_verification: "0987654321".to_string(),
        },
        professional_profile: imphnen_dimentorin::mentors_dto::ProfessionalProfile {
            bio: "Experienced professional seeking to mentor others in software development.".to_string(),
            last_education: Some("S1".to_string()),
            linkedin_url: Some("http://linkedin.com/in/mentor".to_string()),
            github_url: None,
            cv_url: None,
            portfolio_url: Some("http://example.com/portfolio".to_string()),
            industries: vec!["Technology".to_string()],
            expertise: vec!["Rust".to_string(), "Debugging".to_string()],
            languages: vec!["English".to_string()],
            current_company: "Acme Corp".to_string(),
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
    };

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
    let v = crate::common::response_helpers::parse_response_value(response, 8192).await;
    assert!(v.get("message").and_then(|m| m.as_str()).map(|s| s.contains("phone_number")).unwrap_or(false));
}

#[tokio::test]
async fn test_register_mentor_missing_identity_document_url() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "missingdocurl@example.com";
    let test_password = "Password123!";
    let test_fullname = "Test User";
    let test_phone = "1234567890";

    let dto = MentorUserRegisterRequestDto {
        email: test_email.to_string(),
        password: test_password.to_string(),
        fullname: test_fullname.to_string(),
        phone_number: test_phone.to_string(),
        identity_and_verification: imphnen_dimentorin::mentors_dto::IdentityAndVerification {
            legal_name: "Legal Name".to_string(),
            gender: Some("Laki-laki".to_string()),
            domicile: Some("Jakarta Selatan".to_string()),
            identity_document_url: "".to_string(), // Missing identity document url
            phone_for_verification: "0987654321".to_string(),
        },
        professional_profile: imphnen_dimentorin::mentors_dto::ProfessionalProfile {
            bio: "Experienced professional seeking to mentor others in software development.".to_string(),
            last_education: Some("S1".to_string()),
            linkedin_url: Some("http://linkedin.com/in/mentor".to_string()),
            github_url: None,
            cv_url: None,
            portfolio_url: Some("http://example.com/portfolio".to_string()),
            industries: vec!["Technology".to_string()],
            expertise: vec!["Rust".to_string(), "Debugging".to_string()],
            languages: vec!["English".to_string()],
            current_company: "Acme Corp".to_string(),
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
    };

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
    let error_response: serde_json::Value =
        crate::common::response_helpers::parse_response_value(response, 8192).await;
    assert!(error_response["message"].as_str().unwrap().contains("identity_document_url"));
}

#[tokio::test]
async fn test_register_mentor_invalid_phone_for_verification_format() {
    let app_state = setup_all_test_environment().await;
    let app = app(app_state.clone());

    let test_email = "invalidphoneveri@example.com";
    let test_password = "Password123!";
    let test_fullname = "Test User";
    let test_phone = "1234567890";

    let dto = MentorUserRegisterRequestDto {
        email: test_email.to_string(),
        password: test_password.to_string(),
        fullname: test_fullname.to_string(),
        phone_number: test_phone.to_string(),
        identity_and_verification: imphnen_dimentorin::mentors_dto::IdentityAndVerification {
            legal_name: "Legal Name".to_string(),
            gender: Some("Laki-laki".to_string()),
            domicile: Some("Jakarta Selatan".to_string()),
            identity_document_url: "http://example.com/id.pdf".to_string(),
            phone_for_verification: "invalid".to_string(), // Invalid phone for verification
        },
        professional_profile: imphnen_dimentorin::mentors_dto::ProfessionalProfile {
            bio: "Experienced professional seeking to mentor others in software development.".to_string(),
            last_education: Some("S1".to_string()),
            linkedin_url: Some("http://linkedin.com/in/mentor".to_string()),
            github_url: None,
            cv_url: None,
            portfolio_url: Some("http://example.com/portfolio".to_string()),
            industries: vec!["Technology".to_string()],
            expertise: vec!["Rust".to_string(), "Debugging".to_string()],
            languages: vec!["English".to_string()],
            current_company: "Acme Corp".to_string(),
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
    };

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
    let v = crate::common::response_helpers::parse_response_value(response, 8192).await;
    assert!(v.get("message").and_then(|m| m.as_str()).map(|s| s.contains("phone_for_verification")).unwrap_or(false));
}
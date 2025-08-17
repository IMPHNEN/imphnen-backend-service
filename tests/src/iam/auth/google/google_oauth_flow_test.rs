#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        Router,
    };
    use http_body_util::BodyExt; // for `collect` and `to_bytes`
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`
    use mockall::predicate::*;
    use mockall::mock;
    use serde_json::json;

    use imphnen_iam::v1::auth::{AuthLoginResponsetDto, TokenDto};
    use imphnen_iam::v1::auth::google::google_oauth_controller::GoogleOauthController;
    use imphnen_iam::v1::auth::google::google_oauth_service::{AuthRequest, GoogleOauthService, GoogleOauthServiceImpl};
    use imphnen_iam::v1::users::users_dto::{UsersDetailItemDto, UsersCreateRequestDto}; // Corrected: removed UserDto alias, used UsersCreateRequestDto
    use imphnen_entities::error_dto::ErrorResponse;
    use imphnen_libs::jsonwebtoken::generate_jwt;
    use imphnen_libs::enviroment::{ENV, Env}; // Import ENV and Env

    mock! {
        pub GoogleOauthServiceMock {}
        impl GoogleOauthService for GoogleOauthServiceMock {
            fn new() -> Self;
            fn with_services(auth_service: crate::v1::auth::auth_service::AuthService, users_service: crate::v1::users::users_service::UsersService, env: &'static Env) -> Self; // Updated signature
            fn google_oauth_client(&self) -> oauth2::basic::BasicClient;
            fn generate_auth_url(&self) -> (url::Url, oauth2::CsrfToken);
            async fn google_oauth_callback(&self, auth_request: AuthRequest, app_state: &crate::AppState) -> anyhow::Result<(crate::v1::users::users_dto::UsersDetailItemDto, crate::v1::auth::TokenDto), imphnen_entities::error_dto::error::Error>;
        }
    }

    mock! {
        pub AuthServiceMock {}
        impl crate::v1::auth::auth_service::AuthServiceTrait for AuthServiceMock {
            async fn mutation_login(payload: crate::v1::auth::AuthLoginRequestDto, state: &crate::AppState) -> axum::response::Response;
            async fn mutation_mentor_login(payload: crate::v1::auth::AuthLoginRequestDto, state: &crate::AppState) -> axum::response::Response;
            async fn mutation_register(payload: crate::v1::auth::AuthRegisterRequestDto, state: &crate::AppState) -> axum::response::Response;
            async fn mutation_resend_otp(payload: crate::v1::auth::AuthResendOtpRequestDto, state: &crate::AppState) -> axum::response::Response;
            async fn mutation_refresh_token(payload: crate::v1::auth::AuthRefreshTokenRequestDto) -> axum::response::Response;
            async fn mutation_forgot_password(payload: crate::v1::auth::AuthResendOtpRequestDto, state: &crate::AppState) -> axum::response::Response;
            async fn mutation_verify_email(payload: crate::v1::auth::AuthVerifyEmailRequestDto, state: &crate::AppState) -> axum::response::Response;
            async fn mutation_new_password(payload: crate::v1::auth::AuthNewPasswordRequestDto, state: &crate::AppState) -> axum::response::Response;
        }
    }

    mock! {
        pub UsersServiceMock {}
        impl crate::v1::users::users_service::UsersServiceTrait for UsersServiceMock {
            async fn get_user_list(state: &crate::AppState, meta: crate::MetaRequestDto) -> axum::response::Response;
            async fn get_user_by_id(state: &crate::AppState, id: String) -> axum::response::Response;
            async fn get_user_me(headers: http::HeaderMap, state: &crate::AppState) -> axum::response::Response;
            async fn create_user(state: &crate::AppState, new_user: crate::v1::users::UsersCreateRequestDto) -> axum::response::Response;
            async fn update_user(state: &crate::AppState, id: String, user: crate::v1::users::UsersUpdateRequestDto) -> axum::response::Response;
            async fn update_user_me(state: &crate::AppState, headers: http::HeaderMap, user: crate::v1::users::UsersUpdateRequestDto) -> axum::response::Response;
            async fn set_user_active_status(state: &crate::AppState, id: String, payload: crate::v1::users::UsersActiveInactiveRequestDto) -> axum::response::Response;
            async fn update_user_password(state: &crate::AppState, email: String, payload: crate::v1::users::UsersSetNewPasswordRequestDto) -> axum::response::Response;
            async fn get_user_by_mentor_id(state: &crate::AppState, mentor_id: String) -> axum::response::Response;
            async fn delete_user(state: &crate::AppState, id: String) -> axum::response::Response;
            async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<UsersDetailItemDto>>; // Updated return type
            async fn create_user_by_dto(&self, new_user: UsersCreateRequestDto) -> anyhow::Result<UsersDetailItemDto>; // Updated return type
        }
    }

    async fn setup_app_with_mocked_google_oauth_service(
        mock_google_oauth_service: MockGoogleOauthServiceMock,
    ) -> Router {
        let google_oauth_controller =
            GoogleOauthController::with_service(mock_google_oauth_service);
        google_oauth_controller.get_routes()
    }

    #[tokio::test]
    async fn google_login_redirects_to_google_auth_url() {
        let app = setup_app_with_mocked_google_oauth_service(
            GoogleOauthServiceImpl::with_services(
                AuthServiceMock::new(),
                UsersServiceMock::new(),
                &ENV, // Pass ENV
            )
        ).await;

        let request = Request::builder()
            .uri("/google/login")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::FOUND);

        let location_header = response.headers().get(http::header::LOCATION).unwrap();
        let location_str = location_header.to_str().unwrap();

        assert!(location_str.starts_with("https://accounts.google.com/o/oauth2/v2/auth"));
        assert!(location_str.contains("client_id="));
        assert!(location_str.contains("redirect_uri="));
        assert!(location_str.contains("response_type=code"));
        assert!(location_str.contains("scope="));
        assert!(location_str.contains("state="));
        assert!(location_str.contains("access_type=offline"));
        assert!(location_str.contains("prompt=consent"));
    }

    #[tokio::test]
    async fn google_callback_new_user_creates_user_and_returns_login_response() {
        let mut mock_google_oauth_service = MockGoogleOauthServiceMock::new();
        let mut mock_users_service = UsersServiceMock::new(); // Changed from MockUsersServiceMock to UsersServiceMock

        let user_email = "new.user@example.com".to_string();
        let expected_access_token = generate_jwt(&user_email, "test_user_id", vec![]).unwrap();
        let expected_refresh_token = generate_jwt(&user_email, "test_user_id", vec![]).unwrap();

        let expected_response_dto = AuthLoginResponsetDto {
            token: TokenDto {
                access_token: expected_access_token.clone(),
                refresh_token: expected_refresh_token.clone(),
            },
            user: UsersDetailItemDto {
                id: "test_user_id".to_string(),
                email: user_email.clone(),
                fullname: "Test User".to_string(), // Updated field
                phone_number: "1234567890".to_string(), // Updated field
                is_active: true,
                gender: None, // Added field
                birthdate: None, // Added field
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                role: imphnen_iam::v1::roles::roles_dto::RolesDetailItemDto {
                    id: "default_role_id".to_string(),
                    name: "User".to_string(),
                    permissions: vec![],
                    created_at: chrono::Utc::now().to_rfc3339(),
                    updated_at: chrono::Utc::now().to_rfc3339(),
                },
            },
        };

        mock_google_oauth_service.expect_google_oauth_callback()
            .with(eq(AuthRequest { code: "some_code".to_string(), state: "some_state".to_string() }))
            .returning(move |_| Ok(expected_response_dto.clone()));

        mock_users_service.expect_get_user_by_email()
            .with(eq(user_email.clone()))
            .returning(|_| Ok(None)); // Simulate no existing user
        
        mock_users_service.expect_create_user_by_dto()
            .returning(|create_user_dto| {
                Ok(UsersDetailItemDto { // Changed to UsersDetailItemDto
                    id: "test_user_id".to_string(),
                    email: create_user_dto.email,
                    fullname: create_user_dto.fullname, // Updated field
                    phone_number: create_user_dto.phone_number, // Updated field
                    is_active: create_user_dto.is_active,
                    gender: None, // Added field
                    birthdate: None, // Added field
                    created_at: chrono::Utc::now().to_rfc3339(),
                    updated_at: chrono::Utc::now().to_rfc3339(),
                    role: imphnen_iam::v1::roles::roles_dto::RolesDetailItemDto {
                        id: "default_role_id".to_string(),
                        name: "User".to_string(),
                        permissions: vec![],
                        created_at: chrono::Utc::now().to_rfc3339(),
                        updated_at: chrono::Utc::now().to_rfc3339(),
                    },
                })
            });

        let app = setup_app_with_mocked_google_oauth_service(
            GoogleOauthServiceImpl::with_services(
                AuthServiceMock::new(),
                mock_users_service,
                &ENV, // Pass ENV here
            )
        ).await;

        let request = Request::builder()
            .uri("/google/callback?code=some_code&state=some_state")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json_body: AuthLoginResponsetDto = serde_json::from_slice(&body).unwrap();
        assert_eq!(json_body.token.access_token, expected_access_token);
        assert_eq!(json_body.token.refresh_token, expected_refresh_token);
        assert_eq!(json_body.user.email, user_email);
    }

    #[tokio::test]
    async fn google_callback_existing_user_returns_login_response() {
        let mut mock_google_oauth_service = MockGoogleOauthServiceMock::new();
        let mut mock_users_service = UsersServiceMock::new(); // Changed from MockUsersServiceMock to UsersServiceMock

        let user_email = "existing.user@example.com".to_string();
        let expected_access_token = generate_jwt(&user_email, "existing_user_id", vec![]).unwrap();
        let expected_refresh_token = generate_jwt(&user_email, "existing_user_id", vec![]).unwrap();

        let existing_user_dto = UsersDetailItemDto { // Changed from UserDto to UsersDetailItemDto
            id: "existing_user_id".to_string(),
            email: user_email.clone(),
            fullname: "Existing User".to_string(), // Updated field
            phone_number: "0987654321".to_string(), // Updated field
            is_active: true,
            gender: None, // Added field
            birthdate: None, // Added field
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            role: imphnen_iam::v1::roles::roles_dto::RolesDetailItemDto {
                id: "default_role_id".to_string(),
                name: "User".to_string(),
                permissions: vec![],
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            },
        };

        let expected_response_dto = AuthLoginResponsetDto {
            token: TokenDto {
                access_token: expected_access_token.clone(),
                refresh_token: expected_refresh_token.clone(),
            },
            user: existing_user_dto.clone(), // Cloned
        };

        mock_google_oauth_service.expect_google_oauth_callback()
            .with(eq(AuthRequest { code: "some_code".to_string(), state: "some_state".to_string() }))
            .returning(move |_| Ok(expected_response_dto.clone()));

        mock_users_service.expect_get_user_by_email()
            .with(eq(user_email.clone()))
            .returning(move |_| {
                Ok(Some(UsersDetailItemDto { // Changed to UsersDetailItemDto
                    id: "existing_user_id".to_string(),
                    email: user_email.clone(),
                    fullname: "Existing User".to_string(), // Updated field
                    phone_number: "0987654321".to_string(), // Updated field
                    is_active: true,
                    gender: None, // Added field
                    birthdate: None, // Added field
                    created_at: chrono::Utc::now().to_rfc3339(),
                    updated_at: chrono::Utc::now().to_rfc3339(),
                    role: imphnen_iam::v1::roles::roles_dto::RolesDetailItemDto {
                        id: "default_role_id".to_string(),
                        name: "User".to_string(),
                        permissions: vec![],
                        created_at: chrono::Utc::now().to_rfc3339(),
                        updated_at: chrono::Utc::now().to_rfc3339(),
                    },
                }))
            }); // Simulate existing user

        // Ensure create_user_by_dto is NOT called for existing user
        mock_users_service.expect_create_user_by_dto().times(0);

        let app = setup_app_with_mocked_google_oauth_service(
            GoogleOauthServiceImpl::with_services(
                AuthServiceMock::new(),
                mock_users_service,
                &ENV, // Pass ENV here
            )
        ).await;

        let request = Request::builder()
            .uri("/google/callback?code=some_code&state=some_state")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json_body: AuthLoginResponsetDto = serde_json::from_slice(&body).unwrap();
        assert_eq!(json_body.token.access_token, expected_access_token);
        assert_eq!(json_body.token.refresh_token, expected_refresh_token);
        assert_eq!(json_body.user.email, user_email);
    }
}
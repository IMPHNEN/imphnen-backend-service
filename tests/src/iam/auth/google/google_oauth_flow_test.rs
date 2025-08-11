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

    use imphnen_iam::v1::auth::google::google_oauth_controller::GoogleOauthController;
    use imphnen_iam::v1::auth::google::google_oauth_service::{AuthRequest, GoogleOauthService, GoogleOauthServiceImpl};
    use imphnen_iam::v1::users::users_dto::{UserDto, CreateUserDto};
    use imphnen_entities::error_dto::ErrorResponse;
    use imphnen_libs::jsonwebtoken::generate_jwt;

    mock! {
        pub GoogleOauthServiceMock {}
        impl GoogleOauthService for GoogleOauthServiceMock {
            fn new() -> Self;
            fn with_services(auth_service: crate::v1::auth::auth_service::AuthService, users_service: crate::v1::users::users_service::UsersService) -> Self;
            fn google_oauth_client(&self) -> oauth2::basic::BasicClient;
            fn generate_auth_url(&self) -> (url::Url, oauth2::CsrfToken);
            async fn google_oauth_callback(&self, auth_request: AuthRequest) -> anyhow::Result<String, ErrorResponse>;
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
            async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<UserDto>>;
            async fn create_user_by_dto(&self, new_user: CreateUserDto) -> anyhow::Result<UserDto>;
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
        let app = setup_app_with_mocked_google_oauth_service(GoogleOauthServiceImpl::new()).await;

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
    async fn google_callback_new_user_creates_user_and_returns_jwt() {
        let mut mock_google_oauth_service = MockGoogleOauthServiceMock::new();
        let mut mock_users_service = MockUsersServiceMock::new();

        let expected_jwt = generate_jwt("test_user_id").unwrap();
        let user_email = "new.user@example.com".to_string();

        mock_google_oauth_service.expect_google_oauth_callback()
            .with(eq(AuthRequest { code: "some_code".to_string(), state: "some_state".to_string() }))
            .returning(move |_| Ok(expected_jwt.clone()));

        mock_users_service.expect_get_user_by_email()
            .with(eq(user_email.clone()))
            .returning(|_| Ok(None)); // Simulate no existing user

        mock_users_service.expect_create_user_by_dto()
            .returning(|create_user_dto| {
                Ok(UserDto {
                    id: "test_user_id".to_string(),
                    email: create_user_dto.email,
                    username: create_user_dto.username,
                    first_name: create_user_dto.first_name,
                    last_name: create_user_dto.last_name,
                    is_active: create_user_dto.is_active,
                    is_email_verified: create_user_dto.is_email_verified,
                    created_at: chrono::Utc::now().to_rfc3339(),
                    updated_at: chrono::Utc::now().to_rfc3339(),
                })
            });

        let app = setup_app_with_mocked_google_oauth_service(
            GoogleOauthServiceImpl::with_services(
                AuthServiceMock::new(), // Not directly used by google_oauth_callback logic, but required by trait
                mock_users_service,
            )
        ).await;

        let request = Request::builder()
            .uri("/google/callback?code=some_code&state=some_state")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json_body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json_body["token"], expected_jwt);
    }

    #[tokio::test]
    async fn google_callback_existing_user_returns_jwt() {
        let mut mock_google_oauth_service = MockGoogleOauthServiceMock::new();
        let mut mock_users_service = MockUsersServiceMock::new();

        let expected_jwt = generate_jwt("existing_user_id").unwrap();
        let user_email = "existing.user@example.com".to_string();

        mock_google_oauth_service.expect_google_oauth_callback()
            .with(eq(AuthRequest { code: "some_code".to_string(), state: "some_state".to_string() }))
            .returning(move |_| Ok(expected_jwt.clone()));

        mock_users_service.expect_get_user_by_email()
            .with(eq(user_email.clone()))
            .returning(|_| {
                Ok(Some(UserDto {
                    id: "existing_user_id".to_string(),
                    email: user_email.clone(),
                    username: Some("existinguser".to_string()),
                    first_name: Some("Existing".to_string()),
                    last_name: Some("User".to_string()),
                    is_active: Some(true),
                    is_email_verified: Some(true),
                    created_at: chrono::Utc::now().to_rfc3339(),
                    updated_at: chrono::Utc::now().to_rfc3339(),
                }))
            }); // Simulate existing user

        // Ensure create_user_by_dto is NOT called for existing user
        mock_users_service.expect_create_user_by_dto().times(0);

        let app = setup_app_with_mocked_google_oauth_service(
            GoogleOauthServiceImpl::with_services(
                AuthServiceMock::new(), // Not directly used by google_oauth_callback logic, but required by trait
                mock_users_service,
            )
        ).await;

        let request = Request::builder()
            .uri("/google/callback?code=some_code&state=some_state")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json_body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json_body["token"], expected_jwt);
    }
}
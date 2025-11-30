#[cfg(test)]
mod rate_limiting_middleware_tests {
    use axum::{http::Request, middleware::Next, response::Response};
    use imphnen_libs::{AppState, environment::Environment, postgres::PostgresConnection};
    use imphnen_middleware::rate_limiting_middleware::{
        RateLimitConfig, RateLimitStore, TokenBucket, create_rate_limiting_middleware,
        auth_rate_limiting_middleware,
    };
    use std::{sync::Arc, time::Duration};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_token_bucket_basic_functionality() {
        let bucket = TokenBucket::new(5, 2); // Capacity 5, refill 2 per second
        
        // Should have full tokens initially
        assert_eq!(bucket.tokens, 5);
        
        // Consume some tokens
        assert!(bucket.try_consume());
        assert_eq!(bucket.tokens, 4);
        
        assert!(bucket.try_consume());
        assert_eq!(bucket.tokens, 3);
        
        assert!(bucket.try_consume());
        assert_eq!(bucket.tokens, 2);
        
        assert!(bucket.try_consume());
        assert_eq!(bucket.tokens, 1);
        
        assert!(bucket.try_consume());
        assert_eq!(bucket.tokens, 0);
        
        // Should not consume when empty
        assert!(!bucket.try_consume());
        assert_eq!(bucket.tokens, 0);
    }

    #[tokio::test]
    async fn test_token_bucket_refill() {
        let mut bucket = TokenBucket::new(3, 1); // Capacity 3, refill 1 per second
        
        // Consume all tokens
        for _ in 0..3 {
            assert!(bucket.try_consume());
        }
        
        assert!(!bucket.try_consume());
        assert_eq!(bucket.tokens, 0);
        
        // Wait for 1 second to allow refill
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        // Should have 1 token after refill
        bucket.refill_tokens();
        assert_eq!(bucket.tokens, 1);
        
        // Consume the refilled token
        assert!(bucket.try_consume());
        assert_eq!(bucket.tokens, 0);
        
        // Wait another second
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        // Should have another token
        bucket.refill_tokens();
        assert_eq!(bucket.tokens, 1);
    }

    #[tokio::test]
    async fn test_rate_limit_store_basic() {
        let config = RateLimitConfig::test();
        let store = Arc::new(RateLimitStore::new(config));
        
        let client_ip = "127.0.0.1";
        
        // First request should succeed
        let result = store.check_limit(client_ip).await;
        assert!(result.is_ok());
        
        // Multiple requests should succeed within limits
        for _ in 0..config.bucket_size {
            let result = store.check_limit(client_ip).await;
            assert!(result.is_ok());
        }
        
        // Next request should fail
        let result = store.check_limit(client_ip).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), axum::http::StatusCode::TOO_MANY_REQUESTS);
    }

    #[tokio::test]
    async fn test_rate_limit_store_window_reset() {
        let config = RateLimitConfig {
            max_requests: 10,
            window_duration: Duration::from_secs(2),
            bucket_size: 2,
            refill_rate: 1,
        };
        let store = Arc::new(RateLimitStore::new(config));
        
        let client_ip = "127.0.0.1";
        
        // Consume all tokens
        assert!(store.check_limit(client_ip).await.is_ok());
        assert!(store.check_limit(client_ip).await.is_ok());
        assert!(store.check_limit(client_ip).await.is_err());
        
        // Wait for window to reset
        tokio::time::sleep(Duration::from_secs(3)).await;
        
        // Should be able to make requests again
        assert!(store.check_limit(client_ip).await.is_ok());
        assert!(store.check_limit(client_ip).await.is_ok());
        assert!(store.check_limit(client_ip).await.is_err());
    }

    #[tokio::test]
    async fn test_different_clients_have_separate_limits() {
        let config = RateLimitConfig::test();
        let store = Arc::new(RateLimitStore::new(config));
        
        let client_ip_1 = "127.0.0.1";
        let client_ip_2 = "127.0.0.2";
        
        // Client 1 should be able to make requests
        for _ in 0..config.bucket_size {
            assert!(store.check_limit(client_ip_1).await.is_ok());
        }
        assert!(store.check_limit(client_ip_1).await.is_err());
        
        // Client 2 should still be able to make requests
        for _ in 0..config.bucket_size {
            assert!(store.check_limit(client_ip_2).await.is_ok());
        }
        assert!(store.check_limit(client_ip_2).await.is_err());
    }

    #[tokio::test]
    async fn test_auth_rate_limiting_middleware_success() {
        // Create a mock AppState with test environment
                let state = AppState {
                    postgres_connection: Arc::new(PostgresConnection::default()),
                    user_lookup_service: Default::default(),
                    auth_repository: Default::default(),
                    env: Environment::Test,
                };

        // Create a mock request to /auth/login
        let mut request = Request::builder()
            .uri("/v1/auth/login")
            .header("x-forwarded-for", "127.0.0.1")
            .body(())
            .unwrap();

        // Create a mock next service
        let next = Next::new(|req| async move {
            let response = Response::builder()
                .status(200)
                .body("Login successful")
                .unwrap();
            Ok::<_, axum::http::StatusCode>((req, response))
        });

        // Call the middleware
        let result = auth_rate_limiting_middleware(
            axum::Extension(state.clone()),
            request,
            next,
        ).await;

        // Should succeed
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_auth_rate_limiting_middleware_429() {
        // Create test configuration with very low limits for testing
        let config = RateLimitConfig {
            max_requests: 1,
            window_duration: Duration::from_secs(10),
            bucket_size: 1,
            refill_rate: 1,
        };

        // Create a mock AppState with test environment
                let state = AppState {
                    postgres_connection: Arc::new(PostgresConnection::default()),
                    user_lookup_service: Default::default(),
                    auth_repository: Default::default(),
                    env: Environment::Test,
                };

        // Create a mock request to /auth/login
        let mut request = Request::builder()
            .uri("/v1/auth/login")
            .header("x-forwarded-for", "127.0.0.1")
            .body(())
            .unwrap();

        // Create a mock next service
        let next = Next::new(|req| async move {
            let response = Response::builder()
                .status(200)
                .body("Login successful")
                .unwrap();
            Ok::<_, axum::http::StatusCode>((req, response))
        });

        // First request should succeed
        let result = auth_rate_limiting_middleware(
            axum::Extension(state.clone()),
            request.clone(),
            next.clone(),
        ).await;
        assert!(result.is_ok());

        // Second request should fail with 429
        let result = auth_rate_limiting_middleware(
            axum::Extension(state),
            request,
            next,
        ).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), 429);
        assert_eq!(response.headers().get("Retry-After").unwrap(), "60");
    }

    #[tokio::test]
    async fn test_non_auth_endpoints_not_rate_limited() {
        // Create a mock AppState with test environment
                let state = AppState {
                    postgres_connection: Arc::new(PostgresConnection::default()),
                    user_lookup_service: Default::default(),
                    auth_repository: Default::default(),
                    env: Environment::Test,
                };

        // Create a mock request to a non-auth endpoint
        let mut request = Request::builder()
            .uri("/v1/users/me")
            .header("x-forwarded-for", "127.0.0.1")
            .body(())
            .unwrap();

        // Create a mock next service
        let next = Next::new(|req| async move {
            let response = Response::builder()
                .status(200)
                .body("User data")
                .unwrap();
            Ok::<_, axum::http::StatusCode>((req, response))
        });

        // Call the middleware - should not apply rate limiting
        let result = auth_rate_limiting_middleware(
            axum::Extension(state),
            request,
            next,
        ).await;

        // Should succeed
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_environment_specific_configurations() {
        // Test development config
        let dev_config = RateLimitConfig::development();
        assert_eq!(dev_config.max_requests, 100);
        assert_eq!(dev_config.bucket_size, 50);
        assert_eq!(dev_config.refill_rate, 10);

        // Test production config
        let prod_config = RateLimitConfig::production();
        assert_eq!(prod_config.max_requests, 10);
        assert_eq!(prod_config.bucket_size, 5);
        assert_eq!(prod_config.refill_rate, 1);

        // Test test config
        let test_config = RateLimitConfig::test();
        assert_eq!(test_config.max_requests, 1000);
        assert_eq!(test_config.bucket_size, 100);
        assert_eq!(test_config.refill_rate, 20);
    }
}
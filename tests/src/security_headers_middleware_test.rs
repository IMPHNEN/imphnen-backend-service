use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};
use imphnen_libs::{AppState, ENV};
use imphnen_middleware::security_headers_middleware::security_headers_middleware;
use tower::ServiceExt;

#[tokio::test]
async fn test_security_headers_middleware_adds_headers() {
    // Create a mock request
    let req = Request::builder()
        .uri("/test")
        .body(axum::body::empty())
        .unwrap();

    // Create a mock response for the next middleware
    let next = Next::new(|req| async move {
        let res = Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::empty())
            .unwrap();
        Ok::<_, axum::http::Error>((req, res))
    });

    // Run the middleware
    let res = security_headers_middleware(Extension(AppState::default()), req, next).await.unwrap();

    // Check that security headers are added
    let headers = res.headers();
    
    // Check X-Frame-Options
    assert_eq!(
        headers.get("X-Frame-Options").unwrap(),
        "DENY"
    );
    
    // Check X-Content-Type-Options
    assert_eq!(
        headers.get("X-Content-Type-Options").unwrap(),
        "nosniff"
    );
    
    // Check Referrer-Policy
    assert_eq!(
        headers.get("Referrer-Policy").unwrap(),
        "strict-origin-when-cross-origin"
    );
    
    // Check that Content-Security-Policy is added
    assert!(headers.contains_key("Content-Security-Policy"));
    
    // Check that Strict-Transport-Security is added
    assert!(headers.contains_key("Strict-Transport-Security"));
}

#[tokio::test]
async fn test_security_headers_middleware_environment_specific_headers() {
    // Temporarily set environment to production for testing
    let original_env = ENV.rust_env.clone();
    std::env::set_var("RUST_ENV", "production");
    
    // Create a mock request
    let req = Request::builder()
        .uri("/test")
        .body(axum::body::empty())
        .unwrap();

    // Create a mock response for the next middleware
    let next = Next::new(|req| async move {
        let res = Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::empty())
            .unwrap();
        Ok::<_, axum::http::Error>((req, res))
    });

    // Run the middleware
    let res = security_headers_middleware(Extension(AppState::default()), req, next).await.unwrap();

    // Check that HSTS header is set for production
    let hsts_header = headers.get("Strict-Transport-Security").unwrap();
    assert!(hsts_header.to_str().unwrap().contains("max-age=31536000"));
    
    // Restore original environment
    std::env::set_var("RUST_ENV", original_env);
}
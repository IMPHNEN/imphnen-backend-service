use axum::body::Body;
use axum::http::Response;
use axum::body::Bytes;
use serde::de::DeserializeOwned;

/// Parse a response body into a serde_json::Value
pub async fn parse_response_value(resp: Response<Body>, limit: usize) -> serde_json::Value {
    let bytes: Bytes = axum::body::to_bytes(resp.into_body(), limit).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

/// Parse a response body into a typed DTO
pub async fn parse_response<T: DeserializeOwned>(resp: Response<Body>, limit: usize) -> T {
    let bytes: Bytes = axum::body::to_bytes(resp.into_body(), limit).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

/// Convenience: parse and return the inner "data" field if the response uses the standard wrapper
pub async fn parse_response_data<T: DeserializeOwned>(resp: Response<Body>, limit: usize) -> T {
    let v = parse_response_value(resp, limit).await;
    // If response uses { "data": ... } wrapper, extract it; otherwise try to parse the whole body as T
    if let Some(inner) = v.get("data") {
        serde_json::from_value(inner.clone()).unwrap()
    } else {
        serde_json::from_value(v).unwrap()
    }
}

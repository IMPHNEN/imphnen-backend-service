use axum::{response::IntoResponse, routing::post, Router};

async fn dummy_login() -> impl IntoResponse {
	"Logged in successfully"
}

pub fn auth_router() -> Router {
	Router::new().route("/login", post(dummy_login))
}

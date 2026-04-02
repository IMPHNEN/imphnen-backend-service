use axum::{
	body::Body,
	http::{Request, Response, StatusCode},
};
use futures::future::BoxFuture;
use imphnen_libs::AppState;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct PaymentLayer {
	app_state: AppState,
}

impl PaymentLayer {
	pub fn new(app_state: AppState) -> Self {
		Self { app_state }
	}
}

impl<S> Layer<S> for PaymentLayer {
	type Service = PaymentMiddleware<S>;
	fn layer(&self, inner: S) -> Self::Service {
		PaymentMiddleware {
			inner,
			app_state: self.app_state.clone(),
		}
	}
}

#[derive(Clone)]
pub struct PaymentMiddleware<S> {
	inner: S,
	app_state: AppState,
}

impl<S> Service<Request<Body>> for PaymentMiddleware<S>
where
	S: Service<Request<Body>, Response = Response<Body>, Error = Response<Body>>
		+ Clone
		+ Send
		+ 'static,
	S::Future: Send + 'static,
{
	type Response = S::Response;
	type Error = S::Error;
	type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
	fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		self.inner.poll_ready(cx)
	}
	fn call(&mut self, req: Request<Body>) -> Self::Future {
		let mut inner = self.inner.clone();
		let _app_state = self.app_state.clone();
		Box::pin(async move {
			let headers = req.headers();

			if let Some(payment_token) = headers.get("X-Payment-Token")
				&& let Ok(token_str) = payment_token.to_str()
				&& !is_valid_payment_token(token_str)
			{
				let error_response = Response::builder()
					.status(StatusCode::PAYMENT_REQUIRED)
					.body(Body::from("Invalid payment token"))
					.expect("valid payment error response");
				return Err(error_response);
			}

			let uri_path = req.uri().path();
			if requires_payment_verification(uri_path)
				&& !headers.contains_key("X-Payment-Token")
			{
				let error_response = Response::builder()
					.status(StatusCode::PAYMENT_REQUIRED)
					.body(Body::from("Payment required for this endpoint"))
					.expect("valid payment required response");
				return Err(error_response);
			}

			inner.call(req).await
		})
	}
}

fn is_valid_payment_token(token: &str) -> bool {
	token.len() >= 16
		&& token
			.chars()
			.all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

fn requires_payment_verification(path: &str) -> bool {
	path.contains("/premium/")
		|| path.contains("/paid/")
		|| path.contains("/subscription/")
}

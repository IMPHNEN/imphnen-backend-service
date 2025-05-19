use axum::{
	body::Body,
	http::{Request, Response, StatusCode},
};
use futures::future::BoxFuture;
use imphnen_iam::{AuthRepository, PermissionsEnum};
use imphnen_libs::AppState;
use imphnen_utils::{common_response, extract_email};
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct PermissionsMiddlewareLayer {
	app_state: AppState,
	permissions: Vec<PermissionsEnum>,
}

impl PermissionsMiddlewareLayer {
	pub fn new(app_state: AppState, permissions: Vec<PermissionsEnum>) -> Self {
		Self {
			app_state,
			permissions,
		}
	}
}

impl<S> Layer<S> for PermissionsMiddlewareLayer {
	type Service = PermissionsMiddleware<S>;
	fn layer(&self, inner: S) -> Self::Service {
		PermissionsMiddleware {
			inner,
			app_state: self.app_state.clone(),
			permissions: self.permissions.clone(),
		}
	}
}

#[derive(Clone)]
pub struct PermissionsMiddleware<S> {
	inner: S,
	app_state: AppState,
	permissions: Vec<PermissionsEnum>,
}

impl<S> Service<Request<Body>> for PermissionsMiddleware<S>
where
	S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
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
		let app_state = self.app_state.clone();
		let permissions = self.permissions.clone();

		Box::pin(async move {
			let headers = req.headers();
			let email = match extract_email(headers) {
				Some(email) => email,
				None => {
					return Ok(common_response(
						StatusCode::UNAUTHORIZED,
						"Invalid or missing authorization token",
					));
				}
			};

			let auth_repo = AuthRepository::new(&app_state);
			let user = match auth_repo.query_get_stored_user(email).await {
				Ok(user) => user,
				Err(_) => {
					return Ok(common_response(
						StatusCode::UNAUTHORIZED,
						"User session expired or not found",
					));
				}
			};

			let user_permissions: Vec<String> =
				user.role.permissions.into_iter().map(|p| p.name).collect();
			let allowed = permissions
				.iter()
				.all(|p| user_permissions.contains(&p.to_string()));

			if !allowed {
				return Ok(common_response(
					StatusCode::FORBIDDEN,
					"You don't have the required permissions",
				));
			}

			inner.call(req).await
		})
	}
}

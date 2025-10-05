use axum::{
	body::Body,
	http::{Request, Response, StatusCode},
};
use futures::future::BoxFuture;
use imphnen_entities::PermissionsEnum;
use imphnen_libs::AppState;
use imphnen_utils::{common_response, extract_email, extract_email_async};
use std::task::{Context, Poll};
use tower::{Layer, Service};

/// Middleware layer for enforcing user permissions on requests.
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
			
			// Try synchronous email extraction first (for internal JWT tokens)
			let email = match extract_email(headers) {
				Some(email) => email,
				None => {
					// If sync extraction fails, try async (for Google tokens)
					match extract_email_async(headers).await {
						Some(email) => email,
						None => {
							return Ok(common_response(
								StatusCode::UNAUTHORIZED,
								"Invalid or missing authorization token",
							));
						}
					}
				}
			};
			
			let user = match app_state.auth_repository.query_get_stored_user(email).await {
				Ok(user) => user,
				Err(_) => {
					return Ok(common_response(
						StatusCode::UNAUTHORIZED,
						"User session expired or not found",
					));
				}
			};
			let user_permissions: Vec<String> =
			user.role.permissions.as_ref().unwrap_or(&vec![]).iter().filter_map(|p| p.as_ref().and_then(|pp| pp.name.clone())).collect();

			// Check if user has Administrator permission - accept either the permission name or the well-known id
			let admin_name = PermissionsEnum::Administrator.to_string();
			let admin_id = PermissionsEnum::Administrator.id();
			let has_administrator_permission = user_permissions.contains(&admin_name) || user_permissions.contains(&admin_id);
			let allowed = has_administrator_permission || permissions
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

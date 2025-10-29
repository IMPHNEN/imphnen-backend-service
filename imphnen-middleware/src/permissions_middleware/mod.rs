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

/// Unified middleware layer for enforcing user permissions on requests.
/// This replaces the legacy permissions_guard function calls with a consistent middleware approach.
#[derive(Clone)]
pub struct PermissionsMiddlewareLayer {
	app_state: AppState,
	permissions: Vec<PermissionsEnum>,
}

impl PermissionsMiddlewareLayer {
	/// Create a new permissions middleware layer with the required permissions
	pub fn new(app_state: AppState, permissions: Vec<PermissionsEnum>) -> Self {
		Self {
			app_state,
			permissions,
		}
	}

	/// Create a middleware layer that requires administrator permissions
	pub fn admin_only(app_state: AppState) -> Self {
		Self::new(app_state, vec![PermissionsEnum::Administrator])
	}

	/// Create a middleware layer that requires specific permission
	pub fn with_permission(app_state: AppState, permission: PermissionsEnum) -> Self {
		Self::new(app_state, vec![permission])
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
	S: Service<Request<Body>, Response = Response<Body>, Error = Response<Body>> + Clone + Send + 'static,
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
			
			// Extract user email from authorization headers
			let email = extract_user_email(headers).await
				.ok_or_else(|| {
					common_response(
						StatusCode::UNAUTHORIZED,
						"Invalid or missing authorization token",
					)
				})?;
			
			// Get user data with permissions from auth repository
			let user = app_state.auth_repository.query_get_stored_user(email).await
				.map_err(|_| {
					common_response(
						StatusCode::UNAUTHORIZED,
						"User session expired or not found",
					)
				})?;
			
			// Extract user permissions from role
			let user_permissions = extract_user_permissions(&user);
			
			// Check if user has required permissions
			if !has_required_permissions(&user_permissions, &permissions) {
				return Err(common_response(
					StatusCode::FORBIDDEN,
					"You don't have the required permissions",
				));
			}
			
			inner.call(req).await
		})
	}
}

/// Extract user email from headers (sync and async fallback)
async fn extract_user_email(headers: &axum::http::HeaderMap) -> Option<String> {
	// Try synchronous extraction first
	match extract_email(headers) {
		Some(email) => Some(email),
		None => {
			// Fallback to async extraction for Google tokens
			extract_email_async(headers).await
		}
	}
}

/// Extract user permissions from user data
fn extract_user_permissions(user: &imphnen_entities::UsersDetailQueryDto) -> Vec<String> {
	user.role
		.permissions
		.as_ref()
		.unwrap_or(&vec![])
		.iter()
		.filter_map(|p| p.as_ref())
		.flat_map(|pp| {
			let mut permissions = Vec::new();
			// Add permission name if available
			if let Some(name) = pp.name.clone() {
				permissions.push(name);
			}
			// Add permission ID if available
			if let Some(id) = pp.id.as_ref().map(|id| id.id.to_raw()) {
				permissions.push(id);
			}
			permissions
		})
		.collect()
}

/// Check if user has required permissions
fn has_required_permissions(user_permissions: &[String], required_permissions: &[PermissionsEnum]) -> bool {
    // Administrator has access to everything
    let admin_name = PermissionsEnum::Administrator.to_string();
    let admin_id = PermissionsEnum::Administrator.id();
    
    if user_permissions.contains(&admin_name) || user_permissions.contains(&admin_id) {
        return true;
    }
    
    // Check if user has all required permissions
    required_permissions.iter().all(|required| {
        let required_name = required.to_string();
        let required_id = required.id();
        
        user_permissions.contains(&required_name) || user_permissions.contains(&required_id)
    })
}

/// Simple permission check function for use in controllers (legacy compatibility)
/// This provides a bridge between old permissions_guard calls and new middleware approach
pub async fn check_permissions(
	headers: &axum::http::HeaderMap,
	app_state: &AppState,
	required_permissions: Vec<PermissionsEnum>,
) -> Result<(), Response<Body>> {
	let email = extract_user_email(headers).await
		.ok_or_else(|| {
			common_response(
				StatusCode::UNAUTHORIZED,
				"Invalid or missing authorization token",
			)
		})?;
	
	let user = app_state.auth_repository.query_get_stored_user(email).await
		.map_err(|_| {
			common_response(
				StatusCode::UNAUTHORIZED,
				"User session expired or not found",
			)
		})?;
	
	let user_permissions = extract_user_permissions(&user);
	
	if !has_required_permissions(&user_permissions, &required_permissions) {
		return Err(common_response(
			StatusCode::FORBIDDEN,
			"You don't have the required permissions",
		));
	}
	
	Ok(())
}

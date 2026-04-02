use axum::response::IntoResponse;
use axum::{
	body::Body,
	http::{Request, Response, StatusCode},
};
use futures::future::BoxFuture;
use imphnen_entities::PermissionsEnum;
use imphnen_libs::{AppState, services::ExtendedUserInfo};
use imphnen_utils::response_format::ApiMessage;
use imphnen_utils::{extract_email, extract_email_async};
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

	pub fn admin_only(app_state: AppState) -> Self {
		Self::new(app_state, vec![PermissionsEnum::Administrator])
	}

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
		let app_state = self.app_state.clone();
		let permissions = self.permissions.clone();
		Box::pin(async move {
			let headers = req.headers();

			let email = extract_user_email(headers).await.ok_or_else(|| {
				ApiMessage::new(
					StatusCode::UNAUTHORIZED,
					"Invalid or missing authorization token",
				)
				.into_response()
			})?;

			let user = app_state
				.user_lookup_service
				.get_user_by_email(&email, &app_state)
				.await
				.map_err(|_| {
					ApiMessage::new(
						StatusCode::UNAUTHORIZED,
						"User session expired or not found",
					)
					.into_response()
				})?;

			let user_permissions = extract_user_permissions(&user);

			if !has_required_permissions(&user_permissions, &permissions) {
				return Err(
					ApiMessage::new(
						StatusCode::FORBIDDEN,
						"You don't have the required permissions",
					)
					.into_response(),
				);
			}

			inner.call(req).await
		})
	}
}

async fn extract_user_email(headers: &axum::http::HeaderMap) -> Option<String> {
	match extract_email(headers) {
		Some(email) => Some(email),
		None => extract_email_async(headers).await,
	}
}

fn extract_user_permissions(user: &ExtendedUserInfo) -> Vec<String> {
	user
		.basic_info
		.role
		.permissions
		.as_ref()
		.unwrap_or(&vec![])
		.iter()
		.filter_map(|p| p.as_ref())
		.flat_map(|pp| {
			let mut permissions = Vec::new();
			if let Some(name) = pp.name.clone() {
				permissions.push(name);
			}
			if let Some(id) = pp.id.as_ref().map(|id| id.to_string()) {
				permissions.push(id);
			}
			permissions
		})
		.collect()
}

fn has_required_permissions(
	user_permissions: &[String],
	required_permissions: &[PermissionsEnum],
) -> bool {
	let admin_name = PermissionsEnum::Administrator.to_string();
	let admin_id = PermissionsEnum::Administrator.id();

	if user_permissions.contains(&admin_name) || user_permissions.contains(&admin_id) {
		return true;
	}

	required_permissions.iter().all(|required| {
		let required_name = required.to_string();
		let required_id = required.id();
		user_permissions.contains(&required_name)
			|| user_permissions.contains(&required_id)
	})
}

pub async fn check_permissions(
	headers: &axum::http::HeaderMap,
	app_state: &AppState,
	required_permissions: Vec<PermissionsEnum>,
) -> Result<(), Response<Body>> {
	let email = extract_user_email(headers).await.ok_or_else(|| {
		ApiMessage::new(
			StatusCode::UNAUTHORIZED,
			"Invalid or missing authorization token",
		)
		.into_response()
	})?;

	let user = app_state
		.user_lookup_service
		.get_user_by_email(&email, app_state)
		.await
		.map_err(|_| {
			ApiMessage::new(
				StatusCode::UNAUTHORIZED,
				"User session expired or not found",
			)
			.into_response()
		})?;

	let user_permissions = extract_user_permissions(&user);

	if !has_required_permissions(&user_permissions, &required_permissions) {
		return Err(
			ApiMessage::new(
				StatusCode::FORBIDDEN,
				"You don't have the required permissions",
			)
			.into_response(),
		);
	}

	Ok(())
}

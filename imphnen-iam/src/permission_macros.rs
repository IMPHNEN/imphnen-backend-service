use crate::AppState;
use crate::permissions_guard;
use axum::{extract::Extension, http::HeaderMap};
use imphnen_entities::PermissionsEnum;
use imphnen_libs::jsonwebtoken::Claims;
use imphnen_utils::AppError;

pub type PermissionGuardResult<T> = Result<(T, AppState), AppError>;

pub async fn check_permissions(
	headers: HeaderMap,
	state: Extension<AppState>,
	required_permissions: Vec<PermissionsEnum>,
) -> PermissionGuardResult<Claims> {
	permissions_guard(headers, state, required_permissions).await
}

pub async fn check_authenticated(
	headers: HeaderMap,
	state: Extension<AppState>,
) -> PermissionGuardResult<Claims> {
	check_permissions(headers, state, vec![]).await
}

#[macro_export]
macro_rules! require_permissions {
    ($headers:expr, $state:expr, [$($perm:expr),*], $body:block) => {
        {
            let state_clone = $state.clone();
            $crate::permissions_guard(
                $headers,
                axum::extract::Extension(state_clone),
                vec![$($perm),*],
            )
            .await?;
            $body
        }
    };
}

#[macro_export]
macro_rules! require_auth {
	($headers:expr, $state:expr, $body:block) => {{
		let state_clone = $state.clone();
		$crate::permissions_guard(
			$headers,
			axum::extract::Extension(state_clone),
			vec![],
		)
		.await?;
		$body
	}};
}

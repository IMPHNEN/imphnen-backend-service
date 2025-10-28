//! Permission guard utilities and macros to reduce boilerplate
//!
//! This module provides utilities to simplify permission checking in handlers

use axum::{
    extract::Extension,
    http::HeaderMap,
    response::Response,
};
use imphnen_entities::PermissionsEnum;
use crate::AppState;
use crate::permissions_guard;
use imphnen_libs::jsonwebtoken::Claims;

/// Result type for permission-guarded handlers
pub type PermissionGuardResult<T> = Result<(T, AppState), Response>;

/// Helper function to extract user and check permissions
///
/// This is a cleaner wrapper around the existing permissions_guard
pub async fn check_permissions(
    headers: HeaderMap,
    state: Extension<AppState>,
    required_permissions: Vec<PermissionsEnum>,
) -> PermissionGuardResult<Claims> {
    match permissions_guard(headers, state, required_permissions).await {
        Ok((user, state)) => Ok((user, state)),
        Err(response) => Err(response),
    }
}

/// Helper function for endpoints that don't require specific permissions
/// but still need authentication
pub async fn check_authenticated(
    headers: HeaderMap,
    state: Extension<AppState>,
) -> PermissionGuardResult<Claims> {
    check_permissions(headers, state, vec![]).await
}

/// Macro to reduce boilerplate in permission-guarded handlers
///
/// # Example
/// ```rust
/// use imphnen_iam::require_permissions;
/// use imphnen_entities::PermissionsEnum;
///
/// pub async fn get_user_list(
///     headers: HeaderMap,
///     Extension(state): Extension<AppState>,
///     Query(meta): Query<MetaRequestDto>,
/// ) -> Response {
///     require_permissions!(headers, state, [PermissionsEnum::ReadListUsers], {
///         UsersService::get_user_list(&state, meta).await
///     })
/// }
/// ```
#[macro_export]
macro_rules! require_permissions {
    ($headers:expr, $state:expr, [$($perm:expr),*], $body:block) => {
        {
            let state_clone = $state.clone();
            match $crate::permissions_guard(
                $headers,
                axum::extract::Extension(state_clone),
                vec![$($perm),*],
            )
            .await
            {
                Ok((_user, _state_inner)) => {
                    let state = &$state;
                    $body
                }
                Err(response) => response,
            }
        }
    };
}

/// Macro for authenticated-only handlers (no specific permissions)
#[macro_export]
macro_rules! require_auth {
    ($headers:expr, $state:expr, $body:block) => {
        {
            let state_clone = $state.clone();
            match $crate::permissions_guard($headers, axum::extract::Extension(state_clone), vec![]).await {
                Ok((_user, _state_inner)) => {
                    let state = &$state;
                    $body
                }
                Err(response) => response,
            }
        }
    };
}

/// Macro for handlers that need access to the authenticated user
#[macro_export]
macro_rules! with_user {
    ($headers:expr, $state:expr, [$($perm:expr),*], |$user:ident, $state_var:ident| $body:block) => {
        {
            let state_clone = $state.clone();
            match $crate::permissions_guard(
                $headers,
                axum::extract::Extension(state_clone),
                vec![$($perm),*],
            )
            .await
            {
                Ok(($user, $state_var)) => $body,
                Err(response) => response,
            }
        }
    };
}

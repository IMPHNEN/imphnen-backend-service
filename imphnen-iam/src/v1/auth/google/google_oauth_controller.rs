use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Json, Router,
};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::sync::Arc; // Import Arc

use crate::v1::auth::google::google_oauth_service::{AuthRequest, GoogleOauthService, GoogleOauthServiceImpl};
use crate::v1::auth::auth_service::AuthServiceTrait;
use crate::v1::users::users_service::UsersServiceTrait;
use imphnen_entities::error_dto::error::Error;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GoogleAuthUrlResponse {
    pub authorize_url: String,
}

pub struct GoogleOauthController<T> { // Generic over T
    google_oauth_service: T,
}

// Concrete implementation for new()
impl GoogleOauthController<GoogleOauthServiceImpl<crate::v1::auth::auth_service::AuthService, crate::v1::users::users_service::UsersService>> {
    pub fn new() -> Self {
        Self {
            google_oauth_service: GoogleOauthServiceImpl::<crate::v1::auth::auth_service::AuthService, crate::v1::users::users_service::UsersService>::new(), // Explicitly specify type parameters
        }
    }
}

// Generic implementation for with_service and get_routes
impl<T> GoogleOauthController<T>
where
    T: GoogleOauthService<crate::v1::auth::auth_service::AuthService, crate::v1::users::users_service::UsersService> + Clone + Send + Sync + 'static, // Explicitly constrain T
{
    pub fn with_service(google_oauth_service: T) -> Self {
        Self {
            google_oauth_service,
        }
    }

    pub fn get_routes(&self) -> Router { // Take self by reference
        Router::new()
            .route(
                "/google/login",
                get(
                    move |State(controller): State<Arc<Self>>| async move {
                        controller.google_oauth_login().await
                    },
                ),
            )
            .route(
                "/google/callback",
                get(
                    move |State(controller): State<Arc<Self>>, Query(auth_request): Query<AuthRequest>| async move {
                        let controller = Arc::clone(&controller);
                        controller.google_oauth_callback(auth_request).await
                    },
                ),
            )
            .with_state(Arc::new(self.clone())) // Pass an Arc clone of self to with_state
    }

    pub async fn google_oauth_login(&self) -> Result<Redirect, Error> {
        let (authorize_url, _csrf_state) = self.google_oauth_service.generate_auth_url();
        Ok(Redirect::to(authorize_url.as_str()))
    }

    pub async fn google_oauth_callback(&self, auth_request: AuthRequest) -> Result<impl IntoResponse + use<T>, Error> {
        let token = self.google_oauth_service.google_oauth_callback(auth_request).await?;
        Ok((StatusCode::OK, Json(serde_json::json!({"token": token}))))
    }
}

// Clone implementation
impl<T> Clone for GoogleOauthController<T>
where
    T: GoogleOauthService<crate::v1::auth::auth_service::AuthService, crate::v1::users::users_service::UsersService> + Clone, // Explicitly constrain T
{
    fn clone(&self) -> Self {
        Self::with_service(self.google_oauth_service.clone())
    }
}
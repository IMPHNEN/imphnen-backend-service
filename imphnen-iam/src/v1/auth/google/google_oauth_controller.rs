use axum::{
    extract::{Query, State},
    response::Redirect,
    routing::get,
    Json, Router, Extension,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::sync::Arc;
use imphnen_libs::environment::ENV; // Import ENV

use crate::v1::auth::google::google_oauth_service::{AuthRequest, GoogleOauthService, GoogleOauthServiceImpl};
use imphnen_entities::error_dto::error::Error;
use crate::v1::auth::AuthLoginResponsetDto;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GoogleAuthUrlResponse {
    pub authorize_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleLoginRequest {
    pub redirect_uri: Option<String>,
}

pub struct GoogleOauthController<T> {
    google_oauth_service: T,
}

impl GoogleOauthController<GoogleOauthServiceImpl<crate::v1::auth::auth_service::AuthService, crate::v1::users::users_service::UsersService>> {
    pub fn new() -> Self {
        let google_oauth_service = GoogleOauthServiceImpl::<crate::v1::auth::auth_service::AuthService, crate::v1::users::users_service::UsersService>::with_services(
            crate::v1::auth::auth_service::AuthService {},
            crate::v1::users::users_service::UsersService {},
            &ENV, // Pass a reference to the global ENV static
        );
        Self::with_service(google_oauth_service)
    }
}

impl<T> GoogleOauthController<T>
where
    T: GoogleOauthService<crate::v1::auth::auth_service::AuthService, crate::v1::users::users_service::UsersService> + Clone + Send + Sync + 'static,
{
    pub fn with_service(google_oauth_service: T) -> Self {
        Self {
            google_oauth_service,
        }
    }

    pub fn get_routes(&self) -> Router {
        Router::new()
            .route(
                "/login",
                get(
                    move |State(controller): State<Arc<Self>>, Query(params): Query<GoogleLoginRequest>| async move {
                        controller.google_oauth_login(params).await
                    },
                ),
            )
            .route(
                "/callback",
                get(
                    move |State(controller): State<Arc<Self>>, Extension(app_state): Extension<AppState>, Query(auth_request): Query<AuthRequest>| async move {
                        let controller = Arc::clone(&controller);
                        controller.google_oauth_callback(auth_request, &app_state).await
                    },
                ),
            )
            .with_state(Arc::new(self.clone()))
    }

    pub async fn google_oauth_login(&self, params: GoogleLoginRequest) -> Result<Redirect, Error> {
        let (authorize_url, _csrf_state) = self.google_oauth_service.generate_auth_url(params.redirect_uri);
        Ok(Redirect::to(authorize_url.as_str()))
    }

    pub async fn google_oauth_callback(&self, auth_request: AuthRequest, app_state: &AppState) -> Result<Json<AuthLoginResponsetDto>, Error> {
        let (user, token) = self.google_oauth_service.google_oauth_callback(auth_request, app_state).await?;
        let auth_response = AuthLoginResponsetDto {
            user,
            token,
        };
        Ok(Json(auth_response))
    }
}

impl<T> Clone for GoogleOauthController<T>
where
    T: GoogleOauthService<crate::v1::auth::auth_service::AuthService, crate::v1::users::users_service::UsersService> + Clone,
{
    fn clone(&self) -> Self {
        Self::with_service(self.google_oauth_service.clone())
    }
}

impl Default for GoogleOauthController<GoogleOauthServiceImpl<crate::v1::auth::auth_service::AuthService, crate::v1::users::users_service::UsersService>> {
    fn default() -> Self {
        Self::new()
    }
}
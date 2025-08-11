use anyhow::Result;
use async_trait::async_trait;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use oauth2::url::Url;
use tracing::{info, error};

use imphnen_entities::error_dto::error::Error;
use imphnen_libs::{jsonwebtoken::{encode_access_token, encode_refresh_token}, enviroment::Env};
use imphnen_utils::{generate_csrf_token, validate_csrf_token};
use crate::v1::auth::TokenDto;
use crate::v1::auth::auth_service::AuthServiceTrait;
use crate::v1::users::users_dto::{UsersCreateRequestDto, UsersDetailItemDto};
use crate::v1::users::users_service::UsersServiceTrait;

use super::google_oauth_dto::GoogleUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}

impl AuthRequest {
    /// Validate the OAuth callback request
    pub fn validate(&self) -> Result<(), Error> {
        // Validate code parameter
        if self.code.is_empty() || self.code.len() > 2048 {
            return Err(Error::Validation("Invalid authorization code".to_string()));
        }
        
        // Validate state parameter  
        if self.state.is_empty() || self.state.len() > 512 {
            return Err(Error::Validation("Invalid state parameter".to_string()));
        }
        
        // Basic format validation for authorization code
        if !self.code.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~') {
            return Err(Error::Validation("Authorization code contains invalid characters".to_string()));
        }
        
        Ok(())
    }
    
    /// Validate CSRF state token with signature verification
    pub fn validate_csrf_state(&self, secret: &str) -> Result<(), Error> {
        // Maximum age of 10 minutes for OAuth flow
        const MAX_AGE_SECONDS: u64 = 600;
        
        validate_csrf_token(&self.state, secret, MAX_AGE_SECONDS)
            .map_err(|e| {
                error!("CSRF validation failed: {:?}", e);
                Error::Auth("Invalid or expired CSRF state token".to_string())
            })
    }
}

/// Helper function to get default role ID for new OAuth users
async fn get_default_role_id(_env: &Env) -> Result<String, Error> {
    // Use the User role ID from the seed data directly
    let default_role_id = "5713cb37-dc02-4e87-8048-d7a41d352059".to_string();
    info!("Using default User role ID from seed: {}", default_role_id);
    Ok(default_role_id)
}

#[async_trait]
pub trait GoogleOauthService<A: AuthServiceTrait + Send + Sync + 'static, U: UsersServiceTrait + Send + Sync + 'static>: Send + Sync + 'static {
    // Removed new() from trait
    fn with_services(auth_service: A, users_service: U, env: &'static Env) -> Self;
    fn google_oauth_client(&self) -> BasicClient;
    fn generate_auth_url(&self) -> (Url, CsrfToken);
    async fn google_oauth_callback(&self, auth_request: AuthRequest) -> Result<(UsersDetailItemDto, TokenDto), Error>; // Changed return type
}

#[derive(Clone)]
pub struct GoogleOauthServiceImpl<A: AuthServiceTrait, U: UsersServiceTrait> {
    users_service: U,
    env: &'static Env,
    #[allow(dead_code)]
    auth_service: A,
}

impl GoogleOauthServiceImpl<crate::v1::auth::auth_service::AuthService, crate::v1::users::users_service::UsersService> {
    // Removed the `new()` method as it will be replaced by `with_services`
}

#[async_trait]
impl<A, U> GoogleOauthService<A, U> for GoogleOauthServiceImpl<A, U>
where
    A: AuthServiceTrait + Send + Sync + 'static,
    U: UsersServiceTrait + Send + Sync + 'static,
{
    fn with_services(auth_service: A, users_service: U, env: &'static Env) -> Self {
        Self {
            auth_service,
            users_service,
            env,
        }
    }

    fn google_oauth_client(&self) -> BasicClient {
        let google_client_id = ClientId::new(self.env.google_client_id.clone());
        let google_client_secret = ClientSecret::new(self.env.google_client_secret.clone());
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
            .expect("Invalid token endpoint URL");

        BasicClient::new(
            google_client_id,
            Some(google_client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(
            RedirectUrl::new(self.env.google_redirect_url.clone())
                .expect("Invalid redirect URL"),
        )
    }

    fn generate_auth_url(&self) -> (Url, CsrfToken) {
        let client = self.google_oauth_client();
        let (pkce_code_challenge, _pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate a signed CSRF token for stateless validation
        let csrf_token_str = generate_csrf_token(&self.env.access_token_secret)
            .unwrap_or_else(|_| uuid::Uuid::new_v4().to_string()); // Fallback to UUID if signing fails
        
        let csrf_token = CsrfToken::new(csrf_token_str);

        client
            .authorize_url(|| csrf_token.clone())
            .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()))
            .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()))
            .set_pkce_challenge(pkce_code_challenge)
            .url()
    }

    async fn google_oauth_callback(&self, auth_request: AuthRequest) -> Result<(UsersDetailItemDto, TokenDto), Error> {
        // Validate input parameters first
        auth_request.validate()?;
        
        // CRITICAL: Validate CSRF state token
        auth_request.validate_csrf_state(&self.env.access_token_secret)?;
        
        info!("Starting Google OAuth callback process");
        
        let client = self.google_oauth_client();

        let token_response = client
            .exchange_code(oauth2::AuthorizationCode::new(auth_request.code))
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .map_err(|e| {
                error!("Failed to exchange OAuth code: {}", e);
                Error::Auth("Failed to exchange authorization code".to_string())
            })?;

        // Note: This part has an issue with parsing Google's token response
        // Google returns the actual access token, not a JSON with our custom format
        let access_token = token_response.access_token().secret();

        let client = reqwest::Client::new();
        let user_info_url = "https://www.googleapis.com/oauth2/v2/userinfo";
        let google_user: GoogleUser = client
            .get(user_info_url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to fetch user info from Google: {}", e);
                Error::Auth("Failed to fetch user information".to_string())
            })?
            .json()
            .await
            .map_err(|e| {
                error!("Failed to parse user info from Google: {}", e);
                Error::Auth("Failed to parse user information".to_string())
            })?;

        info!("Successfully retrieved user info for email: {}", google_user.email);

        let user = self.users_service.get_user_by_email(&google_user.email).await?;

        let user = match user {
            Some(user) => {
                info!("Existing user found for email: {}", google_user.email);
                user
            },
            None => {
                info!("Creating new user for email: {}", google_user.email);
                
                // Get default role ID using robust lookup
                let default_role_id = get_default_role_id(self.env).await
                    .unwrap_or_else(|e| {
                        error!("Failed to get default role ID, using fallback: {:?}", e);
                        "5713cb37-dc02-4e87-8048-d7a41d352059".to_string() // Hardcoded User role ID as final fallback
                    });
                
                let new_user = UsersCreateRequestDto {
                    email: google_user.email.clone(),
                    password: format!("GOOGLE_OAUTH_{}", uuid::Uuid::new_v4()), // Random placeholder
                    fullname: google_user.name.clone(),
                    phone_number: "".to_string(), // Will be updated by user later
                    is_active: true,
                    role_id: default_role_id,
                };
                
                self.users_service.create_user_by_dto(new_user).await?
            }
        };

        let access_token = encode_access_token(user.email.clone())
            .map_err(|e| {
                error!("Failed to generate access token for {}: {:?}", user.email, e);
                Error::Auth("Failed to generate access token".to_string())
            })?;
            
        let refresh_token = encode_refresh_token(user.email.clone())
            .map_err(|e| {
                error!("Failed to generate refresh token for {}: {:?}", user.email, e);
                Error::Auth("Failed to generate refresh token".to_string())
            })?;

        let token_dto = TokenDto {
            access_token,
            refresh_token,
        };

        info!("Successfully completed Google OAuth for user: {}", user.email);
        Ok((user, token_dto))
    }
}
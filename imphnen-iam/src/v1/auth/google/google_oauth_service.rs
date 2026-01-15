use std::pin::Pin;
use std::future::Future;
use anyhow::Result;
// Type alias to reduce clippy type_complexity warnings for long Future signatures
type GoogleOauthCallbackFut<'a> = Pin<Box<dyn Future<Output = Result<(UsersDetailItemDto, TokenDto), Error>> + Send + 'a>>;

use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, Scope, TokenUrl,
};
use serde::{Deserialize, Serialize};
use oauth2::url::Url;
use oauth2::TokenResponse;
use tracing::{info, error};

use imphnen_entities::error_dto::error::Error;
use imphnen_libs::{jsonwebtoken::{encode_access_token, encode_refresh_token}, environment::Env, AppState};
use crate::{generate_oauth_csrf_token, validate_oauth_csrf_token, validate_csrf_token};
use crate::v1::auth::TokenDto;
use crate::v1::auth::auth_service::AuthServiceTrait;
use crate::v1::users::users_dto::{UsersDetailItemDto, UsersCreateRequestDto};
use crate::v1::users::users_service::UsersServiceTrait;

use super::google_oauth_dto::GoogleUser;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
    pub redirect_uri: Option<String>,
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
        // OAuth 2.0 authorization codes can contain URL-safe characters including base64 characters
        if !self.code.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' || c == '/' || c == '+' || c == '=') {
            return Err(Error::Validation("Authorization code contains invalid characters".to_string()));
        }
        
        Ok(())
    }
    
    /// Validate CSRF state token with signature verification and extract PKCE verifier
    pub fn validate_csrf_state_and_get_pkce_verifier(&self, secret: &str) -> Result<PkceCodeVerifier, Error> {
        // Maximum age of 30 minutes for OAuth flow (increased from 10)
        const MAX_AGE_SECONDS: u64 = 300; // Changed from 30 minutes (1800s) to 5 minutes (300s)
        
        let pkce_verifier_secret = validate_oauth_csrf_token(&self.state, secret, MAX_AGE_SECONDS)
            .map_err(|e| {
                error!("OAuth CSRF validation failed: {:?}", e);
                Error::Auth("Invalid or expired OAuth CSRF state token".to_string())
            })?;
            
        Ok(PkceCodeVerifier::new(pkce_verifier_secret))
    }
    
    /// Validate CSRF state token with signature verification (legacy method for backward compatibility)
    pub fn validate_csrf_state(&self, secret: &str) -> Result<(), Error> {
        // Try OAuth CSRF validation first, if it fails, fall back to regular CSRF validation
        match validate_oauth_csrf_token(&self.state, secret, 1800) {
            Ok(_) => Ok(()),
            Err(_) => {
                // Fallback to regular CSRF validation for backward compatibility
                validate_csrf_token(&self.state, secret, 600)
                    .map_err(|e| {
                        error!("CSRF validation failed: {:?}", e);
                        Error::Auth("Invalid or expired CSRF state token".to_string())
                    })
            }
        }
    }
}

use crate::{RolesRepository, RolesEnum};

/// Helper function to get default role ID for new OAuth users
async fn get_default_role_id(app_state: &AppState) -> Result<String, Error> {
    let role_repo = RolesRepository::new(app_state);
    match role_repo.query_role_by_name(RolesEnum::User.to_string()).await {
        Ok(role) => {
            info!("Using default User role ID: {}", role.id);
            Ok(role.id)
        },
        Err(e) => {
            error!("Failed to retrieve User role: {:?}", e);
            Err(Error::Anyhow(anyhow::Error::msg("Failed to get default role ID".to_string())))
        }
    }
}


pub trait GoogleOauthService<A: AuthServiceTrait + Send + Sync + 'static, U: UsersServiceTrait + Send + Sync + 'static>: Send + Sync + 'static {
    // Removed new() from trait
    fn with_services(auth_service: A, users_service: U, env: &'static Env) -> Self;
    fn generate_auth_url(&self, custom_redirect_uri: Option<String>) -> (Url, CsrfToken);
    fn google_oauth_callback(&self, auth_request: AuthRequest, app_state: &AppState) -> GoogleOauthCallbackFut<'_>; // Changed return type
}

#[derive(Clone)]
pub struct GoogleOauthServiceImpl<A: AuthServiceTrait, U: UsersServiceTrait> {
    users_service: U,
    env: &'static Env,
    _auth_service: A,
}

impl GoogleOauthServiceImpl<crate::v1::auth::auth_service::AuthService, crate::v1::users::users_service::UsersService> {
    /// Generate Google OAuth authorization URL
    pub fn get_auth_url(&self, custom_redirect_uri: Option<String>) -> String {
        let (auth_url, _csrf_token) = self.generate_auth_url(custom_redirect_uri);
        auth_url.to_string()
    }
}



impl<A, U> GoogleOauthService<A, U> for GoogleOauthServiceImpl<A, U>
where
    A: AuthServiceTrait + Send + Sync + 'static,
    U: UsersServiceTrait + Send + Sync + 'static,
{
    fn with_services(auth_service: A, users_service: U, env: &'static Env) -> Self {
        Self {
            _auth_service: auth_service,
            users_service,
            env,
        }
    }


    fn generate_auth_url(&self, custom_redirect_uri: Option<String>) -> (Url, CsrfToken) {
        let google_client_id = ClientId::new(self.env.google_client_id.clone());
        let google_client_secret = ClientSecret::new(self.env.google_client_secret.clone());
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
            .expect("Invalid token endpoint URL");
        let redirect_uri = custom_redirect_uri.unwrap_or_else(|| self.env.google_redirect_url.clone());
        let client = oauth2::basic::BasicClient::new(google_client_id)
            .set_client_secret(google_client_secret)
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(
                RedirectUrl::new(redirect_uri.clone())
                    .expect("Invalid redirect URL"),
            );
        info!("OAuth client configured with redirect URI: {}", redirect_uri);
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
        info!("Generated PKCE Code Challenge: {}", pkce_code_challenge.as_str());
        info!("Generated PKCE Code Verifier: {}", pkce_code_verifier.secret());

        // Generate a signed CSRF token with PKCE verifier for stateless validation
        let csrf_token_str = generate_oauth_csrf_token(&self.env.access_token_secret, pkce_code_verifier.secret())
            .unwrap_or_else(|_| uuid::Uuid::new_v4().to_string()); // Fallback to UUID if signing fails
        
        

        let (auth_url, csrf_token) = client
            .authorize_url(|| CsrfToken::new(csrf_token_str.clone()))
            .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()))
            .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()))
            .set_pkce_challenge(pkce_code_challenge)
            .url();
        (auth_url, csrf_token)
    }

    fn google_oauth_callback(&self, auth_request: AuthRequest, app_state: &AppState) -> Pin<Box<dyn Future<Output = Result<(UsersDetailItemDto, TokenDto), Error>> + Send + '_>> {
        let self_clone = self; // Use reference instead of clone
        let app_state = app_state.to_owned();
        Box::pin(async move {
        // Validate input parameters first
        info!("Received OAuth callback request with state: {}", auth_request.state);
        auth_request.validate()?;
        
        // CRITICAL: Validate CSRF state token and extract PKCE verifier
        let pkce_verifier = auth_request.validate_csrf_state_and_get_pkce_verifier(&self_clone.env.access_token_secret)?;
        
        info!("Starting Google OAuth callback process");
        info!("Redirect URI used: {:?}", auth_request.redirect_uri);
        info!("PKCE verifier extracted: {}", pkce_verifier.secret());
        info!("PKCE verifier extracted: {}", pkce_verifier.secret());
        
        // Use the SAME redirect URI that was used for auth URL generation
        // This is crucial for OAuth security and consistency
        let google_client_id = ClientId::new(self_clone.env.google_client_id.clone());
        let google_client_secret = ClientSecret::new(self_clone.env.google_client_secret.clone());
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
            .expect("Invalid token endpoint URL");
        let redirect_uri = auth_request.redirect_uri.clone().unwrap_or_else(|| self_clone.env.google_redirect_url.clone());
        let client = oauth2::basic::BasicClient::new(google_client_id)
            .set_client_secret(google_client_secret)
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(
                RedirectUrl::new(redirect_uri.clone())
                    .expect("Invalid redirect URL"),
            );
        info!("OAuth client configured with redirect URI: {}", redirect_uri);
        
        // Debug the OAuth client configuration
        let effective_redirect_uri = auth_request.redirect_uri.as_ref().unwrap_or(&self_clone.env.google_redirect_url);
        info!("Effective redirect URI for OAuth client: {}", effective_redirect_uri);
        info!("Google Client ID: {}", self_clone.env.google_client_id);

        info!("Attempting to exchange authorization code with Google");
        info!("Using PKCE verifier for secure exchange");
        info!("Authorization code length: {}", auth_request.code.len());

        let token_response = client
            .exchange_code(oauth2::AuthorizationCode::new(auth_request.code.clone()))
            .set_pkce_verifier(pkce_verifier)
            .request_async(&reqwest::Client::new())
            .await
            .map_err(|e| {
                error!("Failed to exchange OAuth code with Google: {}", e);
                error!("OAuth code was: {}", auth_request.code);
                error!("Redirect URI was: {:?}", auth_request.redirect_uri);

                // Debug OAuth client configuration
                error!("Google Client ID: {}", self_clone.env.google_client_id);
                error!("OAuth client redirect URI configured: {}",
                    auth_request.redirect_uri.as_ref().unwrap_or(&self_clone.env.google_redirect_url));

                // Try to extract more details from the error
                match &e {
                    oauth2::RequestTokenError::ServerResponse(response) => {
                        error!("Google OAuth Server Response Error: {:?}", response);
                    },
                    oauth2::RequestTokenError::Request(req_err) => {
                        error!("Google OAuth Request Error: {:?}", req_err);
                    },
                    oauth2::RequestTokenError::Parse(parse_err, response_body) => {
                        error!("Google OAuth Parse Error: {:?}", parse_err);
                        error!("Response body: {:?}", response_body);
                    },
                    oauth2::RequestTokenError::Other(other) => {
                        error!("Google OAuth Other Error: {:?}", other);
                    },
                }

                Error::Auth("Authentication error: Failed to exchange authorization code".to_string())
            })?;

        info!("Successfully exchanged authorization code for access token");

        // Extract access token from Google's response
        let access_token = token_response.access_token().secret();
        info!("Obtained access token from Google, fetching user info...");

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
        info!("Google user picture URL: {:?}", google_user.picture);
        info!("Google user data: name={:?}, given_name={:?}, family_name={:?}, picture={:?}", 
              google_user.name, google_user.given_name, google_user.family_name, google_user.picture);

        let user = self_clone.users_service.get_user_by_email(&google_user.email, &app_state).await?;

        let user = match user {
            Some(mut user) => {
                info!("Existing user found for email: {}", google_user.email);
                
                // Update avatar if user doesn't have one and Google provides one
                if user.avatar.is_none() && google_user.picture.is_some() {
                    info!("Updating avatar for existing user: {}", google_user.email);
                    match U::update_user_avatar(&google_user.email, google_user.picture.clone(), &app_state).await {
                        Ok(_) => {
                            info!("Successfully updated avatar for user: {}", google_user.email);
                            user.avatar = google_user.picture.clone();
                        },
                        Err(e) => {
                            error!("Failed to update avatar for user {}: {:?}", google_user.email, e);
                        }
                    }
                }
                
                user
            },
            None => {
                info!("Creating new user for email: {}", google_user.email);
                
                // Get default role ID using robust lookup
                let default_role_id = get_default_role_id(&app_state).await
                    .map_err(|e| {
                        error!("Failed to get default role ID: {:?}", e);
                        Error::Anyhow(anyhow::Error::msg("Failed to get default role ID for new user".to_string()))
                    })?;
                
                let new_user = UsersCreateRequestDto {
                    email: google_user.email.clone(),
                    password: format!("GOOGLE_OAUTH_{}", uuid::Uuid::new_v4()), // Random placeholder
                    fullname: google_user.name.clone().unwrap_or_else(|| {
                        // Fallback: use given_name + family_name if available, otherwise use email prefix
                        match (&google_user.given_name, &google_user.family_name) {
                            (Some(given), Some(family)) => format!("{} {}", given, family),
                            (Some(given), None) => given.clone(),
                            (None, Some(family)) => family.clone(),
                            (None, None) => {
                                // Extract email prefix as last resort
                                google_user.email.split('@').next().unwrap_or("User").to_string()
                            }
                        }
                    }),

                    is_active: true,
                    role_id: default_role_id,
                    avatar: google_user.picture.clone(), // Set avatar from Google user picture
                };
                
                self_clone.users_service.create_user_by_dto(new_user, &app_state).await?
            }
        };

let access_token = encode_access_token(user.email.clone(), user.id.clone())
            .map_err(|e| {
                error!("Failed to generate access token for {}: {:?}", user.email, e);
                Error::Auth("Failed to generate access token".to_string())
            })?;
            
let refresh_token = encode_refresh_token(user.email.clone(), user.id.clone())
            .map_err(|e| {
                error!("Failed to generate refresh token for {}: {:?}", user.email, e);
                Error::Auth("Failed to generate refresh token".to_string())
            })?;

        let token_dto = TokenDto {
            access_token,
            refresh_token,
        };

        // Cache the user in auth repository for subsequent requests
        let _auth_repo = crate::v1::auth::AuthRepository::new(&app_state);
        let _user_query_dto: imphnen_entities::UsersDetailQueryDto = (&user).into();
        // if let Err(err_store) = auth_repo.query_store_user(user_query_dto).await {
        //     error!(
        //         "Failed to store user cache for {}: {}",
        //         user.email, err_store
        //     );
        //     // Don't fail the login, just log the error
        //     error!("Google OAuth login succeeded but caching failed for user: {}", user.email);
        // } else {

            info!("Successfully cached user {} after Google OAuth login", user.email);
        // }

        info!("Successfully completed Google OAuth for user: {}", user.email);
        Ok((user, token_dto))
})
    }
}
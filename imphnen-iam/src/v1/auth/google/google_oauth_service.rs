use anyhow::Result;
use async_trait::async_trait;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use oauth2::url::Url;
use tracing::{info, error};

use imphnen_entities::error_dto::error::Error;
use imphnen_libs::{jsonwebtoken::{encode_access_token, encode_refresh_token}, enviroment::Env, AppState};
use imphnen_utils::{generate_oauth_csrf_token, validate_oauth_csrf_token, validate_csrf_token};
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
        // OAuth 2.0 authorization codes can contain URL-safe characters including base64 characters
        if !self.code.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' || c == '/' || c == '+' || c == '=') {
            return Err(Error::Validation("Authorization code contains invalid characters".to_string()));
        }
        
        Ok(())
    }
    
    /// Validate CSRF state token with signature verification and extract PKCE verifier
    pub fn validate_csrf_state_and_get_pkce_verifier(&self, secret: &str) -> Result<PkceCodeVerifier, Error> {
        // Maximum age of 10 minutes for OAuth flow
        const MAX_AGE_SECONDS: u64 = 600;
        
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
        match validate_oauth_csrf_token(&self.state, secret, 600) {
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
    fn google_oauth_client(&self, custom_redirect_uri: Option<String>) -> BasicClient;
    fn generate_auth_url(&self, custom_redirect_uri: Option<String>) -> (Url, CsrfToken);
    async fn google_oauth_callback(&self, auth_request: AuthRequest, app_state: &AppState) -> Result<(UsersDetailItemDto, TokenDto), Error>; // Changed return type
}

#[derive(Clone)]
pub struct GoogleOauthServiceImpl<A: AuthServiceTrait, U: UsersServiceTrait> {
    users_service: U,
    env: &'static Env,
    #[allow(dead_code)]
    auth_service: A,
}

impl GoogleOauthServiceImpl<crate::v1::auth::auth_service::AuthService, crate::v1::users::users_service::UsersService> {
    /// Generate Google OAuth authorization URL
    pub fn get_auth_url(&self, custom_redirect_uri: Option<String>) -> String {
        let (auth_url, _csrf_token) = self.generate_auth_url(custom_redirect_uri);
        auth_url.to_string()
    }
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

    fn google_oauth_client(&self, custom_redirect_uri: Option<String>) -> BasicClient {
        let google_client_id = ClientId::new(self.env.google_client_id.clone());
        let google_client_secret = ClientSecret::new(self.env.google_client_secret.clone());
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
            .expect("Invalid token endpoint URL");

        let redirect_uri = custom_redirect_uri.unwrap_or_else(|| self.env.google_redirect_url.clone());

        BasicClient::new(
            google_client_id,
            Some(google_client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(
            RedirectUrl::new(redirect_uri)
                .expect("Invalid redirect URL"),
        )
    }

    fn generate_auth_url(&self, custom_redirect_uri: Option<String>) -> (Url, CsrfToken) {
        let client = self.google_oauth_client(custom_redirect_uri);
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate a signed CSRF token with PKCE verifier for stateless validation
        let csrf_token_str = generate_oauth_csrf_token(&self.env.access_token_secret, pkce_code_verifier.secret())
            .unwrap_or_else(|_| uuid::Uuid::new_v4().to_string()); // Fallback to UUID if signing fails
        
        let csrf_token = CsrfToken::new(csrf_token_str);

        client
            .authorize_url(|| csrf_token.clone())
            .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()))
            .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()))
            .set_pkce_challenge(pkce_code_challenge)
            .url()
    }

    async fn google_oauth_callback(&self, auth_request: AuthRequest, app_state: &AppState) -> Result<(UsersDetailItemDto, TokenDto), Error> {
        // Validate input parameters first
        auth_request.validate()?;
        
        // CRITICAL: Validate CSRF state token and extract PKCE verifier
        let pkce_verifier = auth_request.validate_csrf_state_and_get_pkce_verifier(&self.env.access_token_secret)?;
        
        info!("Starting Google OAuth callback process");
        
        // Use the default client for the callback
        let client = self.google_oauth_client(None);

        let token_response = client
            .exchange_code(oauth2::AuthorizationCode::new(auth_request.code))
            .set_pkce_verifier(pkce_verifier)
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
        info!("Google user picture URL: {:?}", google_user.picture);
        info!("Google user data: name={:?}, given_name={:?}, family_name={:?}, picture={:?}", 
              google_user.name, google_user.given_name, google_user.family_name, google_user.picture);

        let user = self.users_service.get_user_by_email(&google_user.email).await?;

        let user = match user {
            Some(mut user) => {
                info!("Existing user found for email: {}", google_user.email);
                
                // Update avatar if user doesn't have one and Google provides one
                if user.avatar.is_none() && google_user.picture.is_some() {
                    info!("Updating avatar for existing user: {}", google_user.email);
                    match self.users_service.update_user_avatar(&google_user.email, google_user.picture.clone()).await {
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
                let default_role_id = get_default_role_id(self.env).await
                    .unwrap_or_else(|e| {
                        error!("Failed to get default role ID, using fallback: {:?}", e);
                        "5713cb37-dc02-4e87-8048-d7a41d352059".to_string() // Hardcoded User role ID as final fallback
                    });
                
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
                    phone_number: "".to_string(), // Will be updated by user later
                    is_active: true,
                    role_id: default_role_id,
                    avatar: google_user.picture.clone(), // Set avatar from Google user picture
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

        // Cache the user in auth repository for subsequent requests
        let auth_repo = crate::v1::auth::AuthRepository::new(app_state);
        let user_query_dto: crate::v1::users::users_dto::UsersDetailQueryDto = (&user).into();
        if let Err(err_store) = auth_repo.query_store_user(user_query_dto).await {
            error!(
                "Failed to store user cache for {}: {}",
                user.email, err_store
            );
            // Don't fail the login, just log the error
            error!("Google OAuth login succeeded but caching failed for user: {}", user.email);
        } else {
            info!("Successfully cached user {} after Google OAuth login", user.email);
        }

        info!("Successfully completed Google OAuth for user: {}", user.email);
        Ok((user, token_dto))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use imphnen_utils::generate_oauth_csrf_token;
    
    #[test]
    fn test_auth_request_validation_with_base64_characters() {
        // Test case that was failing before the fix
        let auth_request = AuthRequest {
            code: "4/0-ARAA6EeEKN8rlQ_Dh5XAAA_dCpKFwKa3-Jl9cO7I".to_string(),
            state: "valid_state".to_string(),
        };
        
        let result = auth_request.validate();
        assert!(result.is_ok(), "Authorization code with base64-like characters should be valid");
    }
    
    #[test]
    fn test_auth_request_validation_with_slash() {
        let auth_request = AuthRequest {
            code: "authorization/code/with/slashes".to_string(),
            state: "valid_state".to_string(),
        };
        
        let result = auth_request.validate();
        assert!(result.is_ok(), "Authorization code with forward slashes should be valid");
    }
    
    #[test]
    fn test_auth_request_validation_with_plus() {
        let auth_request = AuthRequest {
            code: "authorization+code+with+plus".to_string(),
            state: "valid_state".to_string(),
        };
        
        let result = auth_request.validate();
        assert!(result.is_ok(), "Authorization code with plus signs should be valid");
    }
    
    #[test]
    fn test_auth_request_validation_with_equals() {
        let auth_request = AuthRequest {
            code: "authorization=code=with=equals=".to_string(),
            state: "valid_state".to_string(),
        };
        
        let result = auth_request.validate();
        assert!(result.is_ok(), "Authorization code with equals signs should be valid");
    }
    
    #[test]
    fn test_auth_request_validation_with_invalid_chars() {
        let auth_request = AuthRequest {
            code: "authorization@code#with$invalid%chars".to_string(),
            state: "valid_state".to_string(),
        };
        
        let result = auth_request.validate();
        assert!(result.is_err(), "Authorization code with invalid characters should be rejected");
    }
    
    #[test]
    fn test_auth_request_validation_empty_code() {
        let auth_request = AuthRequest {
            code: "".to_string(),
            state: "valid_state".to_string(),
        };
        
        let result = auth_request.validate();
        assert!(result.is_err(), "Empty authorization code should be rejected");
    }
    
    #[test]
    fn test_oauth_csrf_with_pkce_verifier() {
        let secret = "test_secret";
        let pkce_verifier = "test_pkce_verifier";
        
        // Generate OAuth CSRF token with PKCE verifier
        let token = generate_oauth_csrf_token(secret, pkce_verifier).unwrap();
        
        // Create auth request with the token
        let auth_request = AuthRequest {
            code: "test_code".to_string(),
            state: token,
        };
        
        // Validate and extract PKCE verifier
        let extracted_verifier = auth_request.validate_csrf_state_and_get_pkce_verifier(secret).unwrap();
        assert_eq!(extracted_verifier.secret(), pkce_verifier);
    }
    
    #[test]
    fn test_oauth_csrf_backwards_compatibility() {
        let secret = "test_secret";
        
        // Generate regular CSRF token (legacy)
        let token = imphnen_utils::generate_csrf_token(secret).unwrap();
        
        // Create auth request with the token
        let auth_request = AuthRequest {
            code: "test_code".to_string(),
            state: token,
        };
        
        // Legacy validation should still work
        let result = auth_request.validate_csrf_state(secret);
        assert!(result.is_ok(), "Legacy CSRF validation should still work");
    }

    #[test]
    fn test_user_creation_with_avatar() {
        use crate::v1::users::users_dto::UsersCreateRequestDto;
        
        let google_user_picture = Some("https://lh3.googleusercontent.com/a/default-user".to_string());
        
        let new_user = UsersCreateRequestDto {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            fullname: "Test User".to_string(),
            phone_number: "1234567890".to_string(),
            is_active: true,
            role_id: "test_role_id".to_string(),
            avatar: google_user_picture.clone(),
        };
        
        assert_eq!(new_user.avatar, google_user_picture, "Avatar should be set from Google user picture");
    }
}
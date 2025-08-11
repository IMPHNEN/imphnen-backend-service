use anyhow::Result;
use async_trait::async_trait;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use oauth2::url::Url;

use imphnen_entities::error_dto::error::Error;
use imphnen_libs::{jsonwebtoken::{encode_access_token, encode_refresh_token}, enviroment::Env};
use crate::v1::auth::TokenDto;
use crate::v1::auth::auth_service::AuthServiceTrait;
use crate::v1::users::users_dto::{UsersCreateRequestDto, UsersDetailItemDto};
use crate::v1::users::users_service::UsersServiceTrait;

use super::google_oauth_dto::{GoogleTokenResponse, GoogleUser};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
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

        client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()))
            .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()))
            .set_pkce_challenge(pkce_code_challenge)
            .url()
    }

    async fn google_oauth_callback(&self, auth_request: AuthRequest) -> Result<(UsersDetailItemDto, TokenDto), Error> {
        let client = self.google_oauth_client();

        let token_response = client
            .exchange_code(oauth2::AuthorizationCode::new(auth_request.code))
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .map_err(|e| Error::Db(format!("Failed to exchange code: {}", e)))?;

        let google_token_response: GoogleTokenResponse = serde_json::from_str(&token_response.access_token().secret())
            .map_err(|e| Error::Db(format!("Failed to parse Google token response: {}", e)))?;

        let client = reqwest::Client::new();
        let user_info_url = "https://www.googleapis.com/oauth2/v2/userinfo";
        let google_user: GoogleUser = client
            .get(user_info_url)
            .bearer_auth(google_token_response.access_token)
            .send()
            .await
            .map_err(|e| Error::Db(format!("Failed to fetch user info: {}", e)))?
            .json()
            .await
            .map_err(|e| Error::Db(format!("Failed to parse user info: {}", e)))?;

        let user = self.users_service.get_user_by_email(&google_user.email).await?;

        let user = match user {
            Some(user) => user,
            None => {
                let new_user = UsersCreateRequestDto {
                    email: google_user.email,
                    password: "GOOGLE_OAUTH_PASSWORD".to_string(), // Placeholder password as it's not used
                    fullname: google_user.name.clone(), // Use fullname for UsersCreateRequestDto
                    phone_number: "N/A".to_string(), // Placeholder for phone number
                    is_active: true, // Assuming active by default for new Google users
                    role_id: "default_role_id".to_string(), // Placeholder for role_id
                };
                self.users_service.create_user_by_dto(new_user).await?
            }
        };

        let access_token = encode_access_token(user.email.clone())
            .map_err(|e| Error::Db(format!("Failed to generate access token: {}", e)))?;
        let refresh_token = encode_refresh_token(user.email.clone())
            .map_err(|e| Error::Db(format!("Failed to generate refresh token: {}", e)))?;

        let token_dto = TokenDto {
            access_token,
            refresh_token,
        };

        Ok((user, token_dto))
    }
}
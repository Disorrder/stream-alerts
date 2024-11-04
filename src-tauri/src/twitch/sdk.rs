use super::mock_client;
use super::oauth2::{TokenResponse, TwitchOAuthService};
use super::store::TwitchStore;
use crate::config::store::Store;
use futures::TryStreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use twitch_api::twitch_oauth2::{AccessToken, ClientSecret, RefreshToken, TwitchToken, UserToken};
use twitch_api::{helix, HelixClient};

pub enum TwitchSDKError {
    NotConnected,
    // TODO: TokenExpired,
    String(String),
}

impl TwitchSDKError {
    pub fn message(&self) -> String {
        match self {
            Self::NotConnected => "Twitch account is not connected".to_string(),
            Self::String(s) => s.clone(),
        }
    }
}

pub struct TwitchSDK {
    client: HelixClient<'static, reqwest::Client>,
    store: Arc<Store>,
    oauth_service: Arc<TwitchOAuthService>,
    token: Mutex<Option<UserToken>>,
}

impl TwitchSDK {
    pub fn new(store: Arc<Store>, oauth_service: Arc<TwitchOAuthService>) -> Self {
        let client = HelixClient::default();
        Self {
            client,
            store,
            oauth_service,
            token: Mutex::new(None),
        }
    }

    pub fn get_client(&self) -> HelixClient<'static, reqwest::Client> {
        self.client.clone()
    }

    pub fn get_mock_client(&self) -> HelixClient<'static, mock_client::Wrapper<reqwest::Client>> {
        let client = mock_client::Wrapper::<reqwest::Client>::new(
            twitch_api::client::ClientDefault::default_client(),
            "http://127.0.0.1:8080".to_string(),
        );
        twitch_api::HelixClient::with_client(client)
    }

    pub async fn get_user_token(&self) -> Result<Option<UserToken>, String> {
        let refresh_token: String;

        let token_guard = self.token.lock().await;

        // If we have a token already, return it
        if let Some(token) = token_guard.as_ref() {
            if !token.is_elapsed() {
                println!("[DEBUG] TwitchSDK::get_user_token: return token that is not expired");
                return Ok(Some(token.clone()));
            }
            // If token is expired, refresh it
            refresh_token = match &token.refresh_token {
                Some(refresh_token) => refresh_token.to_string(),
                None => return Err(TwitchSDKError::NotConnected.message()), // Never gonna happen
            };
        } else {
            // If we don't have a token, try to get it from the store
            let tokens_res = match self.store.get_twitch_tokens() {
                Ok(data) => data,
                Err(_) => None, // Never gonna happen
            };

            if let Some(token_data) = tokens_res {
                refresh_token = token_data.refresh_token;
            } else {
                return Ok(None);
            }
        }

        let token_data = self
            .oauth_service
            .refresh_token(&refresh_token)
            .await
            .map_err(|e| format!("{:?}", e))?;

        drop(token_guard);

        let user_token = self
            .set_tokens(token_data.access_token, token_data.refresh_token)
            .await?;

        println!("[DEBUG] TwitchSDK::get_user_token: return refreshed token");
        return Ok(user_token);
    }

    pub async fn set_tokens(
        &self,
        access_token: String,
        refresh_token: String,
    ) -> Result<Option<UserToken>, String> {
        let client_secret = ClientSecret::new(
            env!(
                "TWITCH_CLIENT_SECRET",
                "TWITCH_CLIENT_SECRET not set at build time"
            )
            .to_string(),
        );

        let mut token_mut = self.token.lock().await;

        let token_res = UserToken::from_existing(
            &self.client,
            AccessToken::new(access_token.clone()),
            RefreshToken::new(refresh_token.clone()),
            client_secret,
        )
        .await;

        let user_token = match token_res {
            Ok(user_token) => {
                *token_mut = Some(user_token.clone());
                user_token
            }
            Err(e) => {
                println!("[ERROR] TwitchSDK::set_tokens: {}", e);
                return Err(e.to_string());
            }
        };

        let token_data = TokenResponse::new(access_token, refresh_token);
        self.store.set_twitch_tokens(&token_data).map_err(|e| {
            println!("[ERROR] TwitchSDK::set_tokens: {}", e);
            e.to_string()
        })?;

        Ok(Some(user_token))
    }

    pub async fn reset_token(&self) -> Result<(), String> {
        let mut token_mut = self.token.lock().await;
        *token_mut = None;
        Ok(())
    }

    // async fn

    pub async fn get_user(&self) -> Result<Option<helix::users::User>, String> {
        let token = self.get_user_token().await?;
        if token.is_none() {
            return Ok(None);
        }
        let token = token.unwrap();
        let user_id = token.user_id.clone();

        let client = self.client.clone();
        let users: Vec<helix::users::User> = client
            .get_users_from_ids(&[user_id][..].into(), &token)
            .try_collect()
            .await
            .map_err(|e| {
                println!("[ERROR] TwitchSDK::get_user: {}", e); //? throw 401
                e.to_string()
            })?;

        Ok(users.first().cloned())
    }

    pub async fn get_followers_count(&self) -> Result<u64, TwitchSDKError> {
        let token = self
            .get_user_token()
            .await
            .map_err(TwitchSDKError::String)?;
        if token.is_none() {
            return Err(TwitchSDKError::NotConnected);
        }
        let token = token.unwrap();
        let user_id = token.user_id.clone();

        let client = self.client.clone();
        let followers = client
            .get_total_channel_followers(&user_id, &token)
            .await
            .map_err(|e| TwitchSDKError::String(e.to_string()))?;

        Ok(followers as u64)
    }
}

use super::store::TwitchStore;
use crate::config::store::Store;
use futures::TryStreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use twitch_api::twitch_oauth2::{AccessToken, ClientSecret, RefreshToken, UserToken};
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
    token: Arc<Mutex<Option<UserToken>>>,
    store: Arc<Store>,
}

impl TwitchSDK {
    pub fn new(store: Arc<Store>) -> Self {
        let client = HelixClient::default();

        Self {
            client,
            token: Arc::new(Mutex::new(None)),
            store,
        }
    }

    async fn get_or_create_token(&self) -> Result<Option<UserToken>, String> {
        // If we already have a token, return it
        let token_ref = self.token.clone();
        let token_mut = token_ref.lock().await;

        if let Some(token) = token_mut.as_ref() {
            return Ok(Some(token.clone()));
        }

        drop(token_mut);

        let client_secret = ClientSecret::new(
            env!(
                "TWITCH_CLIENT_SECRET",
                "TWITCH_CLIENT_SECRET not set at build time"
            )
            .to_string(),
        );
        let token_data = match self.store.get_twitch_tokens() {
            Ok(data) => data,
            Err(e) => {
                println!("[ERROR] TwitchSDK::get_or_create_token: {}", e);
                return Err(e.to_string());
            }
        };

        if token_data.is_none() {
            return Ok(None);
        }
        let token_data = token_data.unwrap();
        let access_token = AccessToken::new(token_data.access_token);
        let refresh_token = RefreshToken::new(token_data.refresh_token);
        let token_res = UserToken::from_existing(
            &self.client,
            access_token.clone(),
            refresh_token.clone(),
            client_secret,
        )
        .await;
        match token_res {
            Ok(token_data) => {
                *token_ref.lock().await = Some(token_data.clone());
                Ok(Some(token_data))
            }
            Err(e) => {
                println!("[ERROR] TwitchSDK::get_token: {}", e);
                Err(e.to_string())
            }
        }
    }

    pub async fn reset_token(&self) -> Result<(), String> {
        let token_ref = self.token.clone();
        *token_ref.lock().await = None;
        Ok(())
    }

    pub async fn get_user(&self) -> Result<Option<helix::users::User>, String> {
        let token = self.get_or_create_token().await?;
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

        let user = users.into_iter().next();
        Ok(user)
    }

    pub async fn get_followers_count(&self) -> Result<u64, TwitchSDKError> {
        let token = self
            .get_or_create_token()
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

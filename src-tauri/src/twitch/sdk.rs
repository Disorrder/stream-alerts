use super::store::TwitchStore;
use crate::config::store::Store;
use futures::TryStreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use twitch_api::twitch_oauth2::{AccessToken, ClientSecret, RefreshToken, UserToken};
use twitch_api::{helix, HelixClient};

pub struct TwitchSDK {
    client: HelixClient<'static, reqwest::Client>,
    token: Arc<Mutex<Option<UserToken>>>,
    store: Store,
}

impl TwitchSDK {
    pub fn new(store: Store) -> Self {
        let client = HelixClient::default();

        Self {
            client,
            token: Arc::new(Mutex::new(None)),
            store,
        }
    }

    async fn get_or_create_token(&self) -> Result<UserToken, String> {
        // If we already have a token, return it
        let token_ref = self.token.clone();
        let token_mut = token_ref.lock().await;

        if let Some(token) = token_mut.as_ref() {
            return Ok(token.clone());
        }

        drop(token_mut);

        let client_secret = ClientSecret::new(
            env!(
                "TWITCH_CLIENT_SECRET",
                "TWITCH_CLIENT_SECRET not set at build time"
            )
            .to_string(),
        );
        let token_data = self.store.get_twitch_tokens().unwrap().unwrap(); //?
        let access_token = AccessToken::new(token_data.access_token);
        let refresh_token = RefreshToken::new(token_data.refresh_token);
        let token_res = UserToken::from_existing(
            &self.client,
            access_token.clone(),
            refresh_token.clone(),
            client_secret,
        )
        .await;
        println!(
            "[DEBUG] TwitchSDK::get_or_create_token access_token {:?}",
            access_token.secret()
        );
        println!(
            "[DEBUG] TwitchSDK::get_or_create_token refresh_token {:?}",
            refresh_token.as_str()
        );
        match token_res {
            Ok(token_data) => {
                println!(
                    "[DEBUG] TwitchSDK::get_or_create_token pre token_data {:?}",
                    token_data
                );
                *token_ref.lock().await = Some(token_data.clone());
                println!(
                    "[DEBUG] TwitchSDK::get_or_create_token token_data {:?}",
                    token_data
                );
                Ok(token_data)
            }
            Err(e) => {
                println!("[ERROR] TwitchSDK::get_token: {}", e);
                Err(e.to_string())
            }
        }
    }

    pub async fn get_user(&self) -> Result<helix::users::User, String> {
        let token = self.get_or_create_token().await?;
        let user_id = token.user_id.clone();

        let client = self.client.clone();
        let users: Vec<helix::users::User> = client
            .get_users_from_ids(&[user_id][..].into(), &token)
            .try_collect()
            .await
            .map_err(|e| {
                println!("[ERROR] TwitchSDK::get_user: {}", e);
                e.to_string()
            })?;

        users
            .into_iter()
            .next()
            .ok_or_else(|| "User not found".to_string())
    }
}

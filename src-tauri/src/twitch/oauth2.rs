use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i32,
    scope: Vec<String>,
    token_type: String,
}

pub struct TwitchOAuthService {
    client: Client,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl TwitchOAuthService {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            client_id: env::var("TWITCH_CLIENT_ID")
                .map_err(|_| anyhow!("TWITCH_CLIENT_ID must be set"))?,
            client_secret: env::var("TWITCH_CLIENT_SECRET")
                .map_err(|_| anyhow!("TWITCH_CLIENT_SECRET must be set"))?,
            redirect_uri: String::from("http://localhost:6969/auth/twitch-callback"),
        })
    }

    pub async fn exchange_code_for_token(&self, code: &str) -> Result<TokenResponse> {
        println!("Exchanging code for token: {}", code);
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("code", code),
            ("grant_type", "authorization_code"),
            ("redirect_uri", self.redirect_uri.as_str()),
        ];

        let response = self
            .client
            .post("https://id.twitch.tv/oauth2/token")
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Failed to exchange code: {}", error_text));
        }

        let token_data = response.json::<TokenResponse>().await?;
        Ok(token_data)
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokenResponse> {
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token"),
        ];

        let response = self
            .client
            .post("https://id.twitch.tv/oauth2/token")
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Failed to refresh token: {}", error_text));
        }

        let token_data = response.json::<TokenResponse>().await?;
        Ok(token_data)
    }

    pub fn get_authorization_url(&self, state: &str) -> String {
        format!(
            "https://id.twitch.tv/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=channel:read:subscriptions",
            self.client_id, self.redirect_uri,
        )
    }
}

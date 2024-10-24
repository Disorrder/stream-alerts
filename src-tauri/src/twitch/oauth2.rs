use anyhow::{anyhow, Result};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
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
        let client_id =
            env!("TWITCH_CLIENT_ID", "TWITCH_CLIENT_ID not set at build time").to_string();
        let client_secret = env!(
            "TWITCH_CLIENT_SECRET",
            "TWITCH_CLIENT_SECRET not set at build time"
        )
        .to_string();
        let port = env!("TAURI_WEB_PORT", "TAURI_WEB_PORT not set at build time")
            .to_string()
            .parse::<u16>()
            .unwrap_or(6969);
        let redirect_uri = format!("http://localhost:{}/auth/twitch-callback", port);

        Ok(Self {
            client: Client::new(),
            client_id,
            client_secret,
            redirect_uri,
        })
    }

    pub async fn exchange_code_for_token(
        &self,
        code: &str,
    ) -> Result<TokenResponse, (StatusCode, String)> {
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
            .await
            .map_err(|e| {
                (
                    e.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                    e.to_string(),
                )
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or("Failed to auth user".to_string());
            return Err((status, error_text));
        }

        let token_data = response.json::<TokenResponse>().await.map_err(|e| {
            (
                e.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                e.to_string(),
            )
        })?;
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

    pub fn get_authorization_url(&self, scopes: Option<Vec<&str>>) -> String {
        let scopes = scopes.unwrap_or_else(|| {
            vec![
                "channel:read:hype_train", // TODO
                // "channel:read:redemptions", // TODO
                "channel:read:subscriptions",
                // "channel:read:vips", // TODO
                // "moderation:read", // TODO
                // "moderator:read:banned_users", // TODO
                // "moderator:read:shoutouts", //? Not sure, need to explore
                "moderator:read:followers",
                "user:read:email",
                "user:read:subscriptions",
            ]
        });
        let scope = scopes.join("+");
        format!(
            "https://id.twitch.tv/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}",
            self.client_id, self.redirect_uri, scope,
        )
    }
}

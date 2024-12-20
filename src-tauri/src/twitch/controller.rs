use super::oauth2::{TokenResponse, TWITCH_DEFAULT_SCOPE};
use crate::common::http_error::HttpError;
use crate::twitch::oauth2::TwitchOAuthService;
use crate::twitch::sdk::{TwitchSDK, TwitchSDKError};
use crate::{config::store::Store, twitch::store::TwitchStore};
use axum::routing::{delete, get};
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tauri::{App, Manager};
use tauri_plugin_shell::ShellExt;

pub struct TwitchState {
    app_handle: tauri::AppHandle,
    store: Arc<Store>,
    oauth_service: Arc<TwitchOAuthService>,
    sdk: Arc<TwitchSDK>,
}

pub fn routes(app: &mut App) -> Router {
    let app_handle = app.handle().clone();
    let store = app.state::<Arc<Store>>().inner().clone();

    let oauth_service = TwitchOAuthService::new();
    let oauth_service = Arc::new(oauth_service);

    let sdk = TwitchSDK::new(store.clone(), oauth_service.clone());
    let sdk = Arc::new(sdk);
    app.manage(sdk.clone());

    let state = Arc::new(TwitchState {
        app_handle,
        store,
        oauth_service,
        sdk,
    });

    let router = Router::new()
        // Dev Routes
        .route("/dev-auth/tokens", post(auth_by_tokens))
        // User Routes
        .route("/auth/open", post(open_oauth_window))
        .route("/auth/code", post(auth_by_code))
        .route("/refresh", post(refresh_token))
        .route("/user", get(get_user))
        .route("/user", delete(detach_user))
        .route("/followers", get(get_followers_count))
        .with_state(state);
    router
}

async fn open_oauth_window(State(state): State<Arc<TwitchState>>) {
    let auth_url = state.oauth_service.get_authorization_url(None);
    println!("auth_url: {}", auth_url);

    // Open the default browser with the Twitch auth URL
    let _ = state
        .app_handle
        .shell()
        .open(&auth_url, None)
        .map_err(|e| e.to_string());
}

#[derive(Deserialize)]
struct AuthPayloadDTO {
    code: String,
}

async fn auth_by_code(
    State(state): State<Arc<TwitchState>>,
    Json(payload): Json<AuthPayloadDTO>,
) -> Result<impl IntoResponse, HttpError> {
    let code = payload.code;
    let token_data = state.oauth_service.exchange_code_for_token(&code).await?;
    state
        .store
        .set_twitch_tokens(&token_data)
        .map_err(|e| HttpError::StoreFailed(format!("Can't save tokens, {}", e.to_string())))?;
    Ok((StatusCode::OK, "OK".to_string()))
}

#[derive(Deserialize)]
struct AuthByTokensPayloadDTO {
    access_token: String,
    refresh_token: String,
}

// Check README.md about Twitch CLI usage
async fn auth_by_tokens(
    State(state): State<Arc<TwitchState>>,
    Json(payload): Json<AuthByTokensPayloadDTO>,
) -> Result<impl IntoResponse, HttpError> {
    let access_token = payload.access_token;
    let refresh_token = payload.refresh_token;
    // let token_data = TokenResponse::new(access_token, refresh_token);
    // TODO: Check if token is valid
    state
        .sdk
        .set_tokens(access_token, refresh_token)
        .await
        .map_err(|e| HttpError::StoreFailed(format!("Can't save tokens, {}", e.to_string())))?;
    Ok((StatusCode::OK, "OK".to_string()))
}

async fn refresh_token(
    State(state): State<Arc<TwitchState>>,
) -> Result<impl IntoResponse, HttpError> {
    let token_data = match state.store.get_twitch_tokens() {
        Ok(Some(data)) => data,
        Ok(None) => return Err(HttpError::Unauthorized("No token data".to_string())),
        Err(e) => {
            println!("🚀 ~ refresh_token ~ e: {:?}", e);
            return Err(HttpError::Unauthorized(e.to_string()));
        }
    };

    let refresh_token = token_data.refresh_token;
    let token_data = state.oauth_service.refresh_token(&refresh_token).await?;
    let _ = state
        .store
        .set_twitch_tokens(&token_data)
        .map_err(|e| HttpError::StoreFailed(e.to_string()));

    Ok((StatusCode::OK, "OK".to_string()))
}

async fn get_user(State(state): State<Arc<TwitchState>>) -> impl IntoResponse {
    let user = state.sdk.get_user().await;
    println!("[DEBUG] USER get_user: user: {:?}", user);
    match user {
        Ok(user) => (StatusCode::OK, Json(json!(user))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e))),
    }
}

async fn detach_user(
    State(state): State<Arc<TwitchState>>,
) -> Result<impl IntoResponse, HttpError> {
    state
        .store
        .delete_twitch_tokens()
        .map_err(|e| HttpError::StoreFailed(format!("Can't delete tokens, {}", e.to_string())))?;
    state
        .sdk
        .reset_token()
        .await
        .map_err(|e| HttpError::StoreFailed(e.to_string()))?;
    Ok((StatusCode::OK, "OK".to_string()))
}

async fn get_followers_count(
    State(state): State<Arc<TwitchState>>,
) -> Result<impl IntoResponse, HttpError> {
    let count = state.sdk.get_followers_count().await.map_err(|e| match e {
        TwitchSDKError::NotConnected => HttpError::BadRequest(e.message()),
        TwitchSDKError::String(s) => HttpError::BadRequest(s),
    })?;
    Ok((StatusCode::OK, Json(json!(count))))
}

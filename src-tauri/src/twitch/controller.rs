use crate::common::errors::HttpError;
use crate::twitch::oauth2::TwitchOAuthService;
use crate::twitch::sdk::TwitchSDK;
use crate::{config::store::Store, twitch::store::TwitchStore};
use axum::routing::get;
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

pub struct TwitchState {
    store: Store,
    oauth_service: TwitchOAuthService,
    sdk: TwitchSDK,
}

pub fn routes(store: Store) -> Router {
    let oauth_service = TwitchOAuthService::new().unwrap();
    let sdk = TwitchSDK::new(store.clone());

    let state = Arc::new(TwitchState {
        store,
        oauth_service,
        sdk,
    });

    let router = Router::new()
        .route("/auth/code", post(auth_by_code))
        .route("/refresh", post(refresh_token))
        .route("/user", get(get_user))
        .route("/followers", get(get_followers_count))
        .with_state(state);
    router
}

#[derive(Deserialize)]
struct AuthPayload {
    code: String,
}

async fn auth_by_code(
    State(state): State<Arc<TwitchState>>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse, HttpError> {
    let code = payload.code;
    let token_data = state.oauth_service.exchange_code_for_token(&code).await?;
    state.store.set_twitch_tokens(&token_data)?;
    Ok((StatusCode::OK, "OK".to_string()))
}

async fn refresh_token(
    State(state): State<Arc<TwitchState>>,
) -> Result<impl IntoResponse, HttpError> {
    let token_data = match state.store.get_twitch_tokens() {
        Ok(Some(data)) => data,
        Ok(None) => return Err(HttpError::AuthError("No token data".to_string())),
        Err(e) => {
            println!("🚀 ~ refresh_token ~ e: {:?}", e);
            return Err(HttpError::AuthError(e.to_string()));
        }
    };

    let refresh_token = token_data.refresh_token;
    let token_data = state.oauth_service.refresh_token(&refresh_token).await?;
    state.store.set_twitch_tokens(&token_data)?;

    Ok((StatusCode::OK, "OK".to_string()))
}

async fn get_user(State(state): State<Arc<TwitchState>>) -> impl IntoResponse {
    let user = state.sdk.get_user().await;
    match user {
        Ok(user) => (StatusCode::OK, Json(json!(user))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e))),
    }
}

async fn get_followers_count(State(state): State<Arc<TwitchState>>) -> impl IntoResponse {
    let count = state.sdk.get_followers_count().await;
    (StatusCode::OK, Json(json!(count)))
}

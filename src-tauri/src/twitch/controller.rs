use crate::twitch::oauth2::TwitchOAuthService;
use crate::twitch::sdk::TwitchSDK;
use crate::{config::store::Store, twitch::store::TwitchStore};
use axum::routing::get;
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use reqwest::StatusCode;
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
        .route("/user", get(get_user))
        .with_state(state);
    router
}

async fn auth_by_code(
    State(state): State<Arc<TwitchState>>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    println!("Received event: {:?}", payload);
    let code = payload.get("code").unwrap().as_str().unwrap();

    let token_data = state
        .oauth_service
        .exchange_code_for_token(code)
        .await
        .map_err(|e| {
            eprintln!("Failed to exchange code for token: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to authenticate")
        });

    println!("Token data: {:#?}", token_data);

    state.store.set_twitch_tokens(&token_data.unwrap()).unwrap();

    (StatusCode::OK, "OK")
}

async fn get_user(State(state): State<Arc<TwitchState>>) -> impl IntoResponse {
    let user = state.sdk.get_user().await;
    match user {
        Ok(user) => (StatusCode::OK, Json(json!(user))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e })),
        ),
    }
}

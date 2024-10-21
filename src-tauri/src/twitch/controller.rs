use std::sync::Arc;

use super::store::TwitchStore;
use crate::config::store::Store;
use crate::twitch::oauth2::TwitchOAuthService;
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use reqwest::StatusCode;

pub struct TwitchState {
    store: TwitchStore,
    oauth_service: TwitchOAuthService,
}

pub fn routes(store: Store) -> Router {
    let oauth_service = TwitchOAuthService::new().unwrap();
    let twitch_store = TwitchStore::new(store).unwrap();

    let state = Arc::new(TwitchState {
        store: twitch_store,
        oauth_service,
    });

    let router = Router::new()
        .route("/auth/code", post(auth_by_code))
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

    state.store.set_tokens(&token_data.unwrap()).unwrap();

    (StatusCode::OK, "OK")
}

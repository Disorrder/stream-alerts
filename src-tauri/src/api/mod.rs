use std::env;

use crate::config::store::Store;
use crate::twitch::controller::routes as twitch_routes;
use axum::Router;
// use std::env;
use tower_http::cors::CorsLayer;

pub async fn run(store: Store) -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("TAURI_API_PORT").unwrap_or("6967".to_string());
    let host = format!("0.0.0.0:{}", port);

    let app = Router::new()
        .nest("/twitch", twitch_routes(store))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

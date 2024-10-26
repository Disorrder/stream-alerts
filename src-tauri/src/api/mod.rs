use crate::config::store::Store;
use crate::twitch::controller::routes as twitch_routes;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub async fn run(
    app_handle: Arc<tauri::AppHandle>,
    store: Store,
) -> Result<(), Box<dyn std::error::Error>> {
    let port = env!("TAURI_API_PORT", "TAURI_API_PORT not set at build time")
        .to_string()
        .parse::<u16>()
        .unwrap_or(6967);
    let host = format!("0.0.0.0:{}", port);

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/twitch", twitch_routes(app_handle, store))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

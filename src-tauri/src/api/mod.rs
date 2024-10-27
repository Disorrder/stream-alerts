use crate::twitch::controller::routes as twitch_routes;
use axum::routing::get;
use axum::Router;
use tauri::App;
use tower_http::cors::CorsLayer;

pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let port = env!("TAURI_API_PORT", "TAURI_API_PORT not set at build time")
        .to_string()
        .parse::<u16>()
        .unwrap_or(6967);
    let host = format!("0.0.0.0:{}", port);

    let router = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/twitch", twitch_routes(app))
        .layer(CorsLayer::permissive());

    tauri::async_runtime::spawn(async move {
        println!("Starting REST API server on {}", host);
        let listener = tokio::net::TcpListener::bind(host).await.unwrap();
        axum::serve(listener, router).await.unwrap();
    });

    Ok(())
}

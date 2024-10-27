use axum::routing::get;
use socketioxide::{extract::SocketRef, SocketIo};
use std::sync::Arc;
use tauri::{App, Manager};
use tower_http::cors::CorsLayer;

pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let port = env!("TAURI_WS_PORT", "TAURI_WS_PORT not set at build time")
        .to_string()
        .parse::<u16>()
        .unwrap_or(6968);
    let host = format!("0.0.0.0:{}", port);
    let (layer, io) = SocketIo::new_layer();

    // Register a handler for the default namespace
    io.ns("/", move |s: SocketRef| {
        s.on("message", &message);
        s.on("profile:get", &get_profile);
    });

    let router = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer)
        .layer(CorsLayer::permissive());

    let io = Arc::new(io);
    app.manage(io);

    tauri::async_runtime::spawn(async move {
        println!("Starting websocket server on {}", host);
        let listener = tokio::net::TcpListener::bind(host).await.unwrap();
        axum::serve(listener, router).await.unwrap();
    });

    Ok(())
}

// For each "message" event received, send a "message-back" event with the "Hello World!" event
fn message(s: SocketRef) {
    s.emit("message-back", "Hello World!").ok();
}

fn get_profile(s: SocketRef) {
    println!("[Rust] Get Profile");
    s.emit("profile:patch", "TODO: Profile Data").ok();
}

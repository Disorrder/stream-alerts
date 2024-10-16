mod twitch;

use axum::routing::get;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use twitch::{TwitchCode, WebsocketTwitchController};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (layer, io) = SocketIo::new_layer();

    let twitch_controller = WebsocketTwitchController::new()?;
    let twitch_controller = Arc::new(twitch_controller);

    // Register a handler for the default namespace
    io.ns("/", move |s: SocketRef| {
        s.on("message", &message);
        s.on("profile:get", &get_profile);

        s.on(
            "twitch:auth_by_code",
            move |s: SocketRef, data: Data<TwitchCode>| {
                let twitch_controller = twitch_controller.clone();
                tokio::spawn(async move {
                    let _ = twitch_controller.auth_by_code(s, data).await;
                });
            },
        );
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6968").await.unwrap();
    axum::serve(listener, app).await.unwrap();

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

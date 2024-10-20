use axum::routing::get;
use socketioxide::{extract::SocketRef, SocketIo};
use tower_http::cors::{Any, CorsLayer};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (layer, io) = SocketIo::new_layer();

    // Register a handler for the default namespace
    io.ns("/", move |s: SocketRef| {
        s.on("message", &message);
        s.on("profile:get", &get_profile);
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

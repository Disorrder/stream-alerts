mod store;
mod websocket;

use std::env;
use tauri_plugin_shell::ShellExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let _app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                let _ = websocket::run().await;
            });
            Ok(())
        })
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, authenticate_twitch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn authenticate_twitch(app_handle: tauri::AppHandle) {
    let client_id = env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID must be set");
    let redirect_uri = "http://localhost:6969/auth/twitch-callback";

    let auth_url = format!(
        "https://id.twitch.tv/oauth2/authorize?client_id={}&redirect_uri={}&response_type=token&scope=channel:read:subscriptions",
        client_id, redirect_uri
    );

    // Open the default browser with the Twitch auth URL
    let _ = app_handle
        .shell()
        .open(&auth_url, None)
        .map_err(|e| e.to_string());
}

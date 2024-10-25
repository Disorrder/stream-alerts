mod api;
mod common;
mod config;
mod twitch;
mod websocket;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let port = env!("TAURI_WEB_PORT", "TAURI_WEB_PORT not set at build time")
        .to_string()
        .parse::<u16>()
        .unwrap_or(6969);

    tauri::Builder::default()
        .setup(move |app| {
            let app_handle = app.handle();

            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }

            let app_data_dir = app_handle.path().app_data_dir().unwrap();
            let store = config::store::Store::new(app_data_dir).unwrap();

            tauri::async_runtime::spawn(async move {
                println!("Starting websocket server");
                let _ = websocket::run().await;
            });
            tauri::async_runtime::spawn(async move {
                println!("Starting REST API server");
                let _ = api::run(store).await;
            });

            Ok(())
        })
        .plugin(tauri_plugin_localhost::Builder::new(port).build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            twitch::commands::twitch_open_oauth
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

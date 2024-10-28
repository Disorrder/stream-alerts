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
            println!("Setting up app");
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }

            config::store::setup(app)?;
            config::db::setup(app)?;
            websocket::setup(app)?;
            api::setup(app)?;
            twitch::eventsub::setup(app)?;

            Ok(())
        })
        .plugin(tauri_plugin_localhost::Builder::new(port).build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

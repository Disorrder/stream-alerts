use std::env;
use tauri_plugin_shell::ShellExt;

#[tauri::command]
pub fn twitch_open_oauth(app_handle: tauri::AppHandle) {
    // let client_id = env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID must be set");
    let client_id = "48xtpnf4j5zyr1ib91bncm8yucq2rj";
    let redirect_uri = "http://localhost:6969/auth/twitch-callback";

    let auth_url = format!(
        "https://id.twitch.tv/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=channel:read:subscriptions",
        client_id, redirect_uri
    );

    // Open the default browser with the Twitch auth URL
    let _ = app_handle
        .shell()
        .open(&auth_url, None)
        .map_err(|e| e.to_string());
}

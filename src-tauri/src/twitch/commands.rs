use tauri_plugin_shell::ShellExt;

#[tauri::command]
pub fn twitch_open_oauth(app_handle: tauri::AppHandle) {
    let client_id = env!("TWITCH_CLIENT_ID", "TWITCH_CLIENT_ID not set at build time").to_string();
    let port = env!("TAURI_WEB_PORT", "TAURI_WEB_PORT not set at build time")
        .to_string()
        .parse::<u16>()
        .unwrap_or(6969);
    let redirect_uri = format!("http://localhost:{}/auth/twitch-callback", port);

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

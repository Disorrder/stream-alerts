use tauri_plugin_shell::ShellExt;

use super::oauth2::TwitchOAuthService;

#[tauri::command]
pub fn twitch_open_oauth(app_handle: tauri::AppHandle) {
    let oauth_service = TwitchOAuthService::new();
    let auth_url = oauth_service.get_authorization_url(None);
    println!("auth_url: {}", auth_url);

    // Open the default browser with the Twitch auth URL
    let _ = app_handle
        .shell()
        .open(&auth_url, None)
        .map_err(|e| e.to_string());
}

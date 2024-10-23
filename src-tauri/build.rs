use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok(); // This will load the .env file

    println!(
        "cargo:rustc-env=TAURI_WEB_PORT={}",
        env::var("TAURI_WEB_PORT").unwrap()
    );
    println!(
        "cargo:rustc-env=TAURI_WS_PORT={}",
        env::var("TAURI_WS_PORT").unwrap()
    );
    println!(
        "cargo:rustc-env=TAURI_API_PORT={}",
        env::var("TAURI_API_PORT").unwrap()
    );
    println!(
        "cargo:rustc-env=TWITCH_CLIENT_ID={}",
        env::var("TWITCH_CLIENT_ID").unwrap()
    );
    println!(
        "cargo:rustc-env=TWITCH_CLIENT_SECRET={}",
        env::var("TWITCH_CLIENT_SECRET").unwrap()
    );

    tauri_build::build()
}

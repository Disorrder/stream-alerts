use dotenv::dotenv;
use std::env;

fn main() {
    // List of environment variables you want to embed
    // const ENV_VARS: [&str; 5] = [
    //     "TAURI_ORIGIN",
    //     "TAURI_WS_PORT",
    //     "TAURI_API_PORT",
    //     "TWITCH_CLIENT_ID",
    //     "TWITCH_CLIENT_SECRET",
    // ];

    // for var in ENV_VARS {
    //     if let Ok(value) = env::var(var) {
    //         println!("cargo:rustc-env={}={}", var, value);
    //     } else {
    //         println!("cargo:warning=Environment variable {} not set", var);
    //     }
    // }

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
        "cargo:rustc-env=TAURI_ORIGIN={}",
        env::var("TAURI_ORIGIN").unwrap()
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

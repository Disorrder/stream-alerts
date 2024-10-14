// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ws_server;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok(); // This will load the .env file

    tokio::spawn(async {
        let _ = ws_server::run().await;
    });

    return stream_alerts_lib::run();
}

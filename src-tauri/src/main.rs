// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;

// #[tokio::main]
fn main() {
    dotenv().ok(); // This will load the .env file
    return stream_alerts_lib::run();
}

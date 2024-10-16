// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::must_use_candidate,
    clippy::option_if_let_else,
    clippy::missing_errors_doc,
    clippy::future_not_send,
    clippy::implicit_hasher
)]

use dotenv::dotenv;

// #[tokio::main]
fn main() {
    dotenv().ok(); // This will load the .env file
    return stream_alerts_lib::run();
}

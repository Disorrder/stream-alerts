use sqlx::{sqlite::SqliteConnectOptions, Error, SqlitePool};
use std::path::Path;
use tauri::{App, Manager};

const DB_FILE_PATH: &str = "stream_alerts.db";

async fn connect(filename: impl AsRef<Path>) -> Result<SqlitePool, Error> {
    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true);

    SqlitePool::connect_with(options).await
}

pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle().clone();
    let app_data_dir = app_handle.path().app_data_dir().unwrap();
    let db_path = app_data_dir.join(DB_FILE_PATH);

    tauri::async_runtime::spawn(async move {
        println!("Starting database on {}", db_path.display());
        let db = match connect(db_path).await {
            Ok(db) => db,
            Err(e) => {
                eprintln!("Database error: {}", e);
                return;
            }
        };
        app_handle.manage(db);
    });

    Ok(())
}

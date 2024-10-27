use anyhow::Result;
use sled::Db;
use std::{path::PathBuf, sync::Arc};
use tauri::{App, Manager};

const STORE_PATH: &str = "stream_alerts.sled";

pub struct Store {
    pub db: Db,
}

impl Store {
    pub fn new(app_data_dir: PathBuf) -> Result<Self> {
        let db = sled::open(app_data_dir.join(STORE_PATH))?;
        Ok(Self { db })
    }

    pub fn get_db(&self) -> Db {
        self.db.clone()
    }
}

pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle();
    let app_data_dir = app_handle.path().app_data_dir().unwrap();
    let store = Store::new(app_data_dir)?;
    let store = Arc::new(store);
    app.manage(store);
    Ok(())
}

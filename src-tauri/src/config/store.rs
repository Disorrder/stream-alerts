use std::path::PathBuf;

use anyhow::Result;
use sled::Db;

const STORE_PATH: &str = "stream_alerts.sled";

pub struct Store {
    db: Db,
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

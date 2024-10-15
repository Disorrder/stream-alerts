use anyhow::{anyhow, Result};
use sled::Db;

use super::oauth2::TokenResponse;

const STORE_PATH: &str = "stream_alerts.sled";

pub struct TwitchStore {
    db: Db,
}

impl TwitchStore {
    pub fn new() -> Result<Self> {
        let db = sled::open(STORE_PATH)?;
        Ok(Self { db })
    }

    pub fn set_account(&self, account: &TokenResponse) -> Result<()> {
        let bytes = serde_json::to_vec(account)?;
        self.db.insert("twitch_account", bytes)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn get_account(&self) -> Result<Option<TokenResponse>> {
        match self.db.get("twitch_account")? {
            Some(bytes) => {
                let account = serde_json::from_slice(&bytes)
                    .map_err(|e| anyhow!("Failed to convert bytes to string: {}", e))?;
                Ok(Some(account))
            }
            None => Ok(None),
        }
    }

    pub fn delete_account(&self) -> Result<()> {
        self.db.remove("twitch_account")?;
        self.db.flush()?;
        Ok(())
    }
}

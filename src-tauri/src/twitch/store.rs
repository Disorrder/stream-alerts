use anyhow::{anyhow, Result};
use sled::Db;

use crate::config::store::Store;

use super::oauth2::TokenResponse;

pub struct TwitchStore {
    db: Db,
}

impl TwitchStore {
    pub fn new(store: Store) -> Result<Self> {
        let db = store.get_db();
        Ok(Self { db })
    }

    pub fn set_tokens(&self, tokens: &TokenResponse) -> Result<()> {
        let bytes = serde_json::to_vec(tokens)?;
        self.db.insert("twitch_tokens", bytes)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn get_tokens(&self) -> Result<Option<TokenResponse>> {
        match self.db.get("twitch_tokens")? {
            Some(bytes) => {
                let tokens = serde_json::from_slice(&bytes)
                    .map_err(|e| anyhow!("Failed to convert bytes to string: {}", e))?;
                Ok(Some(tokens))
            }
            None => Ok(None),
        }
    }

    pub fn delete_tokens(&self) -> Result<()> {
        self.db.remove("twitch_tokens")?;
        self.db.flush()?;
        Ok(())
    }
}

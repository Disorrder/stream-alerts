use anyhow::{anyhow, Result};
use sled::Db;
use std::sync::OnceLock;

const STORE_PATH: &str = "stream_alerts.sled";
static STORE: OnceLock<Db> = OnceLock::new();

pub fn get_store() -> Result<&'static Db> {
    let store = STORE.get_or_init(|| sled::open(STORE_PATH).unwrap());
    Ok(store)
}

pub fn set_access_token(token: &str) -> Result<()> {
    let store = get_store()?;
    store.insert("twitch.access_token", token.as_bytes())?;
    store.flush()?;
    Ok(())
}

pub fn get_access_token() -> Result<Option<String>> {
    let store = get_store()?;
    match store.get("twitch.access_token")? {
        Some(bytes) => {
            let token = String::from_utf8(bytes.to_vec())
                .map_err(|e| anyhow!("Failed to convert bytes to string: {}", e))?;
            Ok(Some(token))
        }
        None => Ok(None),
    }
}

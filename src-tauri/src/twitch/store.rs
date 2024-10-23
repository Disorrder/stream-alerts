use super::oauth2::TokenResponse;
use crate::config::store::Store;
use anyhow::Result;

pub trait TwitchStore {
    fn get_twitch_tokens(&self) -> Result<Option<TokenResponse>>;
    fn set_twitch_tokens(&self, tokens: &TokenResponse) -> Result<()>;
    fn delete_twitch_tokens(&self) -> Result<()>;
}

impl TwitchStore for Store {
    fn get_twitch_tokens(&self) -> Result<Option<TokenResponse>> {
        let tokens = self.db.get("twitch_tokens");
        match tokens {
            Ok(Some(bytes)) => {
                let tokens = serde_json::from_slice(&bytes)?;
                Ok(Some(tokens))
            }
            _ => Ok(None),
        }
    }

    fn set_twitch_tokens(&self, tokens: &TokenResponse) -> Result<()> {
        self.db.insert("twitch_tokens", serde_json::to_vec(tokens)?);
        self.db.flush()?;
        Ok(())
    }

    fn delete_twitch_tokens(&self) -> Result<()> {
        self.db.remove("twitch_tokens")?;
        self.db.flush()?;
        Ok(())
    }
}

impl Clone for Store {
    fn clone(&self) -> Self {
        Store {
            db: self.db.clone(),
        }
    }
}

use crate::twitch::oauth2::TwitchOAuthService;
use crate::twitch::store::TwitchStore;
use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, SocketRef};

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchCode {
    code: String,
}

pub struct WebsocketTwitchController {
    oauth_service: TwitchOAuthService,
    store: TwitchStore,
}

impl WebsocketTwitchController {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let oauth_service = TwitchOAuthService::new()?;
        let store = TwitchStore::new()?;
        Ok(Self {
            oauth_service,
            store,
        })
    }

    pub async fn auth_by_code(&self, s: SocketRef, data: Data<TwitchCode>) {
        let code = &data.code;

        let token_data = match self.oauth_service.exchange_code_for_token(code).await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to exchange code for token: {}", e);
                s.emit("twitch:auth_by_code:error", "Failed to authenticate")
                    .ok();
                return;
            }
        };

        println!("Token data: {:#?}", token_data);

        self.store.set_account(&token_data).unwrap();
        s.emit("twitch:auth_by_code:response", "ok").ok();
    }

    pub fn get_account(&self, s: SocketRef) {
        println!("[Twitch] Get Account");
        s.emit("twitch:patch_account", "TODO: Account Data").ok();
    }
}

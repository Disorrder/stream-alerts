use crate::store;
use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, SocketRef};

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchToken {
    access_token: String,
}

pub fn set_token(s: SocketRef, data: Data<TwitchToken>) {
    store::set_access_token(&data.access_token).unwrap();
    let token = store::get_access_token().unwrap();
    println!("[Rust] Got twitch access token: {:?}", token);
    s.emit("twitch:token_set", token).ok();
}

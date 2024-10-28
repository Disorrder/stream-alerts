use std::sync::Arc;
use tauri::{App, Manager};
use tokio_tungstenite::tungstenite;
use tungstenite::protocol::WebSocketConfig;
use twitch_api::{
    eventsub::{self, Event, EventsubWebsocketData, ReconnectPayload, SessionData, WelcomePayload},
    // twitch_oauth2::{self, TwitchToken, UserToken},
};
use url::Url;

use super::sdk::TwitchSDK;

pub struct EventsubClient {
    sdk: Arc<TwitchSDK>,
    connect_url: Url,
}

impl EventsubClient {
    pub fn new(sdk: Arc<TwitchSDK>) -> Self {
        Self {
            sdk,
            // connect_url: twitch_api::TWITCH_EVENTSUB_WEBSOCKET_URL.clone(),
            connect_url: "ws://127.0.0.1:8080/ws".parse().unwrap(),
        }
    }

    pub async fn connect(
        &self,
    ) -> Result<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        String,
    > {
        let config = WebSocketConfig {
            max_message_size: Some(64 << 20), // 64 MiB
            max_frame_size: Some(16 << 20),   // 16 MiB
            accept_unmasked_frames: false,
            ..WebSocketConfig::default()
        };
        let (socket, _) = tokio_tungstenite::connect_async_with_config(
            self.connect_url.as_str(),
            Some(config),
            false,
        )
        .await
        .map_err(|e| format!("Can't connect: {}", e))?;

        println!(
            "WebSocket handshake has been successfully completed on {}",
            self.connect_url.as_str()
        );

        Ok(socket)
    }

    pub async fn run(mut self) -> Result<(), String> {
        // Establish the stream
        let mut s = self
            .connect()
            .await
            .map_err(|e| format!("when establishing connection: {}", e))?;

        // Loop over the stream, processing messages as they come in.
        loop {
            tokio::select!(
            Some(msg) = futures::StreamExt::next(&mut s) => {
                let msg = match msg {
                    Err(tungstenite::Error::Protocol(
                        tungstenite::error::ProtocolError::ResetWithoutClosingHandshake,
                    )) => {
                        println!(
                            "connection was sent an unexpected frame or was reset, reestablishing it"
                        );
                        s = self
                            .connect()
                            .await
                            .map_err(|e| format!("when reestablishing connection: {}", e))?;
                        continue
                    }
                    _ => msg.map_err(|e| format!("when getting message: {}", e))?,
                };
                self.process_message(msg).await?
            })
        }
    }

    /// Process a message from the websocket
    pub async fn process_message(&mut self, msg: tungstenite::Message) -> Result<(), String> {
        match msg {
            tungstenite::Message::Text(s) => {
                println!("{s}");
                // Parse the message into a [twitch_api::eventsub::EventsubWebsocketData]
                match Event::parse_websocket(&s)
                    .map_err(|e| format!("when parsing message: {}", e))?
                {
                    EventsubWebsocketData::Welcome {
                        payload: WelcomePayload { session },
                        ..
                    }
                    | EventsubWebsocketData::Reconnect {
                        payload: ReconnectPayload { session },
                        ..
                    } => {
                        println!("Welcome message received: {session:?}");
                        self.process_welcome_message(session).await?;
                        Ok(())
                    }
                    // Here is where you would handle the events you want to listen to
                    EventsubWebsocketData::Notification {
                        metadata: _,
                        payload,
                    } => {
                        match payload {
                            Event::ChannelBanV1(eventsub::Payload { message, .. }) => {
                                println!("{message:?}");
                            }
                            Event::ChannelUnbanV1(eventsub::Payload { message, .. }) => {
                                println!("{message:?}");
                            }
                            _ => {}
                        }
                        Ok(())
                    }
                    EventsubWebsocketData::Revocation {
                        metadata,
                        payload: _,
                    } => {
                        println!("got revocation event: {metadata:?}");
                        Ok(())
                    }
                    EventsubWebsocketData::Keepalive {
                        metadata: _,
                        payload: _,
                    } => Ok(()),
                    _ => Ok(()),
                }
            }
            tungstenite::Message::Close(_) => todo!(),
            _ => Ok(()),
        }
    }

    pub async fn process_welcome_message(&mut self, data: SessionData<'_>) -> Result<(), String> {
        if let Some(url) = data.reconnect_url {
            self.connect_url = url
                .parse()
                .map_err(|e| format!("when parsing reconnect url: {}", e))?;
        }

        let sdk = self.sdk.clone();
        let client = sdk.get_client();
        let token = sdk.get_or_create_token().await?;

        if token.is_none() {
            return Err("No token".to_string());
        }
        let token = token.unwrap();
        let user_id = token.clone().user_id;

        let transport = eventsub::Transport::websocket(data.id.clone());
        client
            .create_eventsub_subscription(
                eventsub::channel::ChannelSubscribeV1::broadcaster_user_id(user_id.clone()),
                transport.clone(),
                &token.clone(),
            )
            .await
            .map_err(|e| format!("when new subscriber appears: {}", e))?;
        client
            .create_eventsub_subscription(
                eventsub::channel::ChannelSubscriptionMessageV1::broadcaster_user_id(
                    user_id.clone(),
                ),
                transport.clone(),
                &token.clone(),
            )
            .await
            .map_err(|e| format!("when resub message appears: {}", e))?;
        client
            .create_eventsub_subscription(
                eventsub::channel::ChannelSubscriptionGiftV1::broadcaster_user_id(user_id.clone()),
                transport.clone(),
                &token.clone(),
            )
            .await
            .map_err(|e| format!("when subscription gift appears: {}", e))?;
        client
            .create_eventsub_subscription(
                eventsub::channel::ChannelFollowV2::new(user_id.clone(), user_id.clone()),
                transport.clone(),
                &token.clone(),
            )
            .await
            .map_err(|e| format!("when follower appears: {}", e))?;

        println!("listening to ban and unbans");
        Ok(())
    }
}

pub fn setup(app: &App) -> Result<(), String> {
    let sdk = app.state::<Arc<TwitchSDK>>().inner().clone();

    let eventsub = EventsubClient::new(sdk);
    tauri::async_runtime::spawn(async move {
        println!("Starting eventsub server");
        eventsub.run().await.unwrap();
    });

    Ok(())
}

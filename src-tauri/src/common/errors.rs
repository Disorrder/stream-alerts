use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum HttpError {
    JsonDecodeError(serde_json::Error),
    StoreError(sled::Error),
    ReqwestError(reqwest::Error),
    TwitchError(reqwest::Error),
    AuthError(String),
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let err = match self {
            /* Libraries */
            Self::JsonDecodeError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("JsonDecode: {}", e.to_string()),
            ),
            Self::StoreError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Store: {}", e.to_string()),
            ),
            Self::ReqwestError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Reqwest: {}", e.to_string()),
            ),
            /* External services */
            Self::TwitchError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Twitch: {}", e.to_string()),
            ),
            Self::AuthError(message) => (StatusCode::UNAUTHORIZED, format!("Auth: {}", message)),
        };

        // it's often easiest to implement `IntoResponse` by calling other implementations
        err.into_response()
    }
}

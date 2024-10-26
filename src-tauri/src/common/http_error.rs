use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum HttpError {
    Unauthorized(String),
    LibSerdeFailed(serde_json::Error),
    LibSledFailed(sled::Error),
    LibReqwestFailed(reqwest::Error),
    ExternalServiceFailed(String),
    TwitchFailed(reqwest::Error),
    StoreFailed(String),
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let err = match self {
            Self::Unauthorized(message) => (
                StatusCode::UNAUTHORIZED,
                format!("Unauthorized: {}", message),
            ),
            /* Libraries */
            Self::LibSerdeFailed(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Serde: {}", e.to_string()),
            ),
            Self::LibSledFailed(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Sled: {}", e.to_string()),
            ),
            Self::LibReqwestFailed(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Reqwest: {}", e.to_string()),
            ),
            /* External services */
            Self::ExternalServiceFailed(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("External service: {}", e),
            ),
            Self::TwitchFailed(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Twitch: {}", e.to_string()),
            ),
            Self::StoreFailed(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Store: {}", message),
            ),
        };

        // it's often easiest to implement `IntoResponse` by calling other implementations
        err.into_response()
    }
}

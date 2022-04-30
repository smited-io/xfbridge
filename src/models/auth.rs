use serde::Serialize;
use thiserror::Error;

/// The request for interacting with the XenForo /auth endpoint.
#[derive(Serialize)]
pub struct Auth {
    pub login: String,
    pub password: String,
}

/// The request for interacting with the XenForo /auth/from-session endpoint.
#[derive(Serialize)]
pub struct AuthFromSession {
    pub session_id: String,
}

/// The error type for an invalid AuthFromSession request.
#[derive(Error, Debug)]
pub enum AuthFromSessionError {
    #[error("invalid session token")]
    InvalidSession,
}

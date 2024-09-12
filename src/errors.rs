use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum RocketAPIError {
    BadResponse(serde_json::Value),
    NotFound(serde_json::Value),
    RequestError(reqwest::Error),
}

impl fmt::Display for RocketAPIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RocketAPIError::BadResponse(msg) => write!(f, "BadResponse: {}", msg),
            RocketAPIError::NotFound(msg) => write!(f, "NotFound: {}", msg),
            RocketAPIError::RequestError(msg) => write!(f, "RequestError: {}", msg),
        }
    }
}

impl Error for RocketAPIError {}

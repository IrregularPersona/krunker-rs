use reqwest::StatusCode;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Api {
        status: StatusCode,
        body: String,
    },
    Decode {
        message: String,
        body: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Http(e) => write!(f, "HTTP error: {}", e),
            Error::Api { status, body } => write!(f, "API error ({}): {}", status, body),
            Error::Decode { message, body } => write!(f, "Decode error: {} | Body: {}", message, body),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

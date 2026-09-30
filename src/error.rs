use reqwest::StatusCode;
use std::fmt;

/// Failures reported by client configuration or an API call.
#[derive(Debug)]
pub enum Error {
    /// A request, connection, timeout, TLS, or response body transport failure.
    Http(reqwest::Error),
    /// A non-success HTTP status other than 429, with the API message or raw text.
    Api {
        /// HTTP response status.
        status: StatusCode,
        /// JSON `error`, plain response text, or the status reason for an empty body.
        message: String,
    },
    /// HTTP 429 after any configured retries have been exhausted.
    RateLimit {
        /// Recommended delay in seconds before making another attempt.
        retry_after: u64,
    },
    /// A success response could not be decoded as the expected type or complete JSON.
    Decode {
        /// Parser message, including a line and column when available.
        message: String,
        /// Complete response body, available for explicit inspection.
        body: String,
        /// JSON field path; `None` when trailing data caused the failure.
        field: Option<String>,
    },
    /// Invalid local configuration or parameters; no request was sent.
    InvalidInput {
        /// Configuration or endpoint parameter that failed validation.
        parameter: &'static str,
        /// Description of the accepted value, without including an API key.
        message: String,
    },
}

impl Error {
    pub(crate) fn invalid_input(parameter: &'static str, message: impl Into<String>) -> Self {
        Self::InvalidInput {
            parameter,
            message: message.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(error) => write!(f, "HTTP error: {error}"),
            Self::Api { status, message } => {
                let preview: String = message.chars().take(512).collect();
                let suffix = if preview.len() < message.len() {
                    "…"
                } else {
                    ""
                };
                write!(f, "API error ({status}): {preview}{suffix}")
            }
            Self::RateLimit { retry_after } => {
                write!(f, "Rate limit exceeded. Retry after {retry_after} seconds")
            }
            Self::Decode { message, field, .. } => {
                write!(
                    f,
                    "Decode error (field: {}): {message}",
                    field.as_deref().unwrap_or("response")
                )
            }
            Self::InvalidInput { parameter, message } => {
                write!(f, "Invalid {parameter}: {message}")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Http(error) => Some(error),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error)
    }
}

/// A result returned by client configuration and endpoint methods.
pub type Result<T> = std::result::Result<T, Error>;

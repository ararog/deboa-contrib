use thiserror::Error;

/// Deboa extras errors
#[derive(Debug, Clone, Error, PartialEq)]
pub enum DeboaExtrasError {
    /// SSE error
    #[error("SSE error: {0}")]
    SSE(#[from] SSEError),
}

/// SSE errors
#[derive(Debug, Clone, Error, PartialEq)]
pub enum SSEError {
    /// Failed to receive event
    #[error("Failed to receive event: {message}")]
    ReceiveEvent {
        /// The error message
        message: String,
    },
}

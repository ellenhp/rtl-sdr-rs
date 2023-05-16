use thiserror::Error;

#[derive(Debug, Error)]
pub enum RtlsdrError {
    #[error("RtlSdr error {0}")]
    RtlsdrErr(String),
    #[error("USB error")]
    Usb(#[from] rusb::Error),
}

/// A result of a function that may return a `Error`.
pub type Result<T> = std::result::Result<T, RtlsdrError>;


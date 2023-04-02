//! Result and Error types

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The seed length is invalid
    #[error("The seed length is invalid {0}")]
    InvalidSeedLength(usize),
    #[error(transparent)]
    Bip32(#[from] bip32::Error),
    #[error(transparent)]
    Bip39(#[from] bip39::Error),
}

/// Result type for this crate
pub type Result<T> = std::result::Result<T, Error>;

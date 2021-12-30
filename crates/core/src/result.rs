//! turbo results

/// turbo errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown error")]
    UnKnown,
    #[error("invalid ethereum endpoint")]
    InvalidEndpoint,
    #[error(transparent)]
    Web3(#[from] web3::error::Error),
}

/// turbo result
pub type Result<T> = core::result::Result<T, Error>;

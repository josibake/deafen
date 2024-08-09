use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Storage error: {0}")]
    Storage(#[from] anyhow::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Compute error: {0}")]
    Compute(String),
    #[error("Client not found")]
    ClientNotFound,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Crypto error: {0}")]
    Crypto(#[from] secp256k1::Error),
    #[error("Silent payments error: {0}")]
    SilentPayments(#[from] silentpayments::Error),
    #[error("Secp256k1 error: {0}")]
    Secp256k1(#[from] silentpayments::secp256k1::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl warp::reject::Reject for Error {}


use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    Pkcs1Error(#[from] rsa::pkcs1::Error),

    #[error(transparent)]
    Pkcs8Error(#[from] rsa::pkcs8::Error),

    #[error(transparent)]
    RsaError(#[from] rsa::errors::Error),

    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}

use thiserror::Error as ThisError;
#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    PoolError(#[from] r2d2::Error),

    #[error(transparent)]
    ConnError(#[from] redis::RedisError),

    #[error("配置错误")]
    ConfigError,
}

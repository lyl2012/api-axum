use axum::response::IntoResponse;
use thiserror::Error as ThisError;

use super::response::AppResponse;
/// 业务错误定义
#[derive(Debug, ThisError, Clone, Copy)]
#[repr(C)]
pub enum BusinessError {
    //未登录
    #[error("请先登录~")]
    NoLogin = 100,
}

/// 系统自定义错误
#[derive(ThisError, Debug)]
pub enum AppError {
    //sql错误
    //#[error(transparent)]
    //SqlError(#[from] sqlx::Error),

    //业务错误信息
    #[error("{0}")]
    BusinessError(#[source] BusinessError),

    #[error(transparent)]
    RedisError(#[from] itoo_redis::Error),

    #[error(transparent)]
    UtilsError(#[from] itoo_utils::Error),
    // #[error(transparent)]
    // RedisErrors(#[from] itoo_redis::RedisError),
}

impl From<BusinessError> for AppError {
    fn from(b: BusinessError) -> Self {
        Self::BusinessError(b)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let data: AppResponse<&str> = match self {
            Self::BusinessError(e) => {
                //业务错误
                AppResponse::err(e as i32, e.to_string().clone())
            }
            Self::RedisError(e) => AppResponse::err(0, e.to_string()),
            Self::UtilsError(e) => AppResponse::err(0, e.to_string()),
        };
        axum::Json(data).into_response()
    }
}

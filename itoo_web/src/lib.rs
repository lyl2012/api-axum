pub mod api;

mod config;
mod init;

//路由
pub mod routers;

use crate::config::{error::AppError, response::AppResponse};
use log::info;
use state::Container;

use crate::{
    config::log::init_log,
    init::{config::init_config, redis::init_redis},
};

type Result<T> = std::result::Result<T, AppError>;
pub type AppResult<T> = Result<AppResponse<T>>;

/**
 * 整个项目的上下文管理
 * 包括：
 *      配置文件
 *      数据库连接池
 */
pub static CONTEXT_MANAGER: Container![Send + Sync] = <Container![Send + Sync]>::new();

pub async fn init_context() {
    //加载配置
    init_config().await;

    //日志
    init_log();

    info!("ConfigContext init complete");
    //初始化redis
    init_redis();
    info!("RedisContext init complete");
}

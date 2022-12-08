use itoo_config::ApplicationConfig;
use tokio::fs::read_to_string;

use crate::CONTEXT_MANAGER;
//初始化配置信息-写入到上下文管理器
pub async fn init_config() {
    let content = read_to_string("application.toml").await.unwrap();
    let config = ApplicationConfig::new(&content);

    CONTEXT_MANAGER.set::<ApplicationConfig>(config);
}

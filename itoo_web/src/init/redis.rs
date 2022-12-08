use itoo_config::ApplicationConfig;
use itoo_redis::{init_redis_pool, Redis};
use log::info;

use crate::CONTEXT_MANAGER;
pub fn init_redis() {
    let config = CONTEXT_MANAGER.get::<ApplicationConfig>();
    let redis_pool = init_redis_pool(&config.redis);

    let redis = Redis::new(redis_pool, 3);
    info!(
        "init redis pool success! {}:{}",
        &config.redis.host, &config.redis.port
    );
    CONTEXT_MANAGER.set::<Redis>(redis);
}

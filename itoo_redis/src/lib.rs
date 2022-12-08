mod error;
mod redis;

extern crate redis as redis_other;
pub use error::Error;
use itoo_config::RedisConfig;

pub use crate::redis::Redis;
pub use crate::redis::RedisPool;

//初始化redis连接池
pub fn init_redis_pool(config: &RedisConfig) -> RedisPool {
    let url = format!("redis://{}:{}", config.host, config.port);
    let client = redis_other::Client::open(url).expect("redis链接失败");
    let pool = r2d2::Pool::builder()
        .max_size(10)
        .min_idle(Some(5))
        .build(client)
        .expect("redis链接池处理失败");

    pool
}

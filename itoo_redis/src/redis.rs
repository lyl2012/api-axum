use std::time::Duration;

use crate::Error;
use redis::{Client, Commands, ConnectionLike};
pub type RedisPool = r2d2::Pool<Client>;

#[derive(Debug, Clone)]
pub struct Redis {
    pool: RedisPool,
    //获取链接，超时时间
    timeout: u8,
}

impl Redis {
    pub fn new(pool: RedisPool, timeout: u8) -> Self {
        Self { pool, timeout }
    }

    //从连接池获取连接
    fn connection(&self) -> Result<r2d2::PooledConnection<Client>, Error> {
        let conn = self
            .pool
            .get_timeout(Duration::from_secs(self.timeout as u64))?;
        Ok(conn)
    }
    //redis get
    pub fn get<K, V>(&self, key: K) -> Result<V, Error>
    where
        K: redis::ToRedisArgs,
        V: redis::FromRedisValue,
    {
        let mut conn = self.connection()?;
        let v = conn.get(key)?;
        Ok(v)
    }
    //切换db
    pub fn select(&self, database: u8) -> Result<redis::Value, Error> {
        let mut con = self.connection()?;
        let r = con.req_command(redis::cmd("SELECT").arg(database))?;
        Ok(r)
    }

    pub fn set<K, V, RV>(&self, key: K, value: V) -> Result<RV, Error>
    where
        K: redis::ToRedisArgs,
        V: redis::ToRedisArgs,
        RV: redis::FromRedisValue,
    {
        let mut conn = self.connection()?;
        let r = conn.set(key, value)?;
        Ok(r)
    }
}

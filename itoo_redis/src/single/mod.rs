use std::time::Duration;

use r2d2::{Pool, PooledConnection};
use redis::{Connection, ConnectionLike};

use crate::{config::SingleConfig, Error};

/// r2d2 对redis实现连接池功能
pub struct ConnectionManager {
    client: redis::Client,
}

impl r2d2::ManageConnection for ConnectionManager {
    type Connection = Connection;
    type Error = Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let conn = self.client.get_connection()?;

        Ok(conn)
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        redis::cmd("PING").query(conn)?;
        Ok(())
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        !conn.is_open()
    }
}

impl ConnectionManager {
    fn new(config: &SingleConfig) -> Result<Self, Error> {
        let hosts = format!("redis://{}:{}", config.host.clone(), config.port.clone());
        let client = redis::Client::open(hosts)?;
        Ok(Self { client })
    }
}

///单点模式 连接池
pub struct RedisPool {
    pub pool: Pool<ConnectionManager>,
}

impl RedisPool {
    pub fn new(config: &SingleConfig) -> Result<Self, Error> {
        let manager = ConnectionManager::new(config)?;

        let pool = r2d2::Pool::builder()
            .max_size(10)
            .min_idle(Some(10))
            .connection_timeout(Duration::from_secs(5))
            .build(manager)?;
        Ok(Self { pool })
    }

    pub fn connection(&self) -> Result<PooledConnection<ConnectionManager>, Error> {
        let conn = self.pool.get()?;
        Ok(conn)
    }
}

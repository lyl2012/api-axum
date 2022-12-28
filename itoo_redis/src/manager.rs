// use std::{borrow::BorrowMut, time::Duration};

// use r2d2::Pool;
// use redis::{
//     cluster::{ClusterClient, ClusterClientBuilder, ClusterConnection},
//     Client, Commands, Connection, ConnectionLike,
// };

// use crate::{
//     config::{Config, DeployMode},
//     Error,
// };

// ///redis client
// pub(crate) enum RedisClient {
//     //单点模式
//     Single(Client),
//     //集群模式
//     Cluster(ClusterClient),
// }

// impl RedisClient {
//     pub(crate) fn new(config: &Config) -> Result<Self, Error> {
//         //处理redis连接
//         let conf = config.host.as_str().split(",").collect::<Vec<&str>>();
//         if conf.is_empty() {
//             return Err(Error::ConfigError);
//         }
//         let hosts = conf
//             .iter()
//             .map(|h| format!("redis://{}", *h))
//             .collect::<Vec<String>>();

//         return match config.deploy {
//             DeployMode::Single => {
//                 let sc = Client::open(hosts[0].clone())?;
//                 Ok(Self::Single(sc))
//             }
//             DeployMode::Cluster => {
//                 let mut cc = ClusterClientBuilder::new(hosts);
//                 if !config.password.is_empty() {
//                     cc = cc.password(config.password.clone());
//                 }
//                 let cl = cc.open()?;
//                 Ok(Self::Cluster(cl))
//             }
//         };
//     }

//     pub(crate) fn get_connection(&self) -> Result<RedisConnection, Error> {
//         return match self {
//             Self::Single(sc) => {
//                 let conn = sc.get_connection()?;
//                 Ok(RedisConnection::Single(conn))
//             }
//             Self::Cluster(cc) => {
//                 let conn = cc.get_connection()?;
//                 conn.get_db()
//                 Ok(RedisConnection::Cluster(conn))
//             }
//         };
//     }
// }

// /// redis connection
// pub enum RedisConnection {
//     Single(Connection),
//     Cluster(ClusterConnection),
// }

// impl RedisConnection {
//     fn is_open(&self) -> bool {
//         match self {
//             Self::Single(sc) => sc.is_open(),
//             Self::Cluster(cc) => cc.is_open(),
//         }
//     }

//     fn connection(&self) -> Box<&dyn ConnectionLike> {
//         return match self {
//             Self::Single(sc) => Box::new(sc),
//             Self::Cluster(cc) => Box::new(cc),
//         };
//     }

//     // fn query<T: FromRedisValue>(&mut self, cmd: &Cmd) -> Result<T, Error> {
//     //     return match self {
//     //         Self::Single(sc) => match sc.req_command(cmd) {
//     //             Ok(val) => from_redis_value(&val)?,
//     //             Err(e) => Error::ConnError(()),
//     //         },
//     //         Self::Cluster(cc) => match cc.req_command(cmd) {
//     //             Ok(val) => from_redis_value(&val),
//     //             Err(e) => Err(e),
//     //         },
//     //     };
//     // }
// }

// pub struct RedisConnectionManager {
//     pub(crate) client: RedisClient,
// }

// impl r2d2::ManageConnection for RedisConnectionManager {
//     type Connection = RedisConnection;
//     type Error = Error;

//     fn connect(&self) -> Result<Self::Connection, Self::Error> {
//         let conn = self.client.get_connection()?;
//         Ok(conn)
//     }

//     fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
//         match conn {
//             RedisConnection::Single(sc) => redis::cmd("PING").query(sc)?,
//             RedisConnection::Cluster(cc) => redis::cmd("PING").query(cc)?,
//         }
//         Ok(())
//     }

//     fn has_broken(&self, conn: &mut Self::Connection) -> bool {
//         !conn.is_open()
//     }
// }

// pub struct RedisPool {
//     pub pool: Pool<RedisConnectionManager>,
// }

// impl RedisPool {
//     pub fn new(config: &Config) -> Result<Self, Error> {
//         let client = RedisClient::new(config)?;
//         let manager = RedisConnectionManager { client };

//         let pool = r2d2::Pool::builder()
//             .max_size(10)
//             .min_idle(Some(10))
//             .connection_timeout(Duration::from_secs(5))
//             .build(manager)?;

//         Ok(Self { pool })
//     }

//     pub fn connection(&self) -> Result<Box<&dyn ConnectionLike>, Error> {
//         let mut conn = self.pool.get()?;

//         Ok(conn.)
//     }
// }

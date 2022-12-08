use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
///#[getset(get_mut = "pub", get = "pub", set = "pub")]
pub struct ServerConfig {
    //当前服务器地址
    pub address: String,
    //端口
    pub port: String,
    //服务器名
    pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MysqlConfig {
    pub master: DbConfig,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DbConfig {
    //地址
    pub host: String,
    //端口
    pub port: String,
    //用户名
    pub username: String,
    //密码
    pub password: String,
    //数据库
    pub database: String,
}
///日志相关配置
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LogConfig {
    ///日志目录 "target/logs/"
    pub dir: String,
    /// "100MB" 日志分割尺寸-单位KB,MB,GB
    pub size: String,
    ///日志打包格式可选“”（空-不压缩）“gzip”（gz压缩包）“zip”（zip压缩包）“lz4”（lz4压缩包（非常快
    pub pack_compress: String,
    ///日志滚动配置   保留全部:All,按时间保留:KeepTime(Duration),按版本保留:KeepNum(i64)
    pub rolling_type: String,
    ///日志等级
    pub level: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RedisConfig {
    //host
    pub host: String,
    //端口
    pub port: u32,
    //密码
    pub password: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ApplicationConfig {
    pub server: ServerConfig,
    pub mysql: MysqlConfig,
    pub log: LogConfig,
    pub redis: RedisConfig,
}

impl ApplicationConfig {
    pub fn new(data: &str) -> Self {
        let config = match toml::from_str(data) {
            Ok(c) => c,
            Err(e) => panic!("{}", e),
        };
        config
    }
}

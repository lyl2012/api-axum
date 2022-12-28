use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SingleConfig {
    //host 格式：127.0.0.1
    pub host: String,
    //端口
    pub port: String,

    //密码
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub enum DeployMode {
    //单节点
    Single,
    //集群
    Cluster,
}

use axum::{routing::get, Router};
use itoo_redis::Redis;

use crate::config::response::AppResponse;
use crate::{AppResult, CONTEXT_MANAGER};

pub(crate) async fn user_test() -> AppResult<String> {
    let redis = CONTEXT_MANAGER.get::<Redis>();
    redis.set("test", "this is a test")?;
    let cache: Option<String> = match redis.get("testa") {
        Ok(s) => s,
        Err(_) => None,
    };
    if cache.is_none() {
        //不存在
        return Ok(AppResponse::ok("不存在".to_string()));
    }
    Ok(AppResponse::ok(cache.unwrap()))
}

//路由配置
pub(crate) fn init_router() -> Router {
    Router::new().route("/test", get(user_test))
}

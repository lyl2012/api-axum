use crate::config::response::AppResponse;
use crate::{AppResult, CONTEXT_MANAGER};
use axum::{routing::get, Router};
use itoo_config::ApplicationConfig;
use itoo_redis::Commands;
use itoo_redis::RedisPool;

pub(crate) async fn user_test() -> AppResult<String> {
    //return demo_sign();
    let redis = CONTEXT_MANAGER.get::<RedisPool>();
    let mut conn = redis.connection()?;
    let cache: Option<String> = conn.get("testa")?;

    Ok(AppResponse::ok(cache))
}

fn demo_sign() -> AppResult<String> {
    let conf = CONTEXT_MANAGER.get::<ApplicationConfig>();
    let sign = itoo_utils::generate_sign_str(Some(conf))?;
    Ok(AppResponse::ok(sign))
}

//路由配置
pub(crate) fn init_router() -> Router {
    Router::new().route("/test", get(user_test))
}

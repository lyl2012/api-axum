use axum::Router;

use crate::api;

pub fn routers() -> Router {
    need_auth_routers()
}

//需要权限认证
fn need_auth_routers() -> Router {
    Router::new().merge(api::user::init_router())
}

//不需要权限认证

//@认证和未认证 都可以访问，但数据不一样

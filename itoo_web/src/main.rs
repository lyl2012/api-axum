use axum::{Router, Server};
use itoo_config::ApplicationConfig;
use itoo_web::{init_context, routers::api, CONTEXT_MANAGER};
#[tokio::main]
async fn main() {
    //初始化上下文环境
    init_context().await;

    let itoo_config = CONTEXT_MANAGER.get::<ApplicationConfig>();

    let server = format!(
        "{}:{}",
        &itoo_config.server.address, &itoo_config.server.port
    );
    let app = Router::new().nest("/api", api::routers());
    Server::bind(&server.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

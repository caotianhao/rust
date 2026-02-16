//! 主入口：加载配置、初始化日志与 DB、启动 HTTP 服务并支持优雅关闭

use actix_web::{middleware, web, App, HttpServer};
use tracing_subscriber::EnvFilter;

use testr::{configure, create_pool, run_migrations, Config};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env_or_default();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.log_level)),
        )
        .init();

    tracing::info!("binding on {}", config.bind);
    let pool = create_pool(&config.database_url)
        .await
        .expect("failed to create database pool");
    run_migrations(&pool).await.expect("migrations failed");

    let bind = config.bind;
    let pool = web::Data::new(pool);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::scope("/api").configure(configure))
    })
    .bind(bind)?
    .workers(2);

    let server = server.shutdown_timeout(30);
    let srv = server.run();
    let handle = srv.handle();
    tokio::spawn(async move {
        let _ = tokio::signal::ctrl_c().await;
        tracing::info!("shutting down ...");
        handle.stop(true).await;
    });

    tracing::info!("listening on http://{}", bind);
    srv.await
}

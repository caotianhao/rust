use actix_web::{App, HttpServer, Responder, delete, get, middleware::Logger, post, put};
use tracing_subscriber::EnvFilter;

#[get("/get")]
async fn get_handler() -> impl Responder {
    "GET Hello"
}

#[post("/post")]
async fn post_handler() -> impl Responder {
    "POST Hello"
}

#[put("/put")]
async fn put_handler() -> impl Responder {
    "PUT Hello"
}

#[delete("/delete")]
async fn delete_handler() -> impl Responder {
    "DELETE Hello"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(get_handler)
            .service(post_handler)
            .service(put_handler)
            .service(delete_handler)
    })
    .bind("localhost:3000")?
    .run()
    .await
}

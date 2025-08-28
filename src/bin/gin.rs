use actix_web::{App, HttpServer, Responder, delete, get, middleware::Logger, post, put};
use env_logger::Env;

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
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

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

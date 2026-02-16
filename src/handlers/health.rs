//! 健康检查与就绪探针

use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

#[get("/ready")]
pub async fn ready() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "ready": true }))
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use super::health;

    #[actix_web::test]
    async fn health_returns_ok() {
        let app = test::init_service(App::new().service(health)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}

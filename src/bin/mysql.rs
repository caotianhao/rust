use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use chrono::Utc;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::mysql::MySqlPoolOptions;
use std::env;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct User {
    id: Option<i32>,
    name: String,
    age: i32,
}

async fn init_db(pool: &sqlx::MySqlPool) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id bigint(20) NOT NULL AUTO_INCREMENT COMMENT '自增ID',
            name varchar(50) NOT NULL DEFAULT '' COMMENT '用户名称',
            age int(11) NOT NULL DEFAULT '0' COMMENT '用户年龄',
            ctime bigint(20) NOT NULL DEFAULT '0' COMMENT '创建时间',
            utime bigint(20) NOT NULL DEFAULT '0' COMMENT '更新时间',
            PRIMARY KEY (id)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户表';
        "#,
    )
    .execute(pool)
    .await
    .unwrap();
}

async fn create_user(pool: web::Data<sqlx::MySqlPool>, user: web::Json<User>) -> impl Responder {
    let _ = sqlx::query("INSERT INTO users (name, age, ctime, utime) VALUES (?, ?, ?, ?)")
        .bind(&user.name)
        .bind(user.age)
        .bind(Utc::now().timestamp())
        .bind(Utc::now().timestamp())
        .execute(pool.get_ref())
        .await;
    HttpResponse::Ok().body("User created")
}

async fn get_users(pool: web::Data<sqlx::MySqlPool>) -> impl Responder {
    let users: Vec<User> =
        sqlx::query_as::<_, User>("SELECT id, name, age, ctime, utime FROM users")
            .fetch_all(pool.get_ref())
            .await
            .unwrap_or_default();
    HttpResponse::Ok().json(users)
}

async fn update_user(pool: web::Data<sqlx::MySqlPool>, user: web::Json<User>) -> impl Responder {
    let _ = sqlx::query("UPDATE users SET name = ?, age = ?, utime = ? WHERE id = ?")
        .bind(&user.name)
        .bind(user.age)
        .bind(Utc::now().timestamp())
        .bind(user.id)
        .execute(pool.get_ref())
        .await;
    HttpResponse::Ok().body("User updated")
}

async fn delete_user(pool: web::Data<sqlx::MySqlPool>, id: web::Path<i32>) -> impl Responder {
    let _ = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(*id)
        .execute(pool.get_ref())
        .await;
    HttpResponse::Ok().body("User deleted")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    init_db(&pool).await;

    println!("mysql bin: HTTP server listening on http://127.0.0.1:8080");
    println!("  GET  /users     -> list users");
    println!("  POST /users     -> create user (JSON: name, age)");
    println!("  PUT  /users     -> update user (JSON: id, name, age)");
    println!("  DELETE /users/{{id}} -> delete user");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(create_user))
            .route("/users", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

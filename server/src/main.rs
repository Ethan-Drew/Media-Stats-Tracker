use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;
use sqlx::PgPool;

#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/json")]
async fn json_response() -> impl Responder {
    let response = ApiResponse {
        message: "Hello from JSON endpoint!".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/db_example")]
async fn db_example(pool: web::Data<PgPool>) -> impl Responder {
    let row: (i32,) = sqlx::query_as("SELECT 1 AS value")
        .fetch_one(pool.get_ref())
        .await
        .expect("Failed to execute query");
    
    HttpResponse::Ok().json(serde_json::json!({ "value": row.0 }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Cors::permissive()) // Use permissive CORS for now
            .service(hello_world)
            .service(json_response)
            .service(db_example)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

// src/main.rs
use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use serde::Serialize;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello_world) // Register "Hello, World" endpoint
            .service(json_response) // Register JSON endpoint
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

// src/routes/home.rs

use actix_web::{get, HttpResponse, Responder, web};
use sqlx::PgPool;
use serde::Serialize;

// Define a struct for JSON responses
#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

// Route handler for a basic endpoint
#[get("/")]
pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello, World from home!")
}

// Route handler for JSON response
#[get("/json")]
pub async fn json_response() -> impl Responder {
    let response = ApiResponse {
        message: "Hello from JSON endpoint in home!".to_string(),
    };
    HttpResponse::Ok().json(response)
}

// Database example endpoint
#[get("/db_example")]
pub async fn db_example(pool: web::Data<PgPool>) -> impl Responder {
    // Run the query, expecting a result with an integer column labeled "value"
    match sqlx::query!("SELECT 1 AS value")
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(row) => HttpResponse::Ok().json(serde_json::json!({ "value": row.value })),
        Err(err) => {
            eprintln!("Database query failed: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Function to initialize routes
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_world)
       .service(json_response)
       .service(db_example); // Register the db_example endpoint
}

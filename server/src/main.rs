// src/main.rs
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;
use sqlx::PgPool; // Added import for PgPool

#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

// Basic "Hello, World!" endpoint
#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

// JSON response endpoint
#[get("/json")]
async fn json_response() -> impl Responder {
    let response = ApiResponse {
        message: "Hello from JSON endpoint!".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/db_example")]
async fn db_example(pool: web::Data<PgPool>) -> impl Responder {
    // Run the query, expecting a result with an integer column labeled "value"
    let row: (i32,) = sqlx::query_as("SELECT 1 AS value")
        .fetch_one(pool.get_ref())
        .await
        .expect("Failed to execute query");

    // Use JSON serialization to return the result as {"value": 1}
    HttpResponse::Ok().json(serde_json::json!({ "value": row.0 }))
}

// Main function to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables from `.env`
    
    // Retrieve the database URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Establish the database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5) // Customize max connections if needed
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    // Start the Actix web server and add the database pool to App data
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Add pool to app state
            .service(hello_world)    // Register your services
            .service(json_response)
            .service(db_example)     // Register the db_example endpoint
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

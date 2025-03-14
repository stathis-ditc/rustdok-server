//! # RustDok Server
//! 
//! RustDok is an S3-compatible object storage server built with Rust.
//! This server provides a RESTful API for managing buckets and objects
//! in an S3-compatible storage backend.
//!
//! The server uses Actix Web for the HTTP server implementation and
//! aws-sdk-s3 for interacting with S3-compatible storage services.

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use actix_cors::Cors;
use actix_web::http::header;
use std::env;
use std::sync::Arc;

mod models;
mod rdlib;
mod api;

#[cfg(test)]
mod tests;

/// The main entry point for the RustDok server.
/// 
/// This function initializes the environment, sets up logging,
/// configures CORS, and starts the HTTP server with the API routes.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Initialize env_logger with RUST_LOG environment variable
    // If RUST_LOG is not set, default to "info" level
    if env::var("RUST_LOG").is_err() {
        unsafe {
            env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();

    // Initialize the S3 client at application startup
    // This must be done before any S3Service instances are created
    let _s3_client = rdlib::s3::service::init_s3_client().await;
    
    // Create an S3Service instance to be shared across all workers
    let s3_service = Arc::new(rdlib::s3::service::S3Service::new().await);
    
    HttpServer::new(move || {
        // Create a new Cors instance for each worker
        let cors = Cors::default()
            .allowed_origin(&env::var("RUSTDOK_WEBUI_URL").unwrap_or_else(|_| "http://localhost:3000".to_string()))
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            // Share the S3Service instance with all routes
            .app_data(web::Data::new(s3_service.clone()))
            // Health check endpoints
            .service(api::health::liveness)
            .service(api::health::readiness)
            // API v1 routes
            .service(api::config::configure_api_v1())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
} 
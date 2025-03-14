//! # Health Check API
//! 
//! This module provides health check endpoints for the RustDok server.
//! These endpoints are used for liveness and readiness probes in container environments.

use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

/// Health check endpoint for liveness probe
///
/// This endpoint returns a 200 OK response if the server is running.
/// It's used by container orchestration systems to determine if the
/// application is alive.
#[get("/healthz")]
pub async fn liveness() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "Server is running"
    }))
}

/// Health check endpoint for readiness probe
///
/// This endpoint returns a 200 OK response if the server is ready to accept requests.
/// It's used by container orchestration systems to determine if the
/// application is ready to receive traffic.
#[get("/readyz")]
pub async fn readiness() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "Server is ready to accept requests"
    }))
} 
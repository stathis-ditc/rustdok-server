#[cfg(test)]
// Tests for the objects API endpoints
// These tests focus on the API endpoints for object operations

use actix_web::{test, App};
use actix_web::http::StatusCode;
use serde_json::json;
use crate::api::config::configure_api_v1;
use std::env;
use std::sync::Once;
use dotenv::dotenv;
use log::debug;
use env_logger;

static INIT: Once = Once::new();

fn setup() {
    INIT.call_once(|| {
        // Load env vars from .env file if it exists
        dotenv().ok();
        
        if env::var("RUST_LOG").is_err() {
            unsafe {
                env::set_var("RUST_LOG", "debug");
            }
        }
        env_logger::builder().is_test(true).init();
        
        unsafe {
            env::set_var("S3_ENDPOINT_URL", "http://localhost:7000");
            env::set_var("S3_ACCESS_KEY", "test-access-key");
            env::set_var("S3_SECRET_KEY", "test-secret-key");
        }
        
        assert!(env::var("S3_ENDPOINT_URL").is_ok(), "S3_ENDPOINT_URL not set");
        assert!(env::var("S3_ACCESS_KEY").is_ok(), "S3_ACCESS_KEY not set");
        assert!(env::var("S3_SECRET_KEY").is_ok(), "S3_SECRET_KEY not set");
    });
}

#[actix_web::test]
async fn test_api_endpoints_registration() {
    setup();
    
    debug!("Starting API endpoints registration test");
    
    let app = test::init_service(
        App::new()
            .service(configure_api_v1())
    ).await;
    
    debug!("Test app initialized");
    
    let req = test::TestRequest::get().uri("/api/v1/bucket/test-bucket/objects").to_request();
    let resp = test::call_service(&app, req).await;
    // We expect a 500 error because there's no actual S3 service, but the endpoint should be registered
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    debug!("list_objects_in_bucket endpoint test completed");
    
    let req = test::TestRequest::get().uri("/api/v1/bucket/test-bucket/exists?filename=test.txt").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    
    let req = test::TestRequest::get().uri("/api/v1/bucket/test-bucket/download/test.txt").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    
    let req = test::TestRequest::delete().uri("/api/v1/bucket/test-bucket/object/test.txt").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    
    let req = test::TestRequest::post()
        .uri("/api/v1/bucket/test-bucket/folders")
        .set_json(&json!({
            "folder_name": "test-folder"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    
    let req = test::TestRequest::get().uri("/api/v1/bucket/test-bucket/view/test.txt").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    
    let req = test::TestRequest::post()
        .uri("/api/v1/bucket/test-bucket/move")
        .set_json(&json!({
            "source_key": "source/test.txt",
            "destination_key": "destination/test.txt"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    debug!("move_file_in_bucket endpoint test completed");
    
    debug!("All API endpoints registration tests completed successfully");
} 
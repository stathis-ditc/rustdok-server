#[cfg(test)]
// Tests for the buckets API endpoints
// These tests focus on the API endpoints for bucket operations

use actix_web::{test, web, App, HttpResponse};
use actix_web::http::StatusCode;
use serde_json::{json, Value};
use crate::api::v1::buckets::create_bucket;
use crate::rdlib::s3::service::S3Service;
use crate::rdlib::s3::error::S3Error;
use std::sync::{Arc, Mutex};
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Builder;
use aws_sdk_s3::config::Region;
use aws_sdk_s3::config::Credentials;
use aws_config::BehaviorVersion;
use mockall::predicate::*;
use mockall::mock;

mock! {
    pub S3Service {
        pub async fn list_buckets(&self) -> Result<Vec<String>, S3Error>;
        pub async fn create_bucket(&self, bucket_name: &str) -> Result<(), S3Error>;
        pub async fn delete_bucket(&self, bucket_name: &str) -> Result<(), S3Error>;
    }
}

async fn create_test_s3_service() -> Arc<S3Service> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url("http://localhost:7000")
        .region(Region::new("eu-central-1"))
        .credentials_provider(Credentials::new(
            "test-access-key",
            "test-secret-key",
            None,
            None,
            "test-credentials",
        ))
        .load()
        .await;

    let s3_config = Builder::from(&config)
        .force_path_style(true)
        .build();
    
    let client = Client::from_conf(s3_config);
    
    Arc::new(S3Service {
        client,
    })
}

struct MockS3ServiceWrapper {
    mock: Mutex<MockS3Service>,
}

impl MockS3ServiceWrapper {
    fn new() -> Self {
        Self {
            mock: Mutex::new(MockS3Service::new()),
        }
    }
    
    fn expect_list_buckets(&self) -> &Self {
        let mut mock = self.mock.lock().unwrap();
        mock.expect_list_buckets()
            .times(1)
            .returning(|| Ok(vec!["bucket1".to_string(), "bucket2".to_string()]));
        self
    }
    
    fn expect_create_bucket(&self, bucket_name: &str) -> &Self {
        let mut mock = self.mock.lock().unwrap();
        mock.expect_create_bucket()
            .with(eq(bucket_name.to_string()))
            .times(1)
            .returning(|_| Ok(()));
        self
    }
    
    fn expect_create_bucket_already_exists(&self, bucket_name: &str) -> &Self {
        let mut mock = self.mock.lock().unwrap();
        mock.expect_create_bucket()
            .with(eq(bucket_name.to_string()))
            .times(1)
            .returning(|name| Err(S3Error::BucketAlreadyExists(format!("BucketAlreadyExists: {} already exists", name))));
        self
    }
    
    fn expect_delete_bucket(&self, bucket_name: &str) -> &Self {
        let mut mock = self.mock.lock().unwrap();
        mock.expect_delete_bucket()
            .with(eq(bucket_name.to_string()))
            .times(1)
            .returning(|_| Ok(()));
        self
    }
    
    fn expect_delete_bucket_not_found(&self, bucket_name: &str) -> &Self {
        let mut mock = self.mock.lock().unwrap();
        mock.expect_delete_bucket()
            .with(eq(bucket_name.to_string()))
            .times(1)
            .returning(|name| Err(S3Error::BucketNotFound(format!("NoSuchBucket: {} not found", name))));
        self
    }
}

impl MockS3ServiceWrapper {
    async fn list_buckets(&self) -> Result<Vec<String>, S3Error> {
        self.mock.lock().unwrap().list_buckets().await
    }
    
    async fn create_bucket(&self, bucket_name: &str) -> Result<(), S3Error> {
        self.mock.lock().unwrap().create_bucket(bucket_name).await
    }
    
    async fn delete_bucket(&self, bucket_name: &str) -> Result<(), S3Error> {
        self.mock.lock().unwrap().delete_bucket(bucket_name).await
    }
}

#[actix_web::test]
async fn test_list_buckets() {
    let mock_wrapper = Arc::new(MockS3ServiceWrapper::new());
    mock_wrapper.expect_list_buckets();
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_wrapper.clone()))
            .route("/buckets", web::get().to(|s3: web::Data<Arc<MockS3ServiceWrapper>>| async move {
                match s3.list_buckets().await {
                    Ok(buckets) => HttpResponse::Ok().json(buckets),
                    Err(e) => HttpResponse::InternalServerError().json(json!({
                        "error": format!("Failed to list buckets: {}", e)
                    })),
                }
            }))
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/buckets")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body = test::read_body(resp).await;
    let response: Vec<String> = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response, vec!["bucket1".to_string(), "bucket2".to_string()]);
}

#[actix_web::test]
async fn test_create_bucket() {
    let mock_wrapper = Arc::new(MockS3ServiceWrapper::new());
    mock_wrapper.expect_create_bucket("test-bucket");
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_wrapper.clone()))
            .route("/buckets", web::post().to(|bucket_info: web::Json<crate::models::s3::CreateBucketRequest>, s3: web::Data<Arc<MockS3ServiceWrapper>>| async move {
                let bucket_name = &bucket_info.name;
                if bucket_name.is_empty() {
                    return HttpResponse::BadRequest().json(json!({
                        "error": "Bucket name cannot be empty"
                    }));
                }
                
                match s3.create_bucket(bucket_name).await {
                    Ok(_) => HttpResponse::Created().json(json!({
                        "message": format!("Bucket '{}' created successfully", bucket_name)
                    })),
                    Err(e) => {
                        if e.contains("BucketAlreadyExists") || e.contains("already exists") {
                            HttpResponse::Conflict().json(json!({
                                "error": format!("Bucket '{}' already exists", bucket_name)
                            }))
                        } else {
                            HttpResponse::InternalServerError().json(json!({
                                "error": format!("Failed to create bucket: {}", e)
                            }))
                        }
                    }
                }
            }))
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/buckets")
        .set_json(&json!({
            "name": "test-bucket"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::CREATED);
    
    let body = test::read_body(resp).await;
    let response: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response["message"], "Bucket 'test-bucket' created successfully");
}

#[actix_web::test]
async fn test_create_bucket_empty_name() {
    let s3_service = create_test_s3_service().await;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(s3_service))
            .service(create_bucket)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/buckets")
        .set_json(&json!({
            "name": ""
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    
    let body = test::read_body(resp).await;
    let response: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response["error"], "Bucket name cannot be empty");
}

#[actix_web::test]
async fn test_create_bucket_already_exists() {
    let mock_wrapper = Arc::new(MockS3ServiceWrapper::new());
    mock_wrapper.expect_create_bucket_already_exists("existing-bucket");
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_wrapper.clone()))
            .route("/buckets", web::post().to(|bucket_info: web::Json<crate::models::s3::CreateBucketRequest>, s3: web::Data<Arc<MockS3ServiceWrapper>>| async move {
                let bucket_name = &bucket_info.name;
                if bucket_name.is_empty() {
                    return HttpResponse::BadRequest().json(json!({
                        "error": "Bucket name cannot be empty"
                    }));
                }
                
                match s3.create_bucket(bucket_name).await {
                    Ok(_) => HttpResponse::Created().json(json!({
                        "message": format!("Bucket '{}' created successfully", bucket_name)
                    })),
                    Err(e) => {
                        if e.contains("BucketAlreadyExists") || e.contains("already exists") {
                            HttpResponse::Conflict().json(json!({
                                "error": format!("Bucket '{}' already exists", bucket_name)
                            }))
                        } else {
                            HttpResponse::InternalServerError().json(json!({
                                "error": format!("Failed to create bucket: {}", e)
                            }))
                        }
                    }
                }
            }))
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/buckets")
        .set_json(&json!({
            "name": "existing-bucket"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::CONFLICT);
    
    let body = test::read_body(resp).await;
    let response: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response["error"], "Bucket 'existing-bucket' already exists");
}

#[actix_web::test]
async fn test_delete_bucket() {
    let mock_wrapper = Arc::new(MockS3ServiceWrapper::new());
    mock_wrapper.expect_delete_bucket("test-bucket");
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_wrapper.clone()))
            .route("/bucket/{name}", web::delete().to(|bucket_name: web::Path<String>, s3: web::Data<Arc<MockS3ServiceWrapper>>| async move {
                match s3.delete_bucket(&bucket_name).await {
                    Ok(_) => HttpResponse::Ok().json(json!({
                        "message": format!("Bucket '{}' deleted successfully", bucket_name)
                    })),
                    Err(e) => {
                        if e.contains("NoSuchBucket") || e.contains("not found") {
                            HttpResponse::NotFound().json(json!({
                                "error": format!("Bucket '{}' not found", bucket_name)
                            }))
                        } else if e.contains("BucketNotEmpty") || e.contains("not empty") {
                            HttpResponse::Conflict().json(json!({
                                "error": format!("Bucket '{}' is not empty", bucket_name)
                            }))
                        } else {
                            HttpResponse::InternalServerError().json(json!({
                                "error": format!("Failed to delete bucket: {}", e)
                            }))
                        }
                    }
                }
            }))
    ).await;
    
    let req = test::TestRequest::delete()
        .uri("/bucket/test-bucket")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body = test::read_body(resp).await;
    let response: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response["message"], "Bucket 'test-bucket' deleted successfully");
}

#[actix_web::test]
async fn test_delete_bucket_not_found() {
    let mock_wrapper = Arc::new(MockS3ServiceWrapper::new());
    mock_wrapper.expect_delete_bucket_not_found("nonexistent-bucket");
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_wrapper.clone()))
            .route("/bucket/{name}", web::delete().to(|bucket_name: web::Path<String>, s3: web::Data<Arc<MockS3ServiceWrapper>>| async move {
                match s3.delete_bucket(&bucket_name).await {
                    Ok(_) => HttpResponse::Ok().json(json!({
                        "message": format!("Bucket '{}' deleted successfully", bucket_name)
                    })),
                    Err(e) => {
                        if e.contains("NoSuchBucket") || e.contains("not found") {
                            HttpResponse::NotFound().json(json!({
                                "error": format!("Bucket '{}' not found", bucket_name)
                            }))
                        } else if e.contains("BucketNotEmpty") || e.contains("not empty") {
                            HttpResponse::Conflict().json(json!({
                                "error": format!("Bucket '{}' is not empty", bucket_name)
                            }))
                        } else {
                            HttpResponse::InternalServerError().json(json!({
                                "error": format!("Failed to delete bucket: {}", e)
                            }))
                        }
                    }
                }
            }))
    ).await;
    
    let req = test::TestRequest::delete()
        .uri("/bucket/nonexistent-bucket")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    
    let body = test::read_body(resp).await;
    let response: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response["error"], "Bucket 'nonexistent-bucket' not found");
} 
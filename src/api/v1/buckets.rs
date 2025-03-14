//! # Bucket API Endpoints
//! 
//! This module provides the API endpoints for bucket operations.
//! It includes handlers for listing, creating, and deleting buckets.

use actix_web::{post, get, delete, web, HttpResponse, Error};
use crate::rdlib::s3::service::S3Service;
use serde_json::json;
use crate::models::s3::CreateBucketRequest;
use std::sync::Arc;
use log::error;

/// Lists all buckets.
///
/// This endpoint retrieves a list of all buckets available in the storage.
///
/// # Returns
///
/// * `200 OK` - A JSON array of bucket names
/// * `500 Internal Server Error` - If there was an error listing the buckets
#[get("/buckets")]
pub async fn list_buckets(s3_service: web::Data<Arc<S3Service>>) -> Result<HttpResponse, Error> {
    let s3 = s3_service.as_ref();
    
    match s3.list_buckets().await {
        Ok(buckets) => Ok(HttpResponse::Ok().json(buckets)),
        Err(e) => {
            error!("Error listing buckets: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to list buckets: {}", e)
            })))
        }
    }
}

/// Creates a new bucket.
///
/// This endpoint creates a new bucket with the specified name.
///
/// # Request Body
///
/// * `name` - The name of the bucket to create
///
/// # Returns
///
/// * `201 Created` - If the bucket was created successfully
/// * `400 Bad Request` - If the bucket name is invalid
/// * `409 Conflict` - If the bucket already exists
/// * `500 Internal Server Error` - If there was an error creating the bucket
#[post("/buckets")]
pub async fn create_bucket(
    bucket_info: web::Json<CreateBucketRequest>,
    s3_service: web::Data<Arc<S3Service>>
) -> Result<HttpResponse, Error> {
    let s3 = s3_service.as_ref();
    
    let bucket_name = &bucket_info.name;
    if bucket_name.is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "error": "Bucket name cannot be empty"
        })));
    }
    
    // Create the bucket
    match s3.create_bucket(bucket_name).await {
        Ok(_) => Ok(HttpResponse::Created().json(json!({
            "message": format!("Bucket '{}' created successfully", bucket_name)
        }))),
        Err(e) => {
            if e.contains("BucketAlreadyExists") || e.contains("already exists") {
                Ok(HttpResponse::Conflict().json(json!({
                    "error": format!("Bucket '{}' already exists", bucket_name)
                })))
            } else {
                error!("Error creating bucket: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(json!({
                    "error": format!("Failed to create bucket: {}", e)
                })))
            }
        }
    }
}

/// Deletes a bucket.
///
/// This endpoint deletes the specified bucket.
///
/// # Path Parameters
///
/// * `name` - The name of the bucket to delete
///
/// # Returns
///
/// * `200 OK` - If the bucket was deleted successfully
/// * `404 Not Found` - If the bucket does not exist
/// * `409 Conflict` - If the bucket is not empty
/// * `500 Internal Server Error` - If there was an error deleting the bucket
#[delete("/bucket/{name}")]
pub async fn delete_bucket(
    bucket_name: web::Path<String>,
    s3_service: web::Data<Arc<S3Service>>
) -> Result<HttpResponse, Error> {
    let s3 = s3_service.as_ref();
    
    match s3.delete_bucket(&bucket_name).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "message": format!("Bucket '{}' deleted successfully", bucket_name)
        }))),
        Err(e) => {
            if e.contains("NoSuchBucket") || e.contains("not found") {
                Ok(HttpResponse::NotFound().json(json!({
                    "error": format!("Bucket '{}' not found", bucket_name)
                })))
            } else if e.contains("BucketNotEmpty") || e.contains("not empty") {
                Ok(HttpResponse::Conflict().json(json!({
                    "error": format!("Bucket '{}' is not empty", bucket_name)
                })))
            } else {
                error!("Error deleting bucket: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(json!({
                    "error": format!("Failed to delete bucket: {}", e)
                })))
            }
        }
    }
}


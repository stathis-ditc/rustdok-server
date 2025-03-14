//! # Object API Endpoints
//! 
//! This module provides the API endpoints for object operations.
//! It includes handlers for listing, uploading, downloading, viewing,
//! deleting, and managing objects in buckets.

use actix_web::{post, get, delete, web, HttpResponse, Error};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use serde_json::json;
use crate::rdlib::s3::service::S3Service;
use uuid::Uuid;
use serde::Deserialize;
use crate::models::s3::CreateFolderRequest;
use std::io::Write;
use sanitize_filename;
use std::path::Path;
use log::error;
use std::sync::Arc;

/// Query parameters for listing objects with an optional prefix
#[derive(Deserialize)]
pub struct PrefixQuery {
    /// Optional prefix to filter objects by
    prefix: Option<String>,
    /// Whether to replace existing objects with the same name
    replace: Option<bool>,
}

/// Query parameters for checking if a file exists
#[derive(Deserialize)]
pub struct FileExistsQuery {
    /// The filename to check
    filename: String,
}

/// Request body for moving a file within a bucket
#[derive(Deserialize)]
pub struct MoveFileRequest {
    /// The source key (path) of the file to move
    source_key: String,
    /// The destination key (path) where the file should be moved to
    destination_key: String,
}

/// Lists objects in a bucket, optionally filtered by prefix.
///
/// This endpoint retrieves a list of objects in the specified bucket.
/// If a prefix is provided, only objects with keys starting with that prefix are returned.
///
/// # Path Parameters
///
/// * `bucket` - The name of the bucket to list objects from
///
/// # Query Parameters
///
/// * `prefix` - Optional prefix to filter objects by
///
/// # Returns
///
/// * `200 OK` - A JSON array of objects
/// * `404 Not Found` - If the bucket does not exist
/// * `500 Internal Server Error` - If there was an error listing the objects
#[get("/bucket/{bucket}/objects")]
pub async fn list_objects_in_bucket(
    bucket: web::Path<String>, 
    query: web::Query<PrefixQuery>,
    s3_service: web::Data<Arc<S3Service>>
) -> Result<HttpResponse, Error> {
    let s3 = s3_service.as_ref();
    let prefix = query.prefix.clone().unwrap_or_default();
    
    match s3.list_objects(Some(&prefix), &bucket).await {
        Ok(objects) => Ok(HttpResponse::Ok().json(objects)),
        Err(e) => {
            error!("Error listing objects in bucket {}: {:?}", bucket, e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to list objects in bucket {}: {}", bucket, e)
            })))
        }
    }
}

/// Downloads an object from a bucket.
///
/// This endpoint retrieves the binary data of an object from the specified bucket.
///
/// # Path Parameters
///
/// * `bucket` - The name of the bucket containing the object
/// * `key` - The key (path) of the object to download
///
/// # Returns
///
/// * `200 OK` - The binary data of the object with appropriate content type
/// * `404 Not Found` - If the object does not exist
/// * `500 Internal Server Error` - If there was an error downloading the object
#[get("/bucket/{bucket}/download/{key:.*}")]
pub async fn download_object_from_bucket(
    path: web::Path<(String, String)>,
    s3_service: web::Data<Arc<S3Service>>
) -> Result<HttpResponse, Error> {
    let (bucket, key) = path.into_inner();
    let s3 = s3_service.as_ref();
    
    match s3.get_object(&key, &bucket).await {
        Ok(data) => {
            let filename = Path::new(&key).file_name().unwrap_or_default().to_string_lossy();
            Ok(HttpResponse::Ok()
                .content_type("application/octet-stream")
                .append_header(("Content-Disposition", format!("attachment; filename=\"{}\"", filename)))
                .body(data))
        },
        Err(e) => {
            error!("Error downloading file {}/{}: {:?}", bucket, key, e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to download file: {}", e)
            })))
        }
    }
}

/// Checks if an object exists in a bucket.
///
/// This endpoint checks if an object with the specified filename exists in the bucket.
///
/// # Path Parameters
///
/// * `bucket` - The name of the bucket to check
///
/// # Query Parameters
///
/// * `filename` - The filename to check
///
/// # Returns
///
/// * `200 OK` - A JSON object with `exists` field indicating whether the object exists
/// * `500 Internal Server Error` - If there was an error checking the object
#[get("/bucket/{bucket}/exists")]
pub async fn check_object_exists_in_bucket(
    bucket: web::Path<String>, 
    query: web::Query<FileExistsQuery>,
    s3_service: web::Data<Arc<S3Service>>
) -> Result<HttpResponse, Error> {
    let s3 = s3_service.as_ref();
    let filename = &query.filename;
    
    match s3.check_object_exists(filename, &bucket).await {
        Ok(exists) => Ok(HttpResponse::Ok().json(json!({ "exists": exists }))),
        Err(e) => {
            error!("Error checking if file exists in bucket {}/{}: {:?}", bucket, filename, e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to check if file exists in bucket: {}", e)
            })))
        }
    }
}

/// Uploads an object to a bucket.
///
/// This endpoint uploads a file to the specified bucket.
/// If a prefix is provided, the file is stored under that prefix.
///
/// # Path Parameters
///
/// * `bucket` - The name of the bucket to upload to
///
/// # Query Parameters
///
/// * `prefix` - Optional prefix to store the file under
/// * `replace` - Whether to replace an existing file with the same name
///
/// # Request Body
///
/// * Multipart form data containing the file to upload
///
/// # Returns
///
/// * `201 Created` - If the file was uploaded successfully
/// * `400 Bad Request` - If the file is invalid or missing
/// * `500 Internal Server Error` - If there was an error uploading the file
#[post("/bucket/{bucket}/objects")]
pub async fn upload_object_to_bucket(
    bucket: web::Path<String>, 
    query: web::Query<PrefixQuery>, 
    mut payload: Multipart,
    s3_service: web::Data<Arc<S3Service>>
) -> Result<HttpResponse, Error> {
    let s3 = s3_service.as_ref();
    let prefix = query.prefix.clone().unwrap_or_default();
    let replace = query.replace.unwrap_or(false);
    
    let mut uploaded_files = Vec::new();
    
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        let filename = content_disposition.as_ref().and_then(|cd| cd.get_filename()).map_or_else(
            || Uuid::new_v4().to_string(),
            |f| sanitize_filename::sanitize(f)
        );
        
        let key = if prefix.is_empty() {
            filename.clone()
        } else {
            format!("{}/{}", prefix.trim_end_matches('/'), filename)
        };
        
        // Check if file exists and we're not replacing
        if !replace {
            match s3.check_object_exists(&key, &bucket).await {
                Ok(true) => {
                    return Ok(HttpResponse::Conflict().json(json!({
                        "error": format!("File {} already exists in bucket {}", key, bucket)
                    })));
                },
                Ok(false) => {},
                Err(e) => {
                    error!("Error checking if file exists: {:?}", e);
                    return Ok(HttpResponse::InternalServerError().json(json!({
                        "error": format!("Failed to check if file exists: {}", e)
                    })));
                }
            }
        }
        
        let mut data = Vec::new();
        while let Some(chunk) = field.next().await {
            let data_chunk = chunk?;
            data.write_all(&data_chunk)?;
        }
        
        match s3.put_object(&key, data.to_vec(), &bucket).await {
            Ok(_) => {
                uploaded_files.push(json!({
                    "filename": filename,
                    "key": key,
                    "size": data.len(),
                    "bucket": bucket.to_string()
                }));
            },
            Err(e) => {
                error!("Error uploading file: {:?}", e);
                return Ok(HttpResponse::InternalServerError().json(json!({
                    "error": format!("Failed to upload file: {}", e)
                })));
            }
        }
    }
    
    Ok(HttpResponse::Ok().json(json!({
        "files": uploaded_files
    })))
}

/// Deletes an object from a bucket.
///
/// This endpoint deletes the specified object from the bucket.
///
/// # Path Parameters
///
/// * `bucket` - The name of the bucket containing the object
/// * `key` - The key (path) of the object to delete
///
/// # Returns
///
/// * `204 No Content` - If the object was deleted successfully
/// * `404 Not Found` - If the object does not exist
/// * `500 Internal Server Error` - If there was an error deleting the object
#[delete("/bucket/{bucket}/object/{key:.*}")]
pub async fn delete_object_from_bucket(
    path: web::Path<(String, String)>,
    s3_service: web::Data<Arc<S3Service>>
) -> Result<HttpResponse, Error> {
    let (bucket, key) = path.into_inner();
    let s3 = s3_service.as_ref();
    
    match s3.delete_objects(vec![&key], &bucket).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "message": "File deleted successfully",
            "key": key,
            "bucket": bucket
        }))),
        Err(e) => {
            error!("Error deleting file {}/{}: {:?}", bucket, key, e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to delete file: {}", e)
            })))
        }
    }
}

/// Creates a folder in a bucket.
///
/// This endpoint creates a new folder in the specified bucket.
///
/// # Path Parameters
///
/// * `bucket` - The name of the bucket to create the folder in
///
/// # Request Body
///
/// * `name` - The name of the folder to create
///
/// # Returns
///
/// * `201 Created` - If the folder was created successfully
/// * `400 Bad Request` - If the folder name is invalid
/// * `409 Conflict` - If the folder already exists
/// * `500 Internal Server Error` - If there was an error creating the folder
#[post("/bucket/{bucket}/folders")]
pub async fn create_folder(
    bucket: web::Path<String>, 
    folder_info: web::Json<CreateFolderRequest>,
    s3_service: web::Data<Arc<S3Service>>
) -> Result<HttpResponse, Error> {
    let s3 = s3_service.as_ref();
    let folder_path = if folder_info.name.ends_with('/') {
        folder_info.name.clone()
    } else {
        format!("{}/", folder_info.name)
    };
    
    match s3.put_object(&folder_path, vec![], &bucket).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "message": "Folder created successfully",
            "path": folder_path,
            "bucket": bucket.to_string()
        }))),
        Err(e) => {
            error!("Error creating folder {}/{}: {:?}", bucket, folder_path, e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to create folder: {}", e)
            })))
        }
    }
}

/// Views an object from a bucket.
///
/// This endpoint retrieves the binary data of an object from the specified bucket
/// and returns it with appropriate content type for viewing in a browser.
///
/// # Path Parameters
///
/// * `bucket` - The name of the bucket containing the object
/// * `key` - The key (path) of the object to view
///
/// # Returns
///
/// * `200 OK` - The binary data of the object with appropriate content type
/// * `404 Not Found` - If the object does not exist
/// * `500 Internal Server Error` - If there was an error retrieving the object
#[get("/bucket/{bucket}/view/{key:.*}")]
pub async fn view_object_from_bucket(
    path: web::Path<(String, String)>,
    s3_service: web::Data<Arc<S3Service>>
) -> Result<HttpResponse, Error> {
    let (bucket, key) = path.into_inner();
    let s3 = s3_service.as_ref();
    
    match s3.get_object(&key, &bucket).await {
        Ok(data) => {
            let filename = Path::new(&key).file_name().unwrap_or_default().to_string_lossy();
            let content_type = mime_guess::from_path(&*filename).first_or_octet_stream();
            
            Ok(HttpResponse::Ok()
                .content_type(content_type.as_ref())
                .body(data))
        },
        Err(e) => {
            error!("Error viewing file {}/{}: {:?}", bucket, key, e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to view file: {}", e)
            })))
        }
    }
}

#[post("/bucket/{bucket}/move")]
pub async fn move_file_in_bucket(
    bucket: web::Path<String>,
    move_request: web::Json<MoveFileRequest>,
    s3_service: web::Data<Arc<S3Service>>
) -> Result<HttpResponse, Error> {
    let s3 = s3_service.as_ref();
    
    // Check if source file exists
    match s3.check_object_exists(&move_request.source_key, &bucket).await {
        Ok(false) => {
            return Ok(HttpResponse::NotFound().json(json!({
                "error": format!("Source file {} does not exist in bucket {}", move_request.source_key, bucket)
            })));
        },
        Ok(true) => {},
        Err(e) => {
            error!("Error checking if source file exists: {:?}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to check if source file exists: {}", e)
            })));
        }
    }
    
    // Check if destination file already exists
    match s3.check_object_exists(&move_request.destination_key, &bucket).await {
        Ok(true) => {
            return Ok(HttpResponse::Conflict().json(json!({
                "error": format!("Destination file {} already exists in bucket {}", move_request.destination_key, bucket)
            })));
        },
        Ok(false) => {},
        Err(e) => {
            error!("Error checking if destination file exists: {:?}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to check if destination file exists: {}", e)
            })));
        }
    }
    
    // Copy the file to the new location
    match s3.client.copy_object()
        .bucket(&*bucket)
        .copy_source(format!("{}/{}", &*bucket, &move_request.source_key))
        .key(&move_request.destination_key)
        .send()
        .await {
        Ok(_) => {
            // Delete the source object
            match s3.client.delete_object()
                .bucket(&*bucket)
                .key(&move_request.source_key)
                .send()
                .await {
                Ok(_) => Ok(HttpResponse::Ok().json(json!({
                    "message": "File moved successfully",
                    "source": move_request.source_key,
                    "destination": move_request.destination_key,
                    "bucket": bucket.to_string()
                }))),
                Err(e) => {
                    error!("Error deleting source file after copy: {:?}", e);
                    Ok(HttpResponse::InternalServerError().json(json!({
                        "error": format!("File was copied but could not be deleted from source: {}", e),
                        "source": move_request.source_key,
                        "destination": move_request.destination_key,
                        "bucket": bucket.to_string()
                    })))
                }
            }
        },
        Err(e) => {
            error!("Error copying file: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to copy file: {}", e)
            })))
        }
    }
} 
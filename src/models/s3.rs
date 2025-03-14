//! # S3 Data Models
//! 
//! This module contains the data models specific to S3 operations.
//! It defines the structures for S3 request and response data.

use serde::Deserialize;

/// Request model for creating a folder in a bucket.
///
/// This structure represents the request body for the create folder API endpoint.
#[derive(Deserialize)]
pub struct CreateFolderRequest {
    /// The name of the folder to create
    pub name: String,
}

/// Request model for creating a bucket.
///
/// This structure represents the request body for the create bucket API endpoint.
#[derive(Deserialize)]
pub struct CreateBucketRequest {
    /// The name of the bucket to create
    pub name: String,
} 
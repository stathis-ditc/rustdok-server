//! # Bucket Listing
//! 
//! This module provides functionality for listing S3 buckets.
//! It includes methods for retrieving all available buckets.

use crate::rdlib::s3::error::S3Error;
use crate::rdlib::s3::service::S3Service;

impl S3Service {
    /// Lists all buckets.
    ///
    /// This method retrieves a list of all buckets available to the authenticated user.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<String>)` - A vector of bucket names if successful
    /// * `Err(S3Error)` - If there was an error listing the buckets
    pub async fn list_buckets(&self) -> Result<Vec<String>, S3Error> {
        let resp = self.client.list_buckets().send().await
            .map_err(|e| S3Error::Other(format!("Failed to list buckets: {}", e)))?;
        
        let mut buckets = Vec::new();
        let bucket_list = resp.buckets();
        for bucket in bucket_list {
            if let Some(name) = bucket.name() {
                buckets.push(name.to_string());
            }
        }
        
        Ok(buckets)
    }
}
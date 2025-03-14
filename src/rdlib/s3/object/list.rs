//! # Object Listing
//! 
//! This module provides functionality for listing objects in S3 buckets.
//! It includes methods for listing objects with prefixes to simulate folder-like navigation.

use std::time::{Duration, UNIX_EPOCH};
use chrono::{DateTime, Utc};

use crate::rdlib::s3::error::S3Error;
use crate::rdlib::s3::service::S3Service;
use crate::rdlib::s3::types::S3Object;
use log::info;

impl S3Service {
    /// Lists objects in a specific bucket with a prefix.
    ///
    /// This method retrieves a list of objects from the specified bucket
    /// that have keys starting with the specified prefix. It uses a delimiter
    /// to simulate folder-like navigation.
    ///
    /// # Arguments
    ///
    /// * `bucket` - The name of the bucket to list objects from
    /// * `prefix` - The prefix to filter objects by
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<S3Object>)` - A vector of S3Object structs if successful
    /// * `Err(S3Error)` - If there was an error listing the objects
    async fn list_objects_in_bucket_with_prefix(&self, bucket: &str, prefix: &str) -> Result<Vec<S3Object>, S3Error> {
        info!("Fetching objects from bucket: {} with prefix: '{}'", bucket, prefix);
        
        let mut req = self.client.list_objects_v2().bucket(bucket);
        
        if !prefix.is_empty() {
            req = req.prefix(prefix);
        }
        
        // Add delimiter to simulate folder-like navigation
        // This is crucial for S3 to group objects by "folders"
        req = req.delimiter("/");
        
        let resp = req.send().await?;
        
        info!("Response received from S3");
        
        let mut files = Vec::new();

        let prefixes = resp.common_prefixes();
        info!("Found {} common prefixes (folders)", prefixes.len());
        for prefix_obj in prefixes {
            if let Some(prefix_str) = prefix_obj.prefix() {
                info!("Processing folder: {}", prefix_str);
                
                files.push(S3Object {
                    name: prefix_str.to_string(),
                    size: 0, 
                    last_modified: None, 
                });
            }
        }
        
        let objects = resp.contents();
        info!("Found {} objects (files) in response", objects.len());
        for obj in objects {
            let key = obj.key().unwrap_or_default().to_string();
            let size = obj.size().unwrap_or(0) as u64;
            
            // Skip objects that are exactly the same as the prefix
            // These are the empty objects we create to represent folders
            if !prefix.is_empty() && key == prefix {
                info!("Skipping folder placeholder object: {}", key);
                continue;
            }
            
            info!("Processing file: {}, size: {}", key, size);
            
            let last_modified = obj.last_modified().map(|dt| {
                let secs = dt.secs();

                let secs_u64 = if secs < 0 { 0 } else { secs as u64 };
                let system_time = UNIX_EPOCH + Duration::from_secs(secs_u64);
                
                let datetime: DateTime<Utc> = system_time.into();
                datetime.to_rfc3339()
            });
            
            files.push(S3Object {
                name: key,
                size,
                last_modified,
            });
        }

        info!("Returning {} files from S3 service", files.len());
        Ok(files)
    }

    /// Lists objects in a bucket, optionally filtered by prefix.
    ///
    /// This method retrieves a list of objects from either the specified bucket
    /// or the default bucket if none is provided. If a prefix is provided,
    /// only objects with keys starting with that prefix are returned.
    ///
    /// # Arguments
    ///
    /// * `prefix` - Optional prefix to filter objects by
    /// * `bucket` - Optional bucket name, defaults to the service's default bucket
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<S3Object>)` - A vector of S3Object structs if successful
    /// * `Err(S3Error)` - If there was an error listing the objects
    pub async fn list_objects(&self, prefix: Option<&str>, bucket: &str) -> Result<Vec<S3Object>, S3Error> {
        let prefix_to_use = prefix.unwrap_or("");
        
        self.list_objects_in_bucket_with_prefix(bucket, prefix_to_use).await
    }
} 
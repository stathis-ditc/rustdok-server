//! # Bucket Creation
//! 
//! This module provides functionality for creating S3 buckets.
//! It includes validation of bucket names and checks to prevent
//! creating buckets that already exist.

use crate::rdlib::s3::error::S3Error;
use crate::rdlib::s3::service::S3Service;
use log::info;
impl S3Service {

    /// Creates a new bucket with the specified name.
    ///
    /// This method validates the bucket name, checks if the bucket already exists,
    /// and creates a new bucket if it doesn't.
    ///
    /// # Arguments
    ///
    /// * `bucket_name` - The name of the bucket to create
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the bucket was created successfully
    /// * `Err(S3Error)` - If the bucket name is invalid, the bucket already exists,
    ///   or there was an error creating the bucket
    pub async fn create_bucket(&self, bucket_name: &str) -> Result<(), S3Error> {
        info!("Creating bucket '{}'...", bucket_name);
        
        if let Err(validation_error) = Self::validate_bucket_name(bucket_name) {
            return Err(S3Error::Other(validation_error));
        }
        
        let buckets = self.list_buckets().await?;
        
        if buckets.contains(&bucket_name.to_string()) {
            info!("Bucket '{}' already exists", bucket_name);
            return Err(S3Error::BucketAlreadyExists(bucket_name.to_string()));
        }
        
        self.client
            .create_bucket()
            .bucket(bucket_name)
            .send()
            .await?;
            
        info!("Bucket '{}' created successfully", bucket_name);
        
        Ok(())
    }
}
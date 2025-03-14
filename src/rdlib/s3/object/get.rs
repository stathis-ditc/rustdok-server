//! # Object Download
//! 
//! This module provides functionality for downloading objects from S3 buckets.
//! It includes methods for getting objects from specific buckets or from the default bucket.

use crate::rdlib::s3::error::S3Error;
use crate::rdlib::s3::service::S3Service;

impl S3Service {
    /// Downloads an object from a specific bucket.
    ///
    /// This method retrieves an object with the specified key from the specified bucket.
    ///
    /// # Arguments
    ///
    /// * `bucket` - The name of the bucket to download from
    /// * `key` - The key (path) of the object
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u8>)` - The binary data of the object if downloaded successfully
    /// * `Err(S3Error)` - If there was an error downloading the object
    async fn get_object_from_bucket(&self, bucket: &str, key: &str) -> Result<Vec<u8>, S3Error> {
        let resp = self.client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;
        
        let data = resp.body.collect().await?;
        Ok(data.to_vec())
    }

    /// Downloads an object from a bucket, defaulting to the service's default bucket if none is specified.
    ///
    /// # Arguments
    ///
    /// * `key` - The key (path) of the object
    /// * `bucket` - Optional bucket name, defaults to the service's default bucket
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u8>)` - The binary data of the object if downloaded successfully
    /// * `Err(S3Error)` - If there was an error downloading the object
    pub async fn get_object(&self, key: &str, bucket: &str) -> Result<Vec<u8>, S3Error> {
        self.get_object_from_bucket(bucket, key).await
    }
}
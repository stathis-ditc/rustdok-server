//! # Object Upload
//! 
//! This module provides functionality for uploading objects to S3 buckets.
//! It includes methods for putting objects in specific buckets or in the default bucket.

use crate::rdlib::s3::error::S3Error;
use crate::rdlib::s3::service::S3Service;

impl S3Service {
    /// Uploads an object to a specific bucket.
    ///
    /// This method puts an object with the specified key and data into the specified bucket.
    ///
    /// # Arguments
    ///
    /// * `bucket` - The name of the bucket to upload to
    /// * `key` - The key (path) of the object
    /// * `data` - The binary data of the object
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the object was uploaded successfully
    /// * `Err(S3Error)` - If there was an error uploading the object
    async fn put_object_in_bucket(&self, bucket: &str, key: &str, data: Vec<u8>) -> Result<(), S3Error> {
        self.client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(data.into())
            .send()
            .await?;

        Ok(())
    }

    /// Uploads an object to a bucket, defaulting to the service's default bucket if none is specified.
    ///
    /// # Arguments
    ///
    /// * `key` - The key (path) of the object
    /// * `data` - The binary data of the object
    /// * `bucket` - Optional bucket name, defaults to the service's default bucket
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the object was uploaded successfully
    /// * `Err(S3Error)` - If there was an error uploading the object
    pub async fn put_object(&self, key: &str, data: Vec<u8>, bucket: &str) -> Result<(), S3Error> {
        self.put_object_in_bucket(bucket, key, data).await
    }
}
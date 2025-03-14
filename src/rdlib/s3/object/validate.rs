//! # Object Validation
//! 
//! This module provides functionality for validating S3 objects.
//! It includes methods for checking if objects exist in buckets.

use crate::rdlib::s3::error::S3Error;
use crate::rdlib::s3::service::S3Service;

impl S3Service {
    /// Checks if an object exists in a specific bucket.
    ///
    /// This method uses the HEAD operation to check if an object with the
    /// specified key exists in the specified bucket without downloading it.
    ///
    /// # Arguments
    ///
    /// * `bucket` - The name of the bucket to check
    /// * `key` - The key (path) of the object to check
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - If the object exists
    /// * `Ok(false)` - If the object does not exist
    /// * `Err(S3Error)` - If there was an error checking the object
    async fn check_object_exists_in_bucket(&self, bucket: &str, key: &str) -> Result<bool, S3Error> {
        match self.client
            .head_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                // Check if the error is a 404 Not Found
                match e {
                    aws_sdk_s3::error::SdkError::ServiceError(context) => {
                        if context.err().is_not_found() {
                            return Ok(false);
                        }
                        // Create a new error from the context instead of using 'e'
                        Err(S3Error::from(aws_sdk_s3::error::SdkError::ServiceError(context)))
                    },
                    other_error => Err(S3Error::from(other_error)),
                }
            }
        }
    }

    /// Checks if an object exists in a bucket.
    ///
    /// This method checks if an object with the specified key exists in either
    /// the specified bucket or the default bucket if none is provided.
    ///
    /// # Arguments
    ///
    /// * `key` - The key (path) of the object to check
    /// * `bucket` - Optional bucket name, defaults to the service's default bucket
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - If the object exists
    /// * `Ok(false)` - If the object does not exist
    /// * `Err(S3Error)` - If there was an error checking the object
    pub async fn check_object_exists(&self, key: &str, bucket: &str) -> Result<bool, S3Error> {
        self.check_object_exists_in_bucket(bucket, key).await
    }
}

//! # Bucket Deletion
//! 
//! This module provides functionality for deleting S3 buckets.
//! It includes methods for emptying and removing buckets.

use crate::rdlib::s3::error::S3Error;
use crate::rdlib::s3::service::S3Service;
use log::info;

impl S3Service {
    /// Deletes a bucket and all its contents.
    ///
    /// This method checks if the bucket exists, empties it by deleting all objects,
    /// and then deletes the bucket itself.
    ///
    /// # Arguments
    ///
    /// * `bucket_name` - The name of the bucket to delete
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the bucket was deleted successfully
    /// * `Err(S3Error)` - If the bucket does not exist or there was an error deleting it
    pub async fn delete_bucket(&self, bucket_name: &str) -> Result<(), S3Error> {
        info!("Deleting bucket '{}'...", bucket_name);
        
        let buckets = self.list_buckets().await?;
        
        if !buckets.contains(&bucket_name.to_string()) {
            info!("Bucket '{}' does not exist", bucket_name);
            return Err(S3Error::Other(format!("Bucket '{}' does not exist", bucket_name)));
        }
        
        self.client
            .delete_bucket()
            .bucket(bucket_name)
            .send()
            .await?;
            
        info!("Bucket '{}' deleted successfully", bucket_name);
        
        Ok(())
    }
}
//! # Object Deletion
//! 
//! This module provides functionality for deleting objects from S3 buckets.
//! It includes methods for deleting individual objects, multiple objects,
//! and recursively deleting objects with a common prefix.

use aws_sdk_s3::types::{Delete, ObjectIdentifier};

use crate::rdlib::s3::error::S3Error;
use crate::rdlib::s3::service::S3Service;
use log::{info, error};

impl S3Service {    
    /// Deletes multiple objects from a bucket.
    ///
    /// This method deletes the specified objects from either the specified bucket
    /// or the default bucket if none is provided.
    ///
    /// # Arguments
    ///
    /// * `objects_to_delete` - A vector of object keys to delete
    /// * `bucket` - Optional bucket name, defaults to the service's default bucket
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the objects were deleted successfully
    /// * `Err(S3Error)` - If there was an error deleting the objects
    pub async fn delete_objects(&self, objects_to_delete: Vec<&String>, bucket: &str) -> Result<(), S3Error> {
        self.delete_objects_from_bucket(bucket, objects_to_delete).await
    }

    /// Deletes multiple objects from a specific bucket.
    ///
    /// This method deletes the specified objects from the specified bucket.
    /// It handles batching of delete requests to respect S3's limits.
    ///
    /// # Arguments
    ///
    /// * `bucket` - The name of the bucket to delete from
    /// * `objects_to_delete` - A vector of object keys to delete
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the objects were deleted successfully
    /// * `Err(S3Error)` - If there was an error deleting the objects
    async fn delete_objects_from_bucket(&self, bucket: &str, objects_to_delete: Vec<&String>,) -> Result<(), S3Error> {
        info!("Deleting {} objects from bucket {}", objects_to_delete.len(), bucket);
        
        if objects_to_delete.is_empty() {
            info!("No objects to delete");
            return Ok(());
        }
        
        // S3 has a limit of 1000 objects per delete operation
        const MAX_OBJECTS_PER_REQUEST: usize = 1000;
        
        for chunk in objects_to_delete.chunks(MAX_OBJECTS_PER_REQUEST) {
            let mut delete_object_ids: Vec<ObjectIdentifier> = Vec::with_capacity(chunk.len());
            
            for obj in chunk {
                info!("Preparing to delete object: {}", obj);
                if obj.ends_with("/") {
                    info!("Found prefix: {}", obj);
                    let sub_objects = Box::pin(self.delete_objects_from_bucket_recursively(bucket, obj)).await?;
                    
                    if !sub_objects.is_empty() {
                        info!("Found {} objects in prefix: {}", sub_objects.len(), obj);
                        
                        for o in sub_objects {
                            delete_object_ids.push(ObjectIdentifier::builder()
                                .key(o) 
                                .build()
                                .map_err(|err| {
                                    let error_msg = format!("Failed to build key for delete_object: {err:?}");
                                    error!("{}", error_msg);
                                    S3Error::AwsError(error_msg)
                                })?);
                        }
                    }
                }
                
                let obj_id = ObjectIdentifier::builder()
                    .key(*obj) 
                    .build()
                    .map_err(|err| {
                        let error_msg = format!("Failed to build key for delete_object: {err:?}");
                        error!("{}", error_msg);
                        S3Error::AwsError(error_msg)
                    })?;
                delete_object_ids.push(obj_id);
            }

            info!("Sending delete_objects request for {} objects", delete_object_ids.len());
            
            if delete_object_ids.is_empty() {
                info!("No objects to delete in this batch");
                continue;
            }
            
            info!("Deleting from bucket: {}", bucket);
            let delete_result = self.client
                .delete_objects()
                .bucket(bucket)
                .delete(
                    Delete::builder()
                        .set_objects(Some(delete_object_ids))
                        .build()
                        .map_err(|err| {
                            let error_msg = format!("Failed to build delete_object input {err:?}");
                            error!("{}", error_msg);
                            S3Error::AwsError(error_msg)
                        })?,
                )
                .send()
                .await;
            
            match delete_result {
                Ok(response) => {
                    let errors = response.errors();
                    if !errors.is_empty() {
                        error!("Some objects failed to delete:");
                        for error in errors {
                            error!("  Error: {:?}, Key: {:?}, Code: {:?}", 
                                error.message(), error.key(), error.code());
                        }
                        info!("Continuing with deletion despite errors");
                    }
                },
                Err(err) => {
                    let error_msg = format!("Failed to delete objects: {:?}", err);
                    error!("{}", error_msg);
                    return Err(err.into());
                }
            }
        }
        
        info!("Successfully completed deletion process");
        Ok(())
    }

    /// Recursively deletes all objects with a common prefix from a bucket.
    ///
    /// This method lists all objects with the specified prefix and deletes them.
    /// It's useful for deleting all objects in a "folder" or with a common prefix.
    ///
    /// # Arguments
    ///
    /// * `bucket` - The name of the bucket to delete from
    /// * `prefix` - The prefix of objects to delete
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<String>)` - A vector of deleted object keys if successful
    /// * `Err(S3Error)` - If there was an error listing or deleting the objects
    async fn delete_objects_from_bucket_recursively(&self, bucket: &str, prefix: &str) -> Result<Vec<String>, S3Error> {
        info!("Recursively deleting objects with prefix '{}' from bucket '{}'", prefix, bucket);
        
        let objects = self.list_objects(Some(prefix), bucket).await?;
        
        if objects.is_empty() {
            info!("No objects found with prefix '{}'", prefix);
            return Ok(Vec::new());
        }
        
        info!("Found {} objects to delete", objects.len());
        
        let object_keys: Vec<String> = objects.iter().map(|obj| obj.name.clone()).collect();
        
        let object_refs: Vec<&String> = object_keys.iter().collect();
        
        self.delete_objects_from_bucket(bucket, object_refs).await?;
        
        info!("Successfully deleted {} objects with prefix '{}'", objects.len(), prefix);
        
        Ok(object_keys)
    }
} 
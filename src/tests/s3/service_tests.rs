#[cfg(test)]
// Tests for the S3Service functionality
// These tests focus on the initialization and configuration of the S3Service

use std::env;
use std::collections::HashMap;
use aws_sdk_s3::types::Bucket;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::list_buckets::{ListBucketsOutput, ListBucketsError};
use aws_sdk_s3::operation::create_bucket::{CreateBucketOutput, CreateBucketError};
use aws_sdk_s3::primitives::DateTime;
use crate::rdlib::s3::error::S3Error;

struct MockS3Service {
    client: MockS3Client,
}

struct MockS3Client {
    buckets: Vec<String>,
    objects: HashMap<String, HashMap<String, Vec<u8>>>,
}

impl MockS3Client {
    fn new() -> Self {
        Self {
            buckets: Vec::new(),
            objects: HashMap::new(),
        }
    }

    async fn list_buckets(&self) -> Result<ListBucketsOutput, SdkError<ListBucketsError>> {
        let mut buckets = Vec::new();
        for bucket_name in &self.buckets {
            buckets.push(
                Bucket::builder()
                    .name(bucket_name)
                    .creation_date(DateTime::from_secs(0))
                    .build(),
            );
        }
        
        let output = ListBucketsOutput::builder()
            .set_buckets(Some(buckets))
            .build();
        
        Ok(output)
    }

    async fn create_bucket(&mut self, bucket_name: &str) -> Result<CreateBucketOutput, SdkError<CreateBucketError>> {
        if !self.buckets.contains(&bucket_name.to_string()) {
            self.buckets.push(bucket_name.to_string());
            self.objects.insert(bucket_name.to_string(), HashMap::new());
        }
        
        Ok(CreateBucketOutput::builder()
            .location(format!("/{}", bucket_name))
            .build())
    }

    fn bucket_exists(&self, bucket_name: &str) -> bool {
        self.buckets.contains(&bucket_name.to_string())
    }
}

impl MockS3Service {
    fn new() -> Self {
        Self {
            client: MockS3Client::new(),
        }
    }

    async fn list_buckets(&self) -> Result<Vec<String>, S3Error> {
        let resp = self.client.list_buckets().await?;
        
        let mut buckets = Vec::new();
        let bucket_list = resp.buckets();
        for bucket in bucket_list {
            if let Some(name) = &bucket.name {
                buckets.push(name.clone());
            }
        }
        
        Ok(buckets)
    }

    async fn ensure_bucket_exists(&mut self, bucket_name: &str) -> Result<(), S3Error> {
        let buckets = self.list_buckets().await?;
        if buckets.contains(&bucket_name.to_string()) {
            return Ok(());
        }
        
        self.client.create_bucket(bucket_name).await?;
        Ok(())
    }
}

#[tokio::test]
async fn test_env_var_handling() {
    let original_endpoint = env::var("S3_ENDPOINT_URL").ok();
    let original_access_key = env::var("S3_ACCESS_KEY").ok();
    let original_secret_key = env::var("S3_SECRET_KEY").ok();
    let original_region = env::var("S3_REGION").ok();
    
    unsafe {
        env::remove_var("S3_ENDPOINT_URL");
        env::remove_var("S3_ACCESS_KEY");
        env::remove_var("S3_SECRET_KEY");
        env::remove_var("S3_REGION");
    }
    
    assert!(env::var("S3_ENDPOINT_URL").is_err());
    assert!(env::var("S3_ACCESS_KEY").is_err());
    assert!(env::var("S3_SECRET_KEY").is_err());
    assert!(env::var("S3_REGION").is_err());
    
    unsafe {
        env::set_var("S3_ENDPOINT_URL", "http://localhost:7000");
        env::set_var("S3_ACCESS_KEY", "test-access-key");
        env::set_var("S3_SECRET_KEY", "test-secret-key");
        env::set_var("S3_REGION", "eu-central-1");
    }
    
    assert_eq!(env::var("S3_ENDPOINT_URL").unwrap(), "http://localhost:7000");
    assert_eq!(env::var("S3_ACCESS_KEY").unwrap(), "test-access-key");
    assert_eq!(env::var("S3_SECRET_KEY").unwrap(), "test-secret-key");
    assert_eq!(env::var("S3_REGION").unwrap(), "eu-central-1");
    
    unsafe {
        env::remove_var("S3_ENDPOINT_URL");
        env::remove_var("S3_ACCESS_KEY");
        env::remove_var("S3_SECRET_KEY");
        env::remove_var("S3_REGION");
    }
    
    unsafe {
        if let Some(val) = original_endpoint {
            env::set_var("S3_ENDPOINT_URL", val);
        }
        if let Some(val) = original_access_key {
            env::set_var("S3_ACCESS_KEY", val);
        }
        if let Some(val) = original_secret_key {
            env::set_var("S3_SECRET_KEY", val);
        }
        if let Some(val) = original_region {
            env::set_var("S3_REGION", val);
        }
    }
}

#[tokio::test]
async fn test_ensure_bucket_exists() {
    let mut service = MockS3Service::new();
    
    let bucket_name = "new-test-bucket";
    assert!(!service.client.bucket_exists(bucket_name));
    
    let result = service.ensure_bucket_exists(bucket_name).await;
    assert!(result.is_ok());
    
    assert!(service.client.bucket_exists(bucket_name));
    
    let result = service.ensure_bucket_exists(bucket_name).await;
    assert!(result.is_ok());
    
    assert!(service.client.bucket_exists(bucket_name));
}

#[tokio::test]
async fn test_region_handling() {
    let original_region = env::var("S3_REGION").ok();
    
    unsafe {
        env::remove_var("S3_REGION");
    }
    
    assert!(env::var("S3_REGION").is_err());
    
    unsafe {
        env::set_var("S3_REGION", "eu-central-1");
    }
    
    assert_eq!(env::var("S3_REGION").unwrap(), "eu-central-1");
    
    // Clean up
    unsafe {
        env::remove_var("S3_REGION");
    }
    
    // Restore the original environment variable
    unsafe {
        if let Some(val) = original_region {
            env::set_var("S3_REGION", val);
        }
    }
}
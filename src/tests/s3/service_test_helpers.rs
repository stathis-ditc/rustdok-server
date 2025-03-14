//! # S3 Service Test Helpers
//! 
//! This module provides test-specific helpers for the S3Service.
//! It is only included when running tests.

use aws_sdk_s3::Client;
use crate::rdlib::s3::service::S3Service;

impl S3Service {
    /// Creates a new S3Service instance with a provided client.
    /// 
    /// This constructor is only available in test mode and allows for injecting
    /// a mock client for testing purposes.
    /// 
    /// # Arguments
    /// 
    /// * `client` - The AWS S3 client to use
    /// 
    /// # Returns
    /// 
    /// A new `S3Service` instance configured with the specified parameters.
    #[allow(dead_code)]
    pub fn new_with_client(client: Client) -> Self {
        Self { 
            client,
        }
    }
} 
//! # Bucket Validation
//! 
//! This module provides functionality for validating S3 bucket names.
//! It includes methods for checking if bucket names conform to S3 naming rules
//! and for ensuring buckets exist.

use crate::rdlib::s3::service::S3Service;

impl S3Service {
    /// Validates a bucket name according to S3 naming rules.
    ///
    /// This method checks if a bucket name conforms to the S3 naming conventions:
    /// - Between 3 and 63 characters long
    /// - Contains only lowercase letters, numbers, periods, and hyphens
    /// - Starts and ends with a letter or number
    /// - Does not contain two adjacent periods
    /// - Is not formatted as an IP address
    /// - Does not start with the prefix "xn--"
    /// - Does not end with the suffix "-s3alias"
    ///
    /// # Arguments
    ///
    /// * `name` - The bucket name to validate
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the bucket name is valid
    /// * `Err(String)` - A description of why the bucket name is invalid
    pub fn validate_bucket_name(name: &str) -> Result<(), String> {
        if name.len() < 3 || name.len() > 63 {
            return Err(format!("Bucket name must be between 3 and 63 characters long. Got {} characters.", name.len()));
        }
        if !name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '-') {
            return Err("Bucket name can only contain lowercase letters, numbers, periods (.), and hyphens (-)".to_string());
        }

        let first_char = name.chars().next().unwrap();
        let last_char = name.chars().last().unwrap();
        if !(first_char.is_ascii_lowercase() || first_char.is_ascii_digit()) {
            return Err("Bucket name must begin with a letter or number".to_string());
        }
        if !(last_char.is_ascii_lowercase() || last_char.is_ascii_digit()) {
            return Err("Bucket name must end with a letter or number".to_string());
        }

        if name.contains("..") {
            return Err("Bucket name must not contain two adjacent periods".to_string());
        }

        if name.split('.').count() == 4 && name.split('.').all(|segment| {
            segment.parse::<u8>().is_ok()
        }) {
            return Err("Bucket name must not be formatted as an IP address".to_string());
        }

        if name.starts_with("xn--") {
            return Err("Bucket name must not start with the prefix 'xn--'".to_string());
        }
        if name.ends_with("-s3alias") {
            return Err("Bucket name must not end with the suffix '-s3alias'".to_string());
        }

        Ok(())
    }
}
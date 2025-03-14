//! # S3 Error Handling
//! 
//! This module provides error types and conversion implementations for
//! handling errors that occur during S3 operations. It defines a custom
//! error type `S3Error` and implements conversions from various AWS SDK
//! error types to this custom type.

use aws_sdk_s3::{Error as AwsS3Error};
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::create_bucket::CreateBucketError;
use aws_sdk_s3::operation::delete_bucket::DeleteBucketError;
use aws_sdk_s3::operation::list_buckets::ListBucketsError;
use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Error;
use aws_sdk_s3::operation::put_object::PutObjectError;
use aws_sdk_s3::operation::delete_object::DeleteObjectError;
use aws_sdk_s3::operation::delete_objects::DeleteObjectsError;
use aws_sdk_s3::operation::head_object::HeadObjectError;
use aws_sdk_s3::operation::get_object::GetObjectError;
use aws_sdk_s3::operation::copy_object::CopyObjectError;
use aws_sdk_s3::primitives::ByteStreamError;
use std::fmt;

/// Custom error type for S3 operations.
///
/// This enum represents the various types of errors that can occur
/// during S3 operations. It provides a unified error type for the
/// application to handle.
#[derive(Debug, Clone)]
pub enum S3Error {
    /// An error from the AWS S3 SDK
    AwsError(String),
    /// Error when attempting to create a bucket that already exists
    BucketAlreadyExists(String),
    /// Error when a bucket is not found. Used only in testing so surpressing the warning
    #[allow(dead_code)]
    BucketNotFound(String),
    /// Other miscellaneous errors
    Other(String),
}

impl S3Error {
    /// Check if the error message contains a specific string.
    ///
    /// This method is useful for checking the type of error when
    /// the specific error variant is not known.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to check for in the error message
    ///
    /// # Returns
    ///
    /// `true` if the error message contains the specified string, `false` otherwise
    pub fn contains(&self, s: &str) -> bool {
        match self {
            S3Error::AwsError(msg) => msg.contains(s),
            S3Error::BucketAlreadyExists(msg) => msg.contains(s),
            S3Error::BucketNotFound(msg) => msg.contains(s),
            S3Error::Other(msg) => msg.contains(s),
        }
    }
}

impl fmt::Display for S3Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S3Error::AwsError(e) => write!(f, "AWS S3 Error: {}", e),
            S3Error::BucketAlreadyExists(name) => write!(f, "Bucket '{}' already exists", name),
            S3Error::BucketNotFound(name) => write!(f, "Bucket '{}' not found", name),
            S3Error::Other(e) => write!(f, "S3 Error: {}", e),
        }
    }
}

impl std::error::Error for S3Error {}

impl From<AwsS3Error> for S3Error {
    fn from(err: AwsS3Error) -> Self {
        S3Error::AwsError(err.to_string())
    }
}

// Specific implementations for SdkError types
impl From<SdkError<CreateBucketError>> for S3Error {
    fn from(err: SdkError<CreateBucketError>) -> Self {
        S3Error::AwsError(format!("{:?}", err))
    }
}

impl From<SdkError<ListBucketsError>> for S3Error {
    fn from(err: SdkError<ListBucketsError>) -> Self {
        S3Error::AwsError(format!("{:?}", err))
    }
}

impl From<SdkError<ListObjectsV2Error>> for S3Error {
    fn from(err: SdkError<ListObjectsV2Error>) -> Self {
        S3Error::AwsError(format!("{:?}", err))
    }
}

impl From<SdkError<PutObjectError>> for S3Error {
    fn from(err: SdkError<PutObjectError>) -> Self {
        S3Error::AwsError(format!("{:?}", err))
    }
}

impl From<SdkError<DeleteObjectError>> for S3Error {
    fn from(err: SdkError<DeleteObjectError>) -> Self {
        S3Error::AwsError(format!("{:?}", err))
    }
}

impl From<SdkError<HeadObjectError>> for S3Error {
    fn from(err: SdkError<HeadObjectError>) -> Self {
        S3Error::AwsError(format!("{:?}", err))
    }
}

impl From<SdkError<GetObjectError>> for S3Error {
    fn from(err: SdkError<GetObjectError>) -> Self {
        S3Error::AwsError(format!("{:?}", err))
    }
}

impl From<SdkError<DeleteBucketError>> for S3Error {
    fn from(err: SdkError<DeleteBucketError>) -> Self {
        S3Error::AwsError(format!("{:?}", err))
    }
}

impl From<SdkError<CopyObjectError>> for S3Error {
    fn from(err: SdkError<CopyObjectError>) -> Self {
        S3Error::AwsError(format!("{:?}", err))
    }
}

impl From<SdkError<DeleteObjectsError>> for S3Error {
    fn from(err: SdkError<DeleteObjectsError>) -> Self {
        S3Error::AwsError(format!("{:?}", err))
    }
}

impl From<ByteStreamError> for S3Error {
    fn from(err: ByteStreamError) -> Self {
        S3Error::AwsError(format!("ByteStream Error: {:?}", err))
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for S3Error {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        S3Error::Other(err.to_string())
    }
} 
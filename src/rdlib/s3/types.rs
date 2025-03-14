//! # S3 Types
//! 
//! This module defines the data structures used for S3 operations.
//! It includes types for representing S3 objects and their metadata.

use serde::{Serialize, Deserialize};

/// Represents an object in an S3 bucket.
///
/// This structure contains metadata about an S3 object, including its name (key),
/// size, and last modified timestamp.
#[derive(Debug, Serialize, Deserialize)]
pub struct S3Object {
    /// The name (key) of the object
    pub name: String,
    /// The size of the object in bytes
    pub size: u64,
    /// The last modified timestamp of the object in RFC3339 format
    pub last_modified: Option<String>,
} 
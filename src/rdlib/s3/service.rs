//! # S3 Service
//! 
//! This module provides the core functionality for interacting with S3-compatible
//! storage services. It handles the configuration and initialization of the AWS S3 client
//! and provides methods for bucket and object operations.

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, config::{Builder,Region, Credentials}};
use std::env;
use log::info;
use once_cell::sync::OnceCell;

// Global static S3 client
static S3_CLIENT: OnceCell<Client> = OnceCell::new();

/// Initialize the global S3 client
///
/// This function initializes the AWS S3 client with configuration from environment
/// variables and stores it in a global static variable. It should be called once
/// at application startup.
///
/// # Environment Variables
///
/// * `S3_ENDPOINT_URL` - The URL of the S3-compatible service (required)
/// * `S3_REGION` - The AWS region (optional for Ceph)
/// * `S3_ACCESS_KEY` - The access key for authentication (required)
/// * `S3_SECRET_KEY` - The secret key for authentication (required)
///
/// # Returns
///
/// A reference to the initialized S3 client
pub async fn init_s3_client() -> &'static Client {
    if let Some(client) = S3_CLIENT.get() {
        return client;
    }

    let endpoint_url = env::var("S3_ENDPOINT_URL")
        .unwrap_or_else(|_| {
            if cfg!(test) {
                "http://localhost:7000".to_string()
            } else {
                panic!("S3_ENDPOINT_URL must be set")
            }
        });
    info!("S3 Endpoint URL: {}", endpoint_url);
    
    // Region is optional for some S3-compatible services
    let region = env::var("S3_REGION").ok();
    info!("S3 Region: {:?}", region);
    
    let access_key = env::var("S3_ACCESS_KEY")
        .unwrap_or_else(|_| {
            if cfg!(test) {
                "test-access-key".to_string()
            } else {
                panic!("S3_ACCESS_KEY must be set")
            }
        });
    
    let secret_key = env::var("S3_SECRET_KEY")
        .unwrap_or_else(|_| {
            if cfg!(test) {
                "test-secret-key".to_string()
            } else {
                panic!("S3_SECRET_KEY must be set")
            }
        });

    let mut config_builder = aws_config::defaults(aws_config::BehaviorVersion::latest());

    if let Some(region_str) = region {
        let region_provider = RegionProviderChain::first_try(Region::new(region_str));
        config_builder = config_builder.region(region_provider);
    }
    
    let config = config_builder
        .endpoint_url(&endpoint_url)
        .credentials_provider(Credentials::new(
            access_key,
            secret_key,
            None,
            None,
            "s3-credentials",
        ))
        .load()
        .await;

    info!("S3 config created");

    // Create S3 client with path-style access for Ceph compatibility
    let s3_config = Builder::from(&config)
        .force_path_style(true)
        .build();
        
    let client = Client::from_conf(s3_config);
    info!("S3 client created");

    match S3_CLIENT.set(client) {
        Ok(_) => {},
        Err(_) => {
            info!("S3 client already initialized");
        }
    }

    S3_CLIENT.get().unwrap()
}

/// S3 Service for interacting with S3-compatible storage
///
/// This struct provides methods for bucket and object operations
/// using the AWS S3 client.
pub struct S3Service {
    /// The AWS S3 client used for making API calls
    pub client: Client,
}

impl S3Service {
    /// Creates a new S3Service instance
    ///
    /// This method initializes a new S3Service with the global S3 client.
    /// The global client must be initialized before calling this method.
    ///
    /// # Returns
    ///
    /// A new S3Service instance
    pub async fn new() -> Self {
        let client = S3_CLIENT.get().expect("S3 client not initialized");
        Self { client: client.clone() }
    }
} 
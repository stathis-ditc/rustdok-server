//! # API Configuration
//! 
//! This module contains the configuration for the RustDok API.
//! It defines the API routes and their organization.

use actix_web::{web, Scope};

/// Configure API v1 routes
///
/// This function creates and returns a Scope for the v1 API endpoints.
/// It registers all the bucket and object related routes with their
/// respective handler functions.
///
/// # Returns
///
/// * `Scope` - An Actix Web Scope configured with all v1 API routes
pub fn configure_api_v1() -> Scope {
    web::scope("/api/v1")
        // Bucket routes
        .service(crate::api::v1::buckets::list_buckets)
        .service(crate::api::v1::buckets::create_bucket)
        .service(crate::api::v1::buckets::delete_bucket)
        // Object routes
        .service(crate::api::v1::objects::list_objects_in_bucket)
        .service(crate::api::v1::objects::download_object_from_bucket)
        .service(crate::api::v1::objects::view_object_from_bucket)
        .service(crate::api::v1::objects::upload_object_to_bucket)
        .service(crate::api::v1::objects::delete_object_from_bucket)
        .service(crate::api::v1::objects::create_folder)
        .service(crate::api::v1::objects::check_object_exists_in_bucket)
} 
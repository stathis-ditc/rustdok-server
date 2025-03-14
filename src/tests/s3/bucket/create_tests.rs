#[cfg(test)]
use crate::rdlib::s3::service::S3Service;

#[test]
fn test_validate_bucket_name() {

    let result = S3Service::validate_bucket_name("bucket1");
    assert!(result.is_ok(), "Valid bucket name should pass validation");

    let result = S3Service::validate_bucket_name("b1");
    assert!(result.is_err(), "Bucket name that's too short should fail validation");
    if let Err(error_msg) = result {
        assert!(error_msg.contains("between 3 and 63 characters"), 
                "Error message should mention length requirements");
    }
    
    let long_name = "a".repeat(64);
    let result = S3Service::validate_bucket_name(&long_name);
    assert!(result.is_err(), "Bucket name that's too long should fail validation");
    if let Err(error_msg) = result {
        assert!(error_msg.contains("between 3 and 63 characters"), 
                "Error message should mention length requirements");
    }
    
    let result = S3Service::validate_bucket_name("Bucket1");
    assert!(result.is_err(), "Bucket name with invalid characters should fail validation");
    if let Err(error_msg) = result {
        assert!(error_msg.contains("can only contain lowercase"), 
                "Error message should mention character requirements");
    }
    
    let result = S3Service::validate_bucket_name("-bucket1");
    assert!(result.is_err(), "Bucket name with invalid start should fail validation");
    if let Err(error_msg) = result {
        assert!(error_msg.contains("must begin with a letter or number"), 
                "Error message should mention start requirements");
    }

    let result = S3Service::validate_bucket_name("bucket1-");
    assert!(result.is_err(), "Bucket name with invalid end should fail validation");
    if let Err(error_msg) = result {
        assert!(error_msg.contains("must end with a letter or number"), 
                "Error message should mention end requirements");
    }
} 
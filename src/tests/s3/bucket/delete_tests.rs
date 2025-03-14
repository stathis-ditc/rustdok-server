#[cfg(test)]

#[test]
fn test_delete_bucket_validation() {

    let bucket_name = "";
    assert!(!is_valid_bucket_name(bucket_name), "Empty bucket name should be invalid");
    
    let bucket_name = "bucket1";
    assert!(is_valid_bucket_name(bucket_name), "Valid bucket name should be valid");
}

#[cfg(test)]
fn is_valid_bucket_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= 63
} 
#[cfg(test)]

#[test]
fn test_object_key_validation() {

    let valid_keys = vec![
        "test.pdf",
        "images/test.jpg",
        "backups/test.zip",
        "test-with-dash",
        "test_with_underscores",
        "test.with.dots",
    ];
    
    for key in valid_keys {
        assert!(is_valid_object_key(key), "Key '{}' should be valid", key);
    }
    
    let invalid_keys = vec![
        "",
        "//double-slashes",
        "test-with-trailing-slash/",
        "../test",
        "test-with-\0-null-char",
    ];
    
    for key in invalid_keys {
        assert!(!is_valid_object_key(key), "Key '{}' should be invalid", key);
    }
}

#[cfg(test)]
fn is_valid_object_key(key: &str) -> bool {
    if key.is_empty() {
        return false;
    }
    
    if key.contains("//") {
        return false;
    }
    
    if key.ends_with('/') {
        return false;
    }
    
    if key.contains("..") {
        return false;
    }
    
    if key.contains('\0') {
        return false;
    }
    
    true
} 
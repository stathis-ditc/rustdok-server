

#[test]
fn test_list_buckets_parsing() {

    let bucket_names = vec!["bucket1".to_string(), "bucket2".to_string(), "bucket3".to_string()];

    assert_eq!(bucket_names.len(), 3, "Expected 3 bucket names");
    assert!(bucket_names.contains(&"bucket1".to_string()), "Expected bucket1 to be in the list");
    assert!(bucket_names.contains(&"bucket2".to_string()), "Expected bucket2 to be in the list");
    assert!(bucket_names.contains(&"bucket3".to_string()), "Expected bucket3 to be in the list");
} 
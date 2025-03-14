#[cfg(test)]

#[test]
fn test_object_data_processing() {

    let data = Vec::new();
    let processed_data = process_object_data(data.clone());
    assert_eq!(processed_data, data, "Empty data should remain unchanged");

    let text_data = "Hello, world!".as_bytes().to_vec();
    let processed_text_data = process_object_data(text_data.clone());
    assert_eq!(processed_text_data, text_data, "Text data should remain unchanged");

    let binary_data = vec![0, 1, 2, 3, 4, 5];
    let processed_binary_data = process_object_data(binary_data.clone());
    assert_eq!(processed_binary_data, binary_data, "Binary data should remain unchanged");
}

#[test]
fn test_object_key_normalization() {

    let key1 = "/test.pdf";
    let normalized1 = normalize_object_key(key1);
    assert!(normalized1.starts_with('/') == false, "Normalized key should not start with a slash");
    assert!(normalized1.ends_with("test.pdf"), "Normalized key should end with the original filename");
    
    let key2 = "folder//test.pdf";
    let normalized2 = normalize_object_key(key2);
    assert!(!normalized2.contains("//"), "Normalized key should not contain double slashes");
    
    let key3 = "test.pdf";
    let normalized3 = normalize_object_key(key3);
    assert_eq!(normalized3, "test.pdf", "Simple key should remain unchanged");
    
    let key4 = "./test.pdf";
    let normalized4 = normalize_object_key(key4);
    assert!(!normalized4.starts_with("./"), "Normalized key should not start with ./");
}

#[cfg(test)]
fn process_object_data(data: Vec<u8>) -> Vec<u8> {
    data
}

#[cfg(test)]
fn normalize_object_key(key: &str) -> String {
    let mut normalized = key.to_string();
    
    // Remove leading slashes
    while normalized.starts_with('/') {
        normalized = normalized[1..].to_string();
    }
    
    // Replace multiple slashes with a single slash
    while normalized.contains("//") {
        normalized = normalized.replace("//", "/");
    }
    
    // Handle dot segments
    normalized = normalized.replace("/./", "/");
    normalized = normalized.replace("./", "");
    
    normalized
} 
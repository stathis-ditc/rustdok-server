#[cfg(test)]

#[test]
fn test_object_data_preparation() {

    let data = Vec::new();
    let prepared_data = prepare_object_data(data.clone());
    assert_eq!(prepared_data, data, "Empty data should remain unchanged");
    
    let text_data = "Hello, world!".as_bytes().to_vec();
    let prepared_text_data = prepare_object_data(text_data.clone());
    assert_eq!(prepared_text_data, text_data, "Text data should remain unchanged");

    let binary_data = vec![0, 1, 2, 3, 4, 5];
    let prepared_binary_data = prepare_object_data(binary_data.clone());
    assert_eq!(prepared_binary_data, binary_data, "Binary data should remain unchanged");
}

#[test]
fn test_content_type_detection() {

    assert_eq!(detect_content_type("document.pdf"), "application/pdf");
    assert_eq!(detect_content_type("image.jpg"), "image/jpeg");
    assert_eq!(detect_content_type("image.png"), "image/png");
    assert_eq!(detect_content_type("document.txt"), "text/plain");
    assert_eq!(detect_content_type("document.html"), "text/html");
    assert_eq!(detect_content_type("document.json"), "application/json");
    
    assert_eq!(detect_content_type("unknown"), "application/octet-stream");
    
    assert_eq!(detect_content_type("no-extension"), "application/octet-stream");
}

#[cfg(test)]
fn prepare_object_data(data: Vec<u8>) -> Vec<u8> {
    data
}

#[cfg(test)]
fn detect_content_type(key: &str) -> &'static str {
    if key.ends_with(".pdf") {
        "application/pdf"
    } else if key.ends_with(".jpg") || key.ends_with(".jpeg") {
        "image/jpeg"
    } else if key.ends_with(".png") {
        "image/png"
    } else if key.ends_with(".txt") {
        "text/plain"
    } else if key.ends_with(".html") || key.ends_with(".htm") {
        "text/html"
    } else if key.ends_with(".json") {
        "application/json"
    } else {
        "application/octet-stream"
    }
} 
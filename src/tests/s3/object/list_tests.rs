#[cfg(test)]
#[test]
fn test_object_listing_filtering() {

    let objects = vec![
        "documents/report.pdf".to_string(),
        "documents/presentation.pptx".to_string(),
        "images/photo1.jpg".to_string(),
        "images/photo2.jpg".to_string(),
        "images/photo3.png".to_string(),
        "backups/backup1.zip".to_string(),
        "backups/backup2.zip".to_string(),
    ];
    
    let documents = filter_objects(&objects, Some("documents/"), None);
    assert_eq!(documents.len(), 2, "Should find 2 documents");
    assert!(documents.contains(&"documents/report.pdf".to_string()));
    assert!(documents.contains(&"documents/presentation.pptx".to_string()));
    
    let images = filter_objects(&objects, Some("images/"), None);
    assert_eq!(images.len(), 3, "Should find 3 images");
    
    let top_level = filter_objects(&objects, None, Some("/"));
    assert_eq!(top_level.len(), 3, "Should find 3 top-level prefixes");
    assert!(top_level.contains(&"documents/".to_string()));
    assert!(top_level.contains(&"images/".to_string()));
    assert!(top_level.contains(&"backups/".to_string()));
    
    let image_files = filter_objects(&objects, Some("images/"), Some("/"));
    assert_eq!(image_files.len(), 3, "Should find 3 image files");
}

#[test]
fn test_object_listing_pagination() {

    let objects = (1..=20).map(|i| format!("object{}.txt", i)).collect::<Vec<_>>();
    
    let (page1, next_token1) = paginate_objects(&objects, None, 5);
    assert_eq!(page1.len(), 5, "First page should have 5 objects");
    assert_eq!(page1[0], "object1.txt");
    assert_eq!(page1[4], "object5.txt");
    assert!(next_token1.is_some(), "Should have a next token");
    
    let (page2, next_token2) = paginate_objects(&objects, next_token1, 5);
    assert_eq!(page2.len(), 5, "Second page should have 5 objects");
    assert_eq!(page2[0], "object6.txt");
    assert_eq!(page2[4], "object10.txt");
    assert!(next_token2.is_some(), "Should have a next token");
    
    let (page4, next_token4) = paginate_objects(&objects, Some("15".to_string()), 5);
    assert_eq!(page4.len(), 5, "Last page should have 5 objects");
    assert_eq!(page4[0], "object16.txt");
    assert_eq!(page4[4], "object20.txt");
    assert!(next_token4.is_none(), "Should not have a next token");

    let (page_large, next_token_large) = paginate_objects(&objects, Some("15".to_string()), 10);
    assert_eq!(page_large.len(), 5, "Page should have 5 objects");
    assert!(next_token_large.is_none(), "Should not have a next token");
}

#[cfg(test)]
fn filter_objects(objects: &[String], prefix: Option<&str>, delimiter: Option<&str>) -> Vec<String> {
    let mut filtered = Vec::new();
    
    if let Some(prefix) = prefix {
        // Filter by prefix
        for object in objects {
            if object.starts_with(prefix) {
                filtered.push(object.clone());
            }
        }
    } else if let Some(delimiter) = delimiter {
        // Simulate folder-like behavior with delimiter
        let mut prefixes = std::collections::HashSet::new();
        
        for object in objects {
            if let Some(pos) = object.find(delimiter) {
                let prefix = format!("{}{}", &object[0..=pos], "");
                prefixes.insert(prefix);
            }
        }
        
        filtered = prefixes.into_iter().collect();
    } else {
        filtered = objects.to_vec();
    }
    
    filtered
}

#[cfg(test)]
fn paginate_objects(objects: &[String], continuation_token: Option<String>, max_keys: usize) -> (Vec<String>, Option<String>) {
    let start_index = if let Some(token) = continuation_token {
        token.parse::<usize>().unwrap_or(0)
    } else {
        0
    };
    
    let end_index = std::cmp::min(start_index + max_keys, objects.len());
    
    let page = objects[start_index..end_index].to_vec();
    
    let next_token = if end_index < objects.len() {
        Some(end_index.to_string())
    } else {
        None
    };
    
    (page, next_token)
} 
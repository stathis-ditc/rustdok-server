#[cfg(test)]


#[test]
fn test_single_object_deletion() {

    let mut object_store = MockObjectStore::new();
    object_store.add_object("bucket1", "test.pdf");
    object_store.add_object("bucket1", "test.jpg");
    
    let result = object_store.delete_object("bucket1", "test.pdf");
    assert!(result.is_ok(), "Object deletion should succeed");

    assert!(!object_store.object_exists("bucket1", "test.pdf"), "Object should be deleted");
    assert!(object_store.object_exists("bucket1", "test.jpg"), "Other objects should remain");
    
    let result = object_store.delete_object("bucket1", "nonexistent.txt");
    assert!(result.is_err(), "Deleting a non-existent object should fail");

    let result = object_store.delete_object("nonexistent-bucket", "test.pdf");
    assert!(result.is_err(), "Deleting from a non-existent bucket should fail");
}

#[test]
fn test_multiple_object_deletion() {

    let mut object_store = MockObjectStore::new();
    object_store.add_object("bucket1", "test1.pdf");
    object_store.add_object("bucket1", "test2.pdf");
    object_store.add_object("bucket1", "test1.jpg");
    object_store.add_object("bucket1", "test2.jpg");
    
    let keys = vec!["test1.pdf".to_string(), "test2.pdf".to_string()];
    let result = object_store.delete_objects("bucket1", &keys);
    
    assert!(result.is_ok(), "Multiple object deletion should succeed");
    let (deleted, errors) = result.unwrap();
    assert_eq!(deleted.len(), 2, "Should have deleted 2 objects");
    assert_eq!(errors.len(), 0, "Should have no errors");
    
    assert!(!object_store.object_exists("bucket1", "test1.pdf"), "Object should be deleted");
    assert!(!object_store.object_exists("bucket1", "test2.pdf"), "Object should be deleted");
    assert!(object_store.object_exists("bucket1", "test1.jpg"), "Other objects should remain");
    assert!(object_store.object_exists("bucket1", "test2.jpg"), "Other objects should remain");
    
    let keys = vec!["test1.jpg".to_string(), "nonexistent.txt".to_string()];
    let result = object_store.delete_objects("bucket1", &keys);
    
    assert!(result.is_ok(), "Partial deletion should succeed");
    let (deleted, errors) = result.unwrap();
    assert_eq!(deleted.len(), 1, "Should have deleted 1 object");
    assert_eq!(errors.len(), 1, "Should have 1 error");
}

// Mock object store for testing
#[cfg(test)]
struct MockObjectStore {
    objects: std::collections::HashMap<String, std::collections::HashSet<String>>,
}

#[cfg(test)]
impl MockObjectStore {
    fn new() -> Self {
        Self {
            objects: std::collections::HashMap::new(),
        }
    }
    
    fn add_object(&mut self, bucket: &str, key: &str) {
        let bucket_objects = self.objects.entry(bucket.to_string()).or_insert_with(std::collections::HashSet::new);
        bucket_objects.insert(key.to_string());
    }
    
    fn object_exists(&self, bucket: &str, key: &str) -> bool {
        if let Some(bucket_objects) = self.objects.get(bucket) {
            bucket_objects.contains(key)
        } else {
            false
        }
    }
    
    fn delete_object(&mut self, bucket: &str, key: &str) -> Result<(), String> {
        if let Some(bucket_objects) = self.objects.get_mut(bucket) {
            if bucket_objects.remove(key) {
                Ok(())
            } else {
                Err(format!("Object '{}' not found in bucket '{}'", key, bucket))
            }
        } else {
            Err(format!("Bucket '{}' not found", bucket))
        }
    }
    
    fn delete_objects(&mut self, bucket: &str, keys: &[String]) -> Result<(Vec<String>, Vec<(String, String)>), String> {
        if !self.objects.contains_key(bucket) {
            return Err(format!("Bucket '{}' not found", bucket));
        }
        
        let mut deleted = Vec::new();
        let mut errors = Vec::new();
        
        for key in keys {
            match self.delete_object(bucket, key) {
                Ok(()) => deleted.push(key.clone()),
                Err(e) => errors.push((key.clone(), e)),
            }
        }
        
        Ok((deleted, errors))
    }
} 
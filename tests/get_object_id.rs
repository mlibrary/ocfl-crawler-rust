use ocfl_crawler_rust::get_object_id;
use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn get_object_id_from_inventory_json() {
    let tmp_root = Path::new("tests/tmp");
    // Ensure tests/tmp exists
    fs::create_dir_all(tmp_root).expect("failed to create tests/tmp directory");

    // Create a unique subdirectory to avoid test collisions
    let unique = format!(
        "is_object_root_inventory_{}_{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos()
    );
    let dir = tmp_root.join(unique);

    if dir.exists() {
        let _ = fs::remove_dir_all(&dir);
    }

    fs::create_dir(&dir).expect("failed to create test directory");

    // Add object marker
    let marker = dir.join("0=ocfl_object_1.1");
    fs::write(&marker, b"ocfl_object_1.1\n").expect("failed to create object marker");


    // Add empty inventory.json file
    let inventory = dir.join("inventory.json");
    fs::write(&inventory, b"{}").expect("failed to create inventory.json");

    // With no 'id' field, get_object_id must return an error
    let result = get_object_id(&dir);
    assert!(
        result.is_err(),
        "get_object_id should return an error when 'id' is not present in inventory.json"
    );

    // Add 'id' field to inventory.json (write valid JSON)
    fs::write(&inventory, br#"{"id":"12345"}"#).expect("failed to write inventory.json");
    let result = get_object_id(&dir);
    assert!(
        result.is_ok(),
        "get_object_id should return Ok when 'id' is present in inventory.json"
    );
    assert_eq!(result.unwrap(), "12345");

    // Cleanup
    let _ = fs::remove_file(&marker);
    let _ = fs::remove_file(&inventory);
    let _ = fs::remove_dir(&dir);
}

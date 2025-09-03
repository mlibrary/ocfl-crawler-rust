use ocfl_crawler_rust::is_object_root;
use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn is_object_root_detects_markers() {
    let tmp_root = Path::new("tests/tmp");
    // Ensure tests/tmp exists
    fs::create_dir_all(tmp_root).expect("failed to create tests/tmp directory");

    // Create a unique subdirectory to avoid test collisions
    let unique = format!(
        "is_object_root_{}_{}",
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

    // Add inventory.json for object root checks
    let inventory = dir.join("inventory.json");
    fs::write(&inventory, b"{}").expect("failed to create inventory.json");

    // No marker yet => false
    assert!(
        !is_object_root(&dir),
        "directory without OCFL marker should not be a object root"
    );

    // Add 1.0 marker => true
    let marker_10 = dir.join("0=ocfl_object_1.0");
    fs::write(&marker_10, b"ocfl_object_1.0\n").expect("failed to create 0=ocfl_object_1.0");
    let contents_10 = fs::read_to_string(&marker_10).expect("failed to read 0=ocfl_object_1.0");
    assert_eq!(
        contents_10, "ocfl_object_1.0\n",
        "0=ocfl_object_1.0 contents must be the 'ocfl_object_1.0\\n'"
    );
    assert!(
        is_object_root(&dir),
        "directory with 0=ocfl_object_1.0 should be a object root"
    );

    // Add 1.1 marker => false
    let marker_11 = dir.join("0=ocfl_object_1.1");
    fs::write(&marker_11, b"ocfl_object_1.1\n").expect("failed to create 0=ocfl_object_1.1");
    let contents_11 = fs::read_to_string(&marker_11).expect("failed to read 0=ocfl_object_1.1");
    assert_eq!(
        contents_11, "ocfl_object_1.1\n",
        "0=ocfl_object_1.1 contents must be the 'ocfl_object_1.1\\n'"
    );
    assert!(
        !is_object_root(&dir),
        "directory with 0=ocfl_object_1.0 and 0=ocfl_object_1.1 should not be a object root"
    );

    // Remove 1.0 marker so there is only the 1.1 marker => true
    fs::remove_file(&marker_10).expect("failed to remove 0=ocfl_object_1.0");
    assert!(
        is_object_root(&dir),
        "directory with 0=ocfl_object_1.1 should be a object root"
    );

    // Negative: not a directory
    let not_dir = tmp_root.join(format!(
        "{}_file",
        dir.file_name().unwrap().to_string_lossy()
    ));
    fs::write(&not_dir, b"").expect("failed to create non-directory test file");
    assert!(
        !is_object_root(&not_dir),
        "non-directory path must not be a object root"
    );

    // Cleanup
    let _ = fs::remove_file(&marker_11);
    let _ = fs::remove_file(&not_dir);
    let _ = fs::remove_file(&inventory);
    let _ = fs::remove_dir(&dir);
}

#[test]
fn is_object_root_requires_inventory_json() {
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

    // Add object marker but no inventory.json -> should be false
    let marker = dir.join("0=ocfl_object_1.1");
    fs::write(&marker, b"ocfl_object_1.1\n").expect("failed to create object marker");
    assert!(
        !is_object_root(&dir),
        "object root must contain inventory.json"
    );

    // Add inventory.json -> should be true
    let inventory = dir.join("inventory.json");
    fs::write(&inventory, b"{}").expect("failed to create inventory.json");
    assert!(
        is_object_root(&dir),
        "object root with marker and inventory.json should be detected"
    );

    // Cleanup
    let _ = fs::remove_file(&marker);
    let _ = fs::remove_file(&inventory);
    let _ = fs::remove_dir(&dir);
}

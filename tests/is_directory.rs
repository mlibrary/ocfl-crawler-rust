use ocfl_crawler_rust::is_directory;
use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn is_directory_test() {
    let tmp_root = Path::new("tests/tmp");
    // Ensure tests/tmp exists
    fs::create_dir_all(tmp_root).expect("failed to create tests/tmp directory");

    // Create a unique subdirectory to avoid test collisions
    let unique = format!(
        "is_a_directory_{}_{}",
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

    assert!(
        is_directory(&dir),
        "expected the created path to be a directory"
    );

    fs::remove_dir(&dir).expect("failed to remove test directory");

    assert!(
        !is_directory(&dir),
        "expected the removed path to not be a directory"
    );
}

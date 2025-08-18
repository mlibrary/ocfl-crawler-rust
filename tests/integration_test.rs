use ocfl_crawler_rust::find_matches;

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

#[test]
fn crawl_touch() {
    // Create a unique temporary file with test content
    let mut path = std::env::temp_dir();
    let unique = format!(
        "crawler_it_{}_{}.txt",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    path.push(unique);
    std::fs::write(&path, "lorem ipsum\ndolor sit amet\n").expect("write temp file");

    // Invoke the compiled binary produced by Cargo for this package
    let exe = env!("CARGO_BIN_EXE_ocfl-crawler-rust");
    let output = std::process::Command::new(exe)
        .arg("lorem")
        .arg(&path)
        .output()
        .expect("failed to run binary");

    // Clean up temp file
    let _ = std::fs::remove_file(&path);

    // Validate outcome
    assert!(
        output.status.success(),
        "process failed: status={:?}, stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).expect("stdout not UTF-8"),
        "lorem ipsum\n"
    );
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

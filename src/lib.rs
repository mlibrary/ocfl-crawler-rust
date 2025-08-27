//! Crawler library for OCFL objects and their content.

use std::path::Path;

/// Returns true if `path` is a directory, otherwise false.
pub fn is_directory<P: AsRef<Path>>(path: P) -> bool {
    // This method returns false if the path doesn't exist.
    path.as_ref().is_dir()
}

/// Returns true if `path` is a directory and contains an OCFL storage root marker file
/// ("0=ocfl_1.0" or "0=ocfl_1.1") whose contents are exactly the filename followed by a newline.
pub fn is_storage_root<P: AsRef<Path>>(path: P) -> bool {
    let p = path.as_ref();

    if !is_directory(p) {
        return false;
    }

    ["0=ocfl_1.0", "0=ocfl_1.1"].iter().any(|name| {
        let marker_path = p.join(name);
        if !marker_path.is_file() {
            return false;
        }
        match std::fs::read_to_string(&marker_path) {
            Ok(contents) => contents == format!("{}\n", name),
            Err(_) => false,
        }
    })
}

/// Returns true if `path` is a directory and contains an OCFL object root marker file
/// ("0=ocfl_object_1.0" or "0=ocfl_object_1.1") whose contents are exactly the filename followed by a newline.
pub fn is_object_root<P: AsRef<Path>>(path: P) -> bool {
    let p = path.as_ref();

    if !is_directory(p) {
        return false;
    }

    ["0=ocfl_object_1.0", "0=ocfl_object_1.1"].iter().any(|name| {
        let marker_path = p.join(name);
        if !marker_path.is_file() {
            return false;
        }
        match std::fs::read_to_string(&marker_path) {
            Ok(contents) => contents == format!("{}\n", name),
            Err(_) => false,
        }
    })
}
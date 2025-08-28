use crate::utils::is_directory;
use std::path::Path;

/// Returns true if `path` is a directory and contains a single OCFL storage root marker file
/// ("0=ocfl_1.0" xor "0=ocfl_1.1") whose contents are "ocfl_1.0\n" and "ocfl_1.1\n" respectively.
pub fn is_storage_root<P: AsRef<Path>>(path: P) -> bool {
    let p = path.as_ref();

    if !is_directory(p) {
        return false;
    }

    let marker_10 = p.join("0=ocfl_1.0");
    let marker_11 = p.join("0=ocfl_1.1");

    let exists_10 = marker_10.is_file();
    let exists_11 = marker_11.is_file();

    // Require exactly one marker file to be present (exclusive or).
    if exists_10 == exists_11 {
        return false;
    }

    if exists_10 {
        match std::fs::read_to_string(&marker_10) {
            Ok(contents) => contents == "ocfl_1.0\n",
            Err(_) => false,
        }
    } else {
        match std::fs::read_to_string(&marker_11) {
            Ok(contents) => contents == "ocfl_1.1\n",
            Err(_) => false,
        }
    }
}

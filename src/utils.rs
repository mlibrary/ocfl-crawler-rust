use std::path::Path;

/// Returns true if `path` is a directory, otherwise false.
pub fn is_directory<P: AsRef<Path>>(path: P) -> bool {
    // This method returns false if the path doesn't exist.
    path.as_ref().is_dir()
}

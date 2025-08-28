use crate::utils::is_directory;
use std::path::Path;

/// Returns true if `path` is a directory and contains a single OCFL object root marker file
/// ("0=ocfl_object_1.0" xor "0=ocfl_object_1.1") whose contents are "ocfl_object_1.0\n" and "ocfl_object_1.1\n" respectively,
/// and also contains an `inventory.json` file.
pub fn is_object_root<P: AsRef<Path>>(path: P) -> bool {
    let p = path.as_ref();

    if !is_directory(p) {
        return false;
    }

    let marker_10 = p.join("0=ocfl_object_1.0");
    let marker_11 = p.join("0=ocfl_object_1.1");

    let exists_10 = marker_10.is_file();
    let exists_11 = marker_11.is_file();

    // Require exactly one marker file to be present (exclusive or).
    if exists_10 == exists_11 {
        return false;
    }

    // Object roots must include an inventory.json file
    let inventory = p.join("inventory.json");
    if !inventory.is_file() {
        return false;
    }

    if exists_10 {
        match std::fs::read_to_string(&marker_10) {
            Ok(contents) => contents == "ocfl_object_1.0\n",
            Err(_) => false,
        }
    } else {
        match std::fs::read_to_string(&marker_11) {
            Ok(contents) => contents == "ocfl_object_1.1\n",
            Err(_) => false,
        }
    }
}

/// Returns object id from inventory.json in the OCFL object root directory.
pub fn get_object_id<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let p = path.as_ref();

    if !is_object_root(p) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Not an OCFL object root",
        ));
    }

    let inventory_path = p.join("inventory.json");
    let inventory_content = std::fs::read_to_string(inventory_path)?;

    // Parse JSON and extract the "id" field safely.
    let json: serde_json::Value = serde_json::from_str(&inventory_content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    if let Some(id) = json.get("id").and_then(|v| v.as_str()) {
        if !id.is_empty() {
            return Ok(id.to_string());
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Could not find valid 'id' field in inventory.json",
    ))
}

//! Crawler library for OCFL objects and their content.
pub mod utils;
pub mod object;
pub mod storage;
pub use object::get_object_id;
pub use object::is_object_root;
use std::{env, io, path::{Path, PathBuf}};
pub use storage::is_storage_root;
pub use utils::is_directory;


/// Guard that switches to a directory on creation and restores the previous CWD on drop.
pub struct DirGuard {
    previous: PathBuf,
}

impl DirGuard {
    /// Change to `to` and return a guard that will restore the previous CWD when dropped.
    pub fn change_to<P: AsRef<Path>>(to: P) -> io::Result<Self> {
        let previous = env::current_dir()?;
        env::set_current_dir(&to)?;
        Ok(Self { previous })
    }
}

impl Drop for DirGuard {
    fn drop(&mut self) {
        // Best effort restore; ignore errors during drop.
        let _ = env::set_current_dir(&self.previous);
    }
}

pub fn with_current_dir<P, F, R>(to: P, f: F) -> io::Result<R>
where
    P: AsRef<Path>,
    F: FnOnce() -> R,
{
    let _guard = DirGuard::change_to(to)?;
    Ok(f())
}

mod apt;
mod files;
mod ubuntu;
pub use apt::*;
pub use files::*;
use tracing::trace;
pub use ubuntu::*;

use anyhow::Result;
use std::path::PathBuf;

pub fn replace_file_with_symlink(source: PathBuf, target: PathBuf) -> Result<()> {
    if std::fs::exists(&target)? {
        if target.is_symlink() {
            // TODO: Check the symlink target
            trace!("Skipping {}, symlink already exists", target.display());
            return Ok(());
        }
        backup_file(target.clone())?;
        std::fs::remove_file(&target)?;
    }

    create_symlink(source, target)?;
    Ok(())
}

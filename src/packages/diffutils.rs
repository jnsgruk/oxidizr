use crate::utils::{install_package, list_files, release, replace_file_with_symlink, restore_file};
use anyhow::Result;
use std::path::PathBuf;
use tracing::{info, warn};

const PACKAGE: &str = "rust-diffutils";
const FIRST_SUPPORTED_RELEASE: &str = "24.10";
pub struct RustDiffutils {}

impl RustDiffutils {
    pub fn install() -> Result<()> {
        if !Self::compatible() {
            warn!("Skipping '{PACKAGE}'. Minimum supported release is {FIRST_SUPPORTED_RELEASE}.");
            return Ok(());
        }

        info!("Installing and configuring {}", PACKAGE);

        install_package(PACKAGE)?;

        let files = list_files("/usr/lib/cargo/bin/diffutils")?;

        for f in files {
            let filename = f.file_name().unwrap();
            let existing = PathBuf::from("/usr/bin").join(filename);
            replace_file_with_symlink(f, existing)?;
        }
        Ok(())
    }

    pub fn restore() -> Result<()> {
        if !Self::installed() || !Self::compatible() {
            warn!("{PACKAGE} not found, skipping restore");
            return Ok(());
        }

        info!("Removing {}", PACKAGE);

        let files = list_files("/usr/lib/cargo/bin/diffutils")?;

        for f in files {
            let filename = f.file_name().unwrap();
            let existing = PathBuf::from("/usr/bin").join(filename);
            restore_file(existing)?;
        }

        Ok(())
    }

    fn installed() -> bool {
        list_files("/usr/lib/cargo/bin/diffutils").is_ok()
    }

    fn compatible() -> bool {
        match release() {
            Ok(codename) => codename.as_str() >= FIRST_SUPPORTED_RELEASE,
            Err(_) => false,
        }
    }
}

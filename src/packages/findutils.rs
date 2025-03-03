use crate::utils::{
    install_package, list_files, release, remove_package, replace_file_with_symlink, restore_file,
};
use anyhow::Result;
use std::path::PathBuf;
use tracing::{info, warn};

const PACKAGE: &str = "rust-findutils";
const FIRST_SUPPORTED_RELEASE: &str = "24.04";

pub struct RustFindutils {}

impl RustFindutils {
    pub fn install() -> Result<()> {
        if !Self::compatible() {
            warn!("Skipping '{PACKAGE}'. Minimum supported release is {FIRST_SUPPORTED_RELEASE}.");
            return Ok(());
        }

        info!("Installing and configuring {}", PACKAGE);

        install_package(PACKAGE)?;

        let files = list_files("/usr/lib/cargo/bin/findutils")?;

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

        let files = list_files("/usr/lib/cargo/bin/findutils")?;

        for f in files {
            let filename = f.file_name().unwrap();
            let existing = PathBuf::from("/usr/bin").join(filename);
            restore_file(existing)?;
        }

        remove_package(PACKAGE)?;

        Ok(())
    }

    fn installed() -> bool {
        list_files("/usr/lib/cargo/bin/findutils").is_ok()
    }

    fn compatible() -> bool {
        match release() {
            Ok(codename) => codename.as_str() >= FIRST_SUPPORTED_RELEASE,
            Err(_) => false,
        }
    }
}

use crate::utils::Worker;
use anyhow::Result;
use std::{path::PathBuf, sync::Arc};
use tracing::{info, warn};

use super::Experiment;

pub struct UutilsExperiment {
    system: Arc<dyn Worker>,
    package: String,
    first_supported_release: String,
    unified_binary: Option<PathBuf>,
    bin_directory: PathBuf,
}

impl UutilsExperiment {
    pub fn new(
        system: Arc<dyn Worker>,
        package: &str,
        first_supported_release: &str,
        unified_binary: Option<PathBuf>,
        bin_directory: PathBuf,
    ) -> Self {
        Self {
            system,
            package: package.to_string(),
            first_supported_release: first_supported_release.to_string(),
            unified_binary,
            bin_directory,
        }
    }

    fn check_compatible(&self) -> bool {
        self.system.distribution().release >= self.first_supported_release
    }

    fn check_installed(&self) -> bool {
        self.system.check_installed(&self.package).unwrap_or(false)
    }
}

impl Experiment for UutilsExperiment {
    fn enable(&self) -> Result<()> {
        if !self.check_compatible() {
            warn!(
                "Skipping '{}'. Minimum supported release is {}.",
                self.package, self.first_supported_release
            );
            return Ok(());
        }

        info!("Installing and configuring {}", self.package);

        self.system.install_package(&self.package)?;

        let files = self.system.list_files(self.bin_directory.clone())?;

        for f in files {
            let filename = f.file_name().unwrap();
            let existing = PathBuf::from("/usr/bin").join(filename);

            if let Some(unified_binary) = &self.unified_binary {
                self.system
                    .replace_file_with_symlink(unified_binary.to_path_buf(), existing.clone())?;
            } else {
                self.system.replace_file_with_symlink(f, existing)?;
            }
        }

        Ok(())
    }

    fn disable(&self) -> Result<()> {
        if !self.check_installed() {
            warn!("{} not found, skipping restore", self.package);
            return Ok(());
        }

        info!("Removing {}", self.package);

        let files = self.system.list_files(self.bin_directory.clone())?;

        for f in files {
            let filename = f.file_name().unwrap();
            let existing = PathBuf::from("/usr/bin").join(filename);
            self.system.restore_file(existing)?;
        }

        self.system.remove_package(&self.package)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{Distribution, MockSystem};

    #[test]
    fn test_uutils_incompatible_distribution() {
        let runner = incompatible_runner();
        let coreutils = coreutils_fixture(runner.clone());

        assert!(!coreutils.check_compatible());

        assert!(coreutils.enable().is_ok());
        assert_eq!(runner.commands.clone().into_inner().len(), 0);
        assert_eq!(runner.created_symlinks.clone().into_inner().len(), 0);
        assert_eq!(runner.backed_up_files.clone().into_inner().len(), 0);
        assert_eq!(runner.restored_files.clone().into_inner().len(), 0);
    }

    #[test]
    fn test_uutils_install_success_unified_binary() {
        let runner = coreutils_compatible_runner();
        let coreutils = coreutils_fixture(runner.clone());

        assert!(coreutils.enable().is_ok());

        let commands = runner.commands.clone().into_inner();
        assert_eq!(commands, &["apt-get install -y rust-coreutils"]);

        let backed_up_files = runner.backed_up_files.clone().into_inner();
        let expected = ["/usr/bin/date", "/usr/bin/sort"];

        assert_eq!(backed_up_files.len(), 2);
        for f in backed_up_files.iter() {
            assert!(expected.contains(&f.as_str()));
        }

        let created_symlinks = runner.created_symlinks.clone().into_inner();
        let expected = [
            ("/usr/bin/coreutils", "/usr/bin/sort"),
            ("/usr/bin/coreutils", "/usr/bin/date"),
        ];

        assert!(created_symlinks.len() == 2);
        for (from, to) in created_symlinks.iter() {
            assert!(expected.contains(&(from.as_str(), to.as_str())));
        }

        assert_eq!(runner.restored_files.clone().into_inner().len(), 0);
    }

    #[test]
    fn test_uutils_install_success_non_unified_binary() {
        let runner = findutils_compatible_runner();
        let findutils = findutils_fixture(runner.clone());

        assert!(findutils.enable().is_ok());

        let commands = runner.commands.clone().into_inner();
        assert_eq!(commands, &["apt-get install -y rust-findutils"]);

        let backed_up_files = runner.backed_up_files.clone().into_inner();
        let expected = ["/usr/bin/find", "/usr/bin/xargs"];

        assert_eq!(backed_up_files.len(), 2);
        for f in backed_up_files.iter() {
            assert!(expected.contains(&f.as_str()));
        }

        let created_symlinks = runner.created_symlinks.clone().into_inner();
        let expected = [
            ("/usr/lib/cargo/bin/findutils/find", "/usr/bin/find"),
            ("/usr/lib/cargo/bin/findutils/xargs", "/usr/bin/xargs"),
        ];

        assert!(created_symlinks.len() == 2);
        for (from, to) in created_symlinks.iter() {
            assert!(expected.contains(&(from.as_str(), to.as_str())));
        }

        assert_eq!(runner.restored_files.clone().into_inner().len(), 0);
    }

    #[test]
    fn test_uutils_restore_not_installed() {
        let runner = Arc::new(MockSystem::default());
        let coreutils = coreutils_fixture(runner.clone());

        assert!(coreutils.disable().is_ok());

        assert_eq!(runner.commands.clone().into_inner().len(), 0);
        assert_eq!(runner.created_symlinks.clone().into_inner().len(), 0);
        assert_eq!(runner.backed_up_files.clone().into_inner().len(), 0);
        assert_eq!(runner.restored_files.clone().into_inner().len(), 0);
    }

    #[test]
    fn test_uutils_restore_installed() {
        let runner = coreutils_compatible_runner();
        runner.mock_install_package("rust-coreutils");

        let coreutils = coreutils_fixture(runner.clone());
        assert!(coreutils.disable().is_ok());

        assert_eq!(runner.created_symlinks.clone().into_inner().len(), 0);
        assert_eq!(runner.backed_up_files.clone().into_inner().len(), 0);

        let commands = runner.commands.clone().into_inner();
        assert_eq!(commands.len(), 1);
        assert!(commands.contains(&"apt-get remove -y rust-coreutils".to_string()));

        let restored_files = runner.restored_files.clone().into_inner();
        let expected = ["/usr/bin/date", "/usr/bin/sort"];

        assert_eq!(restored_files.len(), 2);
        for f in restored_files.iter() {
            assert!(expected.contains(&f.as_str()));
        }
    }

    fn coreutils_fixture(system: Arc<MockSystem>) -> UutilsExperiment {
        UutilsExperiment::new(
            system.clone(),
            "rust-coreutils",
            "24.04",
            Some(PathBuf::from("/usr/bin/coreutils")),
            PathBuf::from("/usr/lib/cargo/bin/coreutils"),
        )
    }

    fn coreutils_compatible_runner() -> Arc<MockSystem> {
        let runner = Arc::new(MockSystem::default());
        runner.mock_files(vec![
            ("/usr/lib/cargo/bin/coreutils/date", ""),
            ("/usr/lib/cargo/bin/coreutils/sort", ""),
            ("/usr/bin/sort", ""),
            ("/usr/bin/date", ""),
        ]);
        runner
    }

    fn findutils_fixture(system: Arc<MockSystem>) -> UutilsExperiment {
        UutilsExperiment::new(
            system.clone(),
            "rust-findutils",
            "24.04",
            None,
            PathBuf::from("/usr/lib/cargo/bin/findutils"),
        )
    }

    fn findutils_compatible_runner() -> Arc<MockSystem> {
        let runner = Arc::new(MockSystem::default());
        runner.mock_files(vec![
            ("/usr/lib/cargo/bin/findutils/find", ""),
            ("/usr/lib/cargo/bin/findutils/xargs", ""),
            ("/usr/bin/find", ""),
            ("/usr/bin/xargs", ""),
        ]);
        runner
    }

    fn incompatible_runner() -> Arc<MockSystem> {
        Arc::new(MockSystem::new(Distribution {
            id: "Ubuntu".to_string(),
            release: "20.04".to_string(),
        }))
    }
}

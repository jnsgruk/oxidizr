use crate::utils::Worker;
use anyhow::Result;
use std::path::{Path, PathBuf};
use tracing::info;

const PACKAGE: &str = "sudo-rs";

/// An experiment to install and configure sudo-rs as a replacement for sudo.
pub struct SudoRsExperiment<'a> {
    system: &'a dyn Worker,
}

impl<'a> SudoRsExperiment<'a> {
    /// Create a new SudoRsExperiment.
    pub fn new(system: &'a dyn Worker) -> Self {
        Self { system }
    }

    /// Check if the system is compatible with the experiment.
    pub fn check_compatible(&self) -> bool {
        self.supported_releases().contains(
            &self
                .system
                .distribution()
                .expect("unable to determine distribution information")
                .release,
        )
    }

    /// Reports the first supported release for the experiment.
    pub fn supported_releases(&self) -> Vec<String> {
        vec![
            "24.04".to_string(),
            "24.10".to_string(),
            "25.04".to_string(),
        ]
    }

    /// Check if the package is installed.
    pub fn check_installed(&self) -> bool {
        self.system.check_installed(PACKAGE).unwrap_or(false)
    }

    /// Report the name of the experiment.
    pub fn name(&self) -> String {
        String::from("sudo-rs")
    }

    /// Enable the experiment by installing and configuring the package.
    pub fn enable(&self) -> Result<()> {
        info!("Installing and configuring {}", PACKAGE);
        self.system.install_package(PACKAGE)?;

        for f in Self::sudors_files() {
            let filename = f.file_name().unwrap().to_str().unwrap();
            let existing = match self.system.which(filename) {
                Ok(path) => path,
                Err(_) => Path::new("/usr/bin").join(filename),
            };
            self.system.replace_file_with_symlink(f, existing)?;
        }

        Ok(())
    }

    /// Disable the experiment by removing the package and restoring the original files.
    pub fn disable(&self) -> Result<()> {
        for f in Self::sudors_files() {
            let filename = f.file_name().unwrap().to_str().unwrap();
            let existing = match self.system.which(filename) {
                Ok(path) => path,
                Err(_) => Path::new("/usr/bin").join(filename),
            };
            self.system.restore_file(existing.clone())?;
        }

        info!("Removing {}", PACKAGE);
        self.system.remove_package(PACKAGE)?;

        Ok(())
    }

    /// List of files from the package to replace system equivalents with.
    fn sudors_files() -> Vec<PathBuf> {
        vec![
            PathBuf::from("/usr/lib/cargo/bin/su"),
            PathBuf::from("/usr/lib/cargo/bin/sudo"),
            PathBuf::from("/usr/lib/cargo/bin/visudo"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{Distribution, MockSystem, vecs_eq};

    #[test]
    fn test_sudors_incompatible_distribution() {
        let runner = incompatible_runner();
        let coreutils = sudors_fixture(&runner);
        assert!(!coreutils.check_compatible());
    }

    #[test]
    fn test_sudors_install_success() {
        let runner = sudors_compatible_runner();
        let sudors = sudors_fixture(&runner);

        assert!(sudors.enable().is_ok());

        let commands = runner.commands.clone().into_inner();
        assert_eq!(commands, &["apt-get install -y sudo-rs"]);

        let backed_up_files = runner.backed_up_files.clone().into_inner();
        let expected = vec![
            "/usr/bin/sudo".to_string(),
            "/usr/bin/su".to_string(),
            "/usr/sbin/visudo".to_string(),
        ];
        assert!(vecs_eq(backed_up_files, expected));

        let created_symlinks = runner.created_symlinks.clone().into_inner();
        let expected = vec![
            (
                "/usr/lib/cargo/bin/su".to_string(),
                "/usr/bin/su".to_string(),
            ),
            (
                "/usr/lib/cargo/bin/sudo".to_string(),
                "/usr/bin/sudo".to_string(),
            ),
            (
                "/usr/lib/cargo/bin/visudo".to_string(),
                "/usr/sbin/visudo".to_string(),
            ),
        ];

        assert!(vecs_eq(created_symlinks, expected));
        assert_eq!(runner.restored_files.clone().into_inner().len(), 0);
    }

    #[test]
    fn test_sudors_restore() {
        let runner = sudors_compatible_runner();
        runner.mock_install_package("sudo-rs");

        let sudors = sudors_fixture(&runner);
        assert!(sudors.disable().is_ok());

        assert_eq!(runner.created_symlinks.clone().into_inner().len(), 0);
        assert_eq!(runner.backed_up_files.clone().into_inner().len(), 0);

        let commands = runner.commands.clone().into_inner();
        assert_eq!(commands.len(), 1);
        assert!(commands.contains(&"apt-get remove -y sudo-rs".to_string()));

        let restored_files = runner.restored_files.clone().into_inner();
        let expected = vec![
            "/usr/bin/sudo".to_string(),
            "/usr/bin/su".to_string(),
            "/usr/sbin/visudo".to_string(),
        ];
        assert!(vecs_eq(restored_files, expected));
    }

    fn sudors_fixture(system: &MockSystem) -> SudoRsExperiment {
        SudoRsExperiment::new(system)
    }

    fn sudors_compatible_runner() -> MockSystem {
        let runner = MockSystem::default();
        runner.mock_files(vec![
            ("/usr/lib/cargo/bin/sudo", "", false),
            ("/usr/lib/cargo/bin/su", "", false),
            ("/usr/lib/cargo/bin/visudo", "", false),
            ("/usr/bin/sudo", "", true),
            ("/usr/bin/su", "", true),
            ("/usr/sbin/visudo", "", true),
        ]);
        runner
    }

    fn incompatible_runner() -> MockSystem {
        MockSystem::new(Distribution {
            id: "Ubuntu".to_string(),
            release: "20.04".to_string(),
        })
    }
}

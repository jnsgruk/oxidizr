#[cfg(test)]
pub mod tests {
    use crate::utils::{Command, Distribution, Worker};

    use anyhow::Result;
    use std::{cell::RefCell, collections::HashMap, path::PathBuf, process::Output};

    #[derive(Debug, Clone)]
    pub struct MockSystem {
        /// Tracks the commands executed by the Worker
        pub commands: RefCell<Vec<String>>,
        /// Mock files that the Worker's file-related methods can see/act upon.
        /// The String is the contents, and the bool indicates if this file takes
        /// priority in the simulated $PATH, and returned by the `MockSystem::which` function.
        pub files: RefCell<HashMap<PathBuf, (String, bool)>>,
        /// A list of packages that should report as "installed" on the mock system
        pub installed_packages: RefCell<Vec<String>>,
        /// List of symlinks created by the worker
        pub created_symlinks: RefCell<Vec<(String, String)>>,
        /// List of files restored by the worker
        pub restored_files: RefCell<Vec<String>>,
        /// List of files backed up by the worker
        pub backed_up_files: RefCell<Vec<String>>,
        /// HashMap of mocked commands and their faked responses
        pub mocked_commands: RefCell<HashMap<String, String>>,
    }

    impl Default for MockSystem {
        fn default() -> Self {
            Self::new(Distribution {
                id: "Ubuntu".to_string(),
                release: "24.04".to_string(),
            })
        }
    }

    impl MockSystem {
        pub fn new(distribution: Distribution) -> Self {
            let s = Self {
                commands: RefCell::new(Vec::new()),
                files: RefCell::new(HashMap::new()),
                installed_packages: RefCell::new(Vec::new()),
                created_symlinks: RefCell::new(Vec::new()),
                restored_files: RefCell::new(Vec::new()),
                backed_up_files: RefCell::new(Vec::new()),
                mocked_commands: RefCell::new(HashMap::new()),
            };

            s.mock_command("lsb_release -is", distribution.id.as_str());
            s.mock_command("lsb_release -rs", distribution.release.as_str());
            s
        }

        pub fn mock_files(&self, files: Vec<(&str, &str, bool)>) {
            for (path, contents, primary) in files {
                self.files
                    .borrow_mut()
                    .insert(PathBuf::from(path), (contents.to_string(), primary));
            }
        }

        pub fn mock_install_package(&self, package: &str) {
            self.installed_packages
                .borrow_mut()
                .push(package.to_string());
        }

        pub fn mock_command(&self, command: &str, stdout: &str) {
            self.mocked_commands
                .borrow_mut()
                .insert(command.to_string(), stdout.to_string());
        }
    }

    impl Worker for MockSystem {
        fn run(&self, cmd: &Command) -> Result<Output> {
            self.commands.borrow_mut().push(cmd.command());
            let mocked = self.mocked_commands.borrow();
            let default_stdout = String::default();
            let stdout = mocked.get(&cmd.command()).unwrap_or(&default_stdout);

            Ok(Output {
                stdout: stdout.as_bytes().to_vec(),
                stderr: Vec::new(),
                status: std::process::ExitStatus::default(),
            })
        }

        fn check_installed(&self, package: &str) -> Result<bool> {
            Ok(self
                .installed_packages
                .borrow()
                .contains(&package.to_string()))
        }

        fn list_files(&self, directory: PathBuf) -> Result<Vec<PathBuf>> {
            let files: Vec<PathBuf> = self
                .files
                .borrow()
                .iter()
                .filter(|(k, _)| k.starts_with(directory.to_str().unwrap()))
                .map(|(k, _)| k.clone())
                .collect();
            Ok(files)
        }

        fn which(&self, binary_name: &str) -> Result<PathBuf> {
            for (filename, file) in self.files.borrow().iter() {
                if filename.file_name().unwrap().to_str().unwrap() == binary_name && file.1 {
                    return Ok(filename.clone());
                }
            }
            anyhow::bail!("{} not found in mocked filesystem", binary_name);
        }

        fn replace_file_with_symlink(&self, source: PathBuf, target: PathBuf) -> Result<()> {
            if self.files.borrow().contains_key(&target) {
                self.backup_file(target.clone())?;
            }

            self.create_symlink(source, target.clone())
        }

        fn create_symlink(&self, source: PathBuf, target: PathBuf) -> Result<()> {
            self.created_symlinks.borrow_mut().push((
                source.into_os_string().into_string().unwrap(),
                target.into_os_string().into_string().unwrap(),
            ));
            Ok(())
        }

        fn backup_file(&self, file: PathBuf) -> Result<()> {
            self.backed_up_files
                .borrow_mut()
                .push(file.into_os_string().into_string().unwrap());
            Ok(())
        }

        fn restore_file(&self, file: PathBuf) -> Result<()> {
            self.restored_files
                .borrow_mut()
                .push(file.into_os_string().into_string().unwrap());
            Ok(())
        }
    }
}

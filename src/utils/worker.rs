use std::{path::PathBuf, process::Output};

use anyhow::Result;
use std::fs;
use tracing::{debug, trace};

use super::{Command, Distribution};

pub trait Worker {
    fn distribution(&self) -> Distribution;

    fn run(&self, cmd: &Command) -> Result<Output>;
    fn list_files(&self, directory: PathBuf) -> Result<Vec<PathBuf>>;

    fn install_package(&self, package: &str) -> Result<()> {
        let cmd = Command::build("apt-get", &["install", "-y", package]);
        self.run(&cmd)?;
        Ok(())
    }

    fn remove_package(&self, package: &str) -> Result<()> {
        let cmd = Command::build("apt-get", &["remove", "-y", package]);
        self.run(&cmd)?;
        Ok(())
    }

    fn update_package_lists(&self) -> Result<()> {
        let cmd = Command::build("apt-get", &["update"]);
        self.run(&cmd)?;
        Ok(())
    }

    fn check_installed(&self, package: &str) -> Result<bool> {
        let cmd = Command::build("apt", &["list", package]);
        let output = self.run(&cmd)?;
        let output = String::from_utf8(output.stdout)?.to_string();
        Ok(output.contains("installed"))
    }

    fn replace_file_with_symlink(&self, source: PathBuf, target: PathBuf) -> Result<()>;
    fn backup_file(&self, file: PathBuf) -> Result<()>;
    fn restore_file(&self, file: PathBuf) -> Result<()>;
    fn create_symlink(&self, source: PathBuf, target: PathBuf) -> Result<()>;
}

#[derive(Clone, Debug)]
pub struct System {
    distribution: Distribution,
}

impl System {
    pub fn new() -> Result<Self> {
        Ok(Self {
            distribution: Self::get_distribution()?,
        })
    }

    fn get_distribution() -> Result<Distribution> {
        let cmd = Command::build("lsb_release", &["-is"]);
        debug!("Running command: {}", cmd.command());

        let output = std::process::Command::new(&cmd.command)
            .args(&cmd.args)
            .output()?;

        let dist_id = String::from_utf8(output.stdout)?.trim().to_string();

        let cmd = Command::build("lsb_release", &["-rs"]);
        debug!("Running command: {}", cmd.command());

        let output = std::process::Command::new(&cmd.command)
            .args(&cmd.args)
            .output()?;

        let release = String::from_utf8(output.stdout)?.trim().to_string();

        Ok(Distribution {
            id: dist_id,
            release,
        })
    }
}

impl Worker for System {
    fn distribution(&self) -> Distribution {
        self.distribution.clone()
    }

    fn run(&self, cmd: &Command) -> Result<Output> {
        debug!("Running command: {}", cmd.command());
        let output = std::process::Command::new(&cmd.command)
            .args(&cmd.args)
            .output()?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to run command '{}': {}",
                &cmd.command(),
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(output)
    }

    fn list_files(&self, directory: PathBuf) -> Result<Vec<PathBuf>> {
        if !fs::exists(&directory)? || !fs::metadata(&directory)?.is_dir() {
            anyhow::bail!("{} is not a directory", directory.to_str().unwrap());
        }

        let entries = fs::read_dir(directory)?;

        let files = entries
            .map(|entry| {
                let entry = entry?;
                let path = entry.path();
                Ok(path)
            })
            .collect::<Result<Vec<PathBuf>>>()?;

        Ok(files)
    }

    fn replace_file_with_symlink(&self, source: PathBuf, target: PathBuf) -> Result<()> {
        if fs::exists(&target)? {
            if target.is_symlink() {
                // TODO: Check the symlink target
                trace!("Skipping {}, symlink already exists", target.display());
                return Ok(());
            }
            self.backup_file(target.clone())?;
            fs::remove_file(&target)?;
        }

        self.create_symlink(source, target)?;
        Ok(())
    }

    fn create_symlink(&self, source: PathBuf, target: PathBuf) -> Result<()> {
        if fs::exists(&target)? {
            fs::remove_file(&target)?;
        }

        trace!("Symlinking {} -> {}", source.display(), target.display());
        std::os::unix::fs::symlink(source, target)?;
        Ok(())
    }

    fn backup_file(&self, file: PathBuf) -> Result<()> {
        let mut backup_file = file.parent().unwrap_or(&PathBuf::from(".")).to_path_buf();
        backup_file.push(format!(
            ".{}.oxidizr.bak",
            file.file_name().unwrap().to_string_lossy()
        ));

        trace!("Backing up {} -> {}", file.display(), backup_file.display());
        fs::copy(&file, &backup_file)?;
        Ok(())
    }

    fn restore_file(&self, file: PathBuf) -> Result<()> {
        let mut backup_file = file.parent().unwrap_or(&PathBuf::from(".")).to_path_buf();
        backup_file.push(format!(
            ".{}.oxidizr.bak",
            file.file_name().unwrap().to_string_lossy()
        ));

        if fs::exists(&file)? {
            fs::remove_file(&file)?;
        }

        if fs::exists(&backup_file)? {
            trace!("Restoring {} -> {}", backup_file.display(), file.display());
            fs::rename(&backup_file, &file)?;
        }

        Ok(())
    }
}

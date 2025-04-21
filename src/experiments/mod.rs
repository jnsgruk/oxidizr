mod sudors;
mod uutils;
use crate::utils::Worker;
use anyhow::Result;
use std::path::PathBuf;
pub use sudors::SudoRsExperiment;
use tracing::warn;
pub use uutils::UutilsExperiment;

pub enum Experiment<'a> {
    Uutils(UutilsExperiment<'a>),
    SudoRs(SudoRsExperiment<'a>),
}

impl Experiment<'_> {
    pub fn name(&self) -> String {
        match self {
            Experiment::Uutils(uutils) => uutils.name(),
            Experiment::SudoRs(sudors) => sudors.name(),
        }
    }

    pub fn enable(&self, no_compatibility_check: bool) -> Result<()> {
        if !no_compatibility_check && !self.check_compatible() {
            warn!(
                "Skipping '{}'. Minimum supported release is {}.",
                self.name(),
                self.first_supported_release()
            );
            return Ok(());
        }
        match self {
            Experiment::Uutils(e) => e.enable(),
            Experiment::SudoRs(e) => e.enable(),
        }
    }

    pub fn disable(&self) -> Result<()> {
        if !self.check_installed() {
            warn!("'{}' not enabled, skipping restore", self.name());
            return Ok(());
        }
        match self {
            Experiment::Uutils(e) => e.disable(),
            Experiment::SudoRs(e) => e.disable(),
        }
    }

    pub fn check_compatible(&self) -> bool {
        match self {
            Experiment::Uutils(e) => e.check_compatible(),
            Experiment::SudoRs(e) => e.check_compatible(),
        }
    }

    pub fn first_supported_release(&self) -> &str {
        match self {
            Experiment::Uutils(e) => e.first_supported_release(),
            Experiment::SudoRs(e) => e.first_supported_release(),
        }
    }

    pub fn check_installed(&self) -> bool {
        match self {
            Experiment::Uutils(e) => e.check_installed(),
            Experiment::SudoRs(e) => e.check_installed(),
        }
    }
}

pub fn all_experiments<'a>(system: &'a impl Worker) -> Vec<Experiment<'a>> {
    vec![
        Experiment::Uutils(UutilsExperiment::<'a>::new(
            "coreutils",
            system,
            "rust-coreutils",
            "24.04",
            Some(PathBuf::from("/usr/bin/coreutils")),
            PathBuf::from("/usr/lib/cargo/bin/coreutils"),
        )),
        Experiment::Uutils(UutilsExperiment::<'a>::new(
            "diffutils",
            system,
            "rust-diffutils",
            "24.10",
            Some(PathBuf::from("/usr/lib/cargo/bin/diffutils/diffutils")),
            PathBuf::from("/usr/lib/cargo/bin/diffutils"),
        )),
        Experiment::Uutils(UutilsExperiment::<'a>::new(
            "findutils",
            system,
            "rust-findutils",
            "24.04",
            None,
            PathBuf::from("/usr/lib/cargo/bin/findutils"),
        )),
        Experiment::SudoRs(SudoRsExperiment::<'a>::new(system)),
    ]
}

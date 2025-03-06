mod uutils;
use crate::utils::Worker;
use anyhow::Result;
use std::path::PathBuf;
pub use uutils::UutilsExperiment;

pub enum Experiment<'a> {
    Uutils(UutilsExperiment<'a>),
}

impl Experiment<'_> {
    pub fn name(&self) -> String {
        match self {
            Experiment::Uutils(uutils) => uutils.name(),
        }
    }

    pub fn enable(&self) -> Result<()> {
        match self {
            Experiment::Uutils(uutils) => uutils.enable(),
        }
    }

    pub fn disable(&self) -> Result<()> {
        match self {
            Experiment::Uutils(uutils) => uutils.disable(),
        }
    }
}

pub fn all_experiments<'a>(system: &'a impl Worker) -> Vec<UutilsExperiment<'a>> {
    vec![
        UutilsExperiment::<'a>::new(
            "coreutils",
            system,
            "rust-coreutils",
            "24.04",
            Some(PathBuf::from("/usr/bin/coreutils")),
            PathBuf::from("/usr/lib/cargo/bin/coreutils"),
        ),
        UutilsExperiment::<'a>::new(
            "diffutils",
            system,
            "rust-diffutils",
            "24.10",
            Some(PathBuf::from("/usr/lib/cargo/bin/diffutils/diffutils")),
            PathBuf::from("/usr/lib/cargo/bin/diffutils"),
        ),
        UutilsExperiment::<'a>::new(
            "findutils",
            system,
            "rust-findutils",
            "24.04",
            None,
            PathBuf::from("/usr/lib/cargo/bin/findutils"),
        ),
    ]
}

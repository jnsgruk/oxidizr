mod uutils;

use std::{path::PathBuf, sync::Arc};

pub use uutils::UutilsExperiment;

use anyhow::Result;

use crate::utils::Worker;

pub trait Experiment {
    fn enable(&self) -> Result<()>;
    fn disable(&self) -> Result<()>;
}

pub fn all_experiments(system: Arc<dyn Worker>) -> Vec<(String, Box<dyn Experiment>)> {
    vec![
        (
            String::from("coreutils"),
            Box::new(UutilsExperiment::new(
                system.clone(),
                "rust-coreutils",
                "24.04",
                Some(PathBuf::from("/usr/bin/coreutils")),
                PathBuf::from("/usr/lib/cargo/bin/coreutils"),
            )),
        ),
        (
            String::from("diffutils"),
            Box::new(UutilsExperiment::new(
                system.clone(),
                "rust-diffutils",
                "24.10",
                Some(PathBuf::from("/usr/lib/cargo/bin/diffutils/diffutils")),
                PathBuf::from("/usr/lib/cargo/bin/diffutils"),
            )),
        ),
        (
            String::from("findutils"),
            Box::new(UutilsExperiment::new(
                system.clone(),
                "rust-findutils",
                "24.04",
                None,
                PathBuf::from("/usr/lib/cargo/bin/findutils"),
            )),
        ),
    ]
}

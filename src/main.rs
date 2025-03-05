//! A command-line utility for managing system experiments that replace traditional Unix utilities
//! with modern Rust-based alternatives on Ubuntu systems.
//!
//! # Overview
//! This utility allows users to replace traditional Unix utilities (like coreutils, findutils,
//! and diffutils) with their modern Rust implementations. It provides functionality to both
//! enable and disable these experiments safely.
//!
//! # Usage
//! The program must be run as root and supports two main commands:
//! - `enable`: Activates selected experiments
//! - `disable`: Deactivates selected experiments
//!
//! # Safety
//! The utility includes built-in safety measures:
//! - Distribution compatibility check
//! - Confirmation prompts (unless explicitly skipped)
//! - Package list updates before modifications
//!
//! # Example
//! ```bash
//! sudo oxidizr enable --experiments coreutils findutils
//! ```
//!
//! # Warning
//! This utility can make significant system changes that might affect system stability
//! and functionality. Users should proceed with caution and understand the implications
//! of replacing system utilities.
pub mod experiments;
pub mod utils;

use std::{process::exit, sync::Arc};

use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use experiments::{all_experiments, Experiment};
use inquire::Confirm;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*};
use utils::{System, Worker};

/// A command-line utility to install modern Rust-based replacements of essential
/// packages such as coreutils, findutils, diffutils and sudo and make them the
/// default on an Ubuntu system.
#[derive(Debug, Parser)]
#[command(version, about, long_about)]
struct Args {
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,

    #[arg(
        short,
        long,
        default_value_t = false,
        global = true,
        help = "Skip confirmation prompts"
    )]
    yes: bool,

    #[arg(
        short,
        long,
        default_values_t = vec!["coreutils".to_string(), "findutils".to_string(), "diffutils".to_string()],
        global = true,
        num_args = 1..,
        help = "Select experiments to enable or disable"
    )]
    experiments: Vec<String>,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Enable experiments with oxidizr.
    Enable,
    /// Disable any previous experiments enabled with oxidizr.
    Disable,
}

fn main() -> Result<()> {
    // The application must run as root - exit immediately if it's not.
    anyhow::ensure!(
        uzers::get_current_uid() == 0,
        "This program must be run as root"
    );

    let args = Args::parse();

    // Initialise the tracing system to enable nice logging. Take into account the verbosity
    // specified at the command line.
    tracing_subscriber::registry()
        .with(args.verbose.tracing_level_filter())
        .with(fmt::layer().compact().with_target(false))
        .init();

    // Initialise the system, gather system information.
    let system = Arc::new(System::new()?);

    // Exit if the application is run on a non-Ubuntu machine.
    anyhow::ensure!(
        system.distribution().id == "Ubuntu",
        "This program only supports Ubuntu"
    );

    // Get the set of enabled experiments. If the user specified nothing, all experiments are
    // enabled.
    let selected_experiments = enabled_experiments(system.clone(), args.experiments);

    // Handle subcommands
    match args.cmd {
        Commands::Enable => {
            confirm_or_exit(args.yes);

            info!("Updating apt package cache");
            system.update_package_lists()?;

            enable(selected_experiments)
        }
        Commands::Disable => {
            confirm_or_exit(args.yes);
            disable(selected_experiments)
        }
    }
}

/// Returns a list of initialised experiments that are enabled, either by the default options
/// or by the user issuing `--experiments` at the command-line.
fn enabled_experiments(system: Arc<dyn Worker>, args: Vec<String>) -> Vec<Box<dyn Experiment>> {
    // Fetch the full list of available experiments
    let all_experiments = all_experiments(system.clone());

    // Filter the list of experiments by what the user selected
    all_experiments
        .into_iter()
        .filter_map(|(name, experiment)| {
            if args.contains(&name) {
                Some(experiment)
            } else {
                None
            }
        })
        .collect()
}

/// Display a confirmation prompt to the user asking whether they'd like to continue.
/// If they select no, or there is an error - exit the program.
/// If `--yes` was supplied on the command line, skip the check and return.
fn confirm_or_exit(yes: bool) {
    // If the user has specified '--yes', skip the prompt and carry on.
    if yes {
        return;
    }

    // Otherwise prompt the user before continuing
    let ans = Confirm::new("Continue?")
                .with_default(false)
                .with_help_message("⚠️ oxidizr can cause harm to your system! ⚠️\nDepending on your configuration and workload, oxidizr's\nexperiments could cause your machine to fail to boot, or\nyour workloads to fail. Use with caution.")
                .prompt();

    match ans {
        Ok(true) => (),
        Ok(false) => exit(1),
        Err(_) => exit(1),
    }
}

/// Enables selected experiments
fn enable(experiments: Vec<Box<dyn Experiment>>) -> Result<()> {
    for e in experiments.iter() {
        e.enable()?;
    }
    Ok(())
}

// Disable selected experiments
fn disable(experiments: Vec<Box<dyn Experiment>>) -> Result<()> {
    for e in experiments.iter() {
        e.disable()?;
    }
    Ok(())
}

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

use std::process::exit;

use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use experiments::{Experiment, all_experiments};
use inquire::Confirm;
use tracing::{info, warn};
use tracing_subscriber::{fmt, prelude::*};
use utils::{System, Worker, vecs_eq};

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
        default_value_t = false,
        global = true,
        help = "Enable/disable all known experiments"
    )]
    all: bool,

    #[arg(
        long,
        default_value_t = false,
        global = true,
        help = "Skip compatibility checks (dangerous)"
    )]
    no_compatibility_check: bool,

    #[arg(
        short,
        long,
        global = true,
        num_args = 1..,
        default_values_t = default_experiments(),
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
    let args = Args::parse();

    // The application must run as root - exit immediately if it's not.
    anyhow::ensure!(
        uzers::get_current_uid() == 0,
        "This program must be run as root"
    );

    // Initialise the tracing system to enable nice logging. Take into account the verbosity
    // specified at the command line.
    tracing_subscriber::registry()
        .with(args.verbose.tracing_level_filter())
        .with(fmt::layer().compact().with_target(false))
        .init();

    // Initialise the system, gather system information.
    let system = System::new()?;

    // Exit if the application is run on a non-Ubuntu machine (unless compatibility check is skipped).
    if !args.no_compatibility_check {
        anyhow::ensure!(
            system.distribution()?.id == "Ubuntu",
            "This program only supports Ubuntu"
        );
    } else if system.distribution()?.id != "Ubuntu" {
        warn!(
            "Running on a non-Ubuntu distribution. This is unsupported and may cause system instability."
        );
    }

    // Get selected experiments from the command line arguments
    let selected = selected_experiments(args.all, args.experiments.clone(), &system);

    // Handle subcommands
    match args.cmd {
        Commands::Enable => enable(&system, selected, args.yes, args.no_compatibility_check),
        Commands::Disable => disable(selected, args.yes),
    }
}

/// Enables selected experiments
fn enable(
    system: &impl Worker,
    experiments: Vec<Experiment>,
    yes: bool,
    no_compatibility_check: bool,
) -> Result<()> {
    confirm_or_exit(yes);

    info!("Updating apt package cache");
    system.update_package_lists()?;

    for e in experiments.iter() {
        e.enable(no_compatibility_check)?;
    }
    Ok(())
}

// Disable selected experiments
fn disable(experiments: Vec<Experiment<'_>>, yes: bool) -> Result<()> {
    confirm_or_exit(yes);
    for e in experiments.iter() {
        e.disable()?;
    }
    Ok(())
}

/// Get selected experiments from the command line arguments.
fn selected_experiments(
    all: bool,
    selected: Vec<String>,
    system: &impl Worker,
) -> Vec<Experiment<'_>> {
    let all_experiments = all_experiments(system);
    let default_experiments = default_experiments();

    match all {
        true => {
            if !selected.is_empty() && !vecs_eq(selected, default_experiments) {
                warn!("Ignoring --experiments flag as --all is set");
            }

            all_experiments
        }
        false => {
            // If no experiments are selected, default to coreutils and sudo-rs
            let filter = match selected.len() {
                0 => default_experiments,
                _ => selected,
            };

            // Filter the list of all experiments to only include the selected ones
            all_experiments
                .into_iter()
                .filter(|e| filter.contains(&e.name()))
                .collect()
        }
    }
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

// Default experiments to enable if none are specified
fn default_experiments() -> Vec<String> {
    let mut defaults = vec!["coreutils".to_string(), "sudo-rs".to_string()];
    defaults.sort();
    defaults
}

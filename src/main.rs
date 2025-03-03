pub mod packages;
pub mod utils;

use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use tracing_subscriber::{fmt, prelude::*};
use utils::update_package_lists;

/// A command-line utility to install modern Rust-based replacements of essential
/// packages such as coreutils, findutils, diffutils and sudo and make them the
/// default on an Ubuntu system.
#[derive(Debug, Parser)]
#[command(version, about, long_about)]
struct Cli {
    /// Restore a machine that was previously oxidized.
    #[arg(long, default_value = "false")]
    restore: bool,

    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    tracing_subscriber::registry()
        .with(args.verbose.tracing_level_filter())
        .with(fmt::layer().compact().with_target(false))
        .init();

    if args.restore {
        packages::RustCoreutils::restore()?;
        packages::RustFindutils::restore()?;
        packages::RustDiffutils::restore()?;
    } else {
        update_package_lists()?;
        packages::RustCoreutils::install()?;
        packages::RustFindutils::install()?;
        packages::RustDiffutils::install()?;
    }

    Ok(())
}

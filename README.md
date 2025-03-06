# oxidizr

`oxidizr` is a command-line utility for managing system experiments that replace traditional Unix utilities with modern Rust-based alternatives on Ubuntu systems.

## Installation

> [!WARNING]
> `oxidizr` is an experimental tool to help developers and tinkerers play with relatively new alternatives to core system utilities. It may cause a loss of data, or prevent your system from booting, so use with caution!

You can install `oxidizr` using `cargo`:

```bash
cargo install --git https://github.com/jnsgruk/oxidizr
```

## Usage

The program must be run as root and supports two main commands:

- `enable`: Activates selected experiments
- `disable`: Deactivates selected experiments

```bash
A command-line utility to install modern Rust-based replacements of essential packages such as coreutils, findutils, diffutils and sudo and make them the default on an Ubuntu system.

Usage: oxidizr [OPTIONS] <COMMAND>

Commands:
  enable   Enable experiments with oxidizr
  disable  Disable any previous experiments enabled with oxidizr
  help     Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...
          Increase logging verbosity

  -q, --quiet...
          Decrease logging verbosity

  -y, --yes
          Skip confirmation prompts

  -e, --experiments <EXPERIMENTS>...
          Select experiments to enable or disable

          [default: coreutils findutils diffutils]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Example

```bash
# Enable just coreutils and findutils experiments
sudo oxidizr enable --experiments coreutils findutils
# Enable just coreutils experiment without prompting with debug logging enabled
sudo oxidizr enable --experiments coreutils --yes -v
```

## Building `oxidizr`

```bash
# Build with cargo
cargo build

# Run tests
cargo test -- --show-output

# Lint / format
cargo clippy
cargo fmt
```

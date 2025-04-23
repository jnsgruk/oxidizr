# oxidizr

<a href="https://github.com/jnsgruk/oxidizr/actions/workflows/push.yml"><img src="https://github.com/jnsgruk/oxidizr/actions/workflows/push.yml/badge.svg"></a>
<a href="https://github.com/jnsgruk/oxidizr/actions/workflows/release.yml"><img src="https://github.com/jnsgruk/oxidizr/actions/workflows/release.yml/badge.svg"></a>

`oxidizr` is a command-line utility for managing system experiments that replace traditional Unix utilities with modern Rust-based alternatives on Ubuntu systems.

It currently supports the following experiments:

- [uutils coreutils](https://github.com/uutils/coreutils)
- [uutils findutils](https://github.com/uutils/findutils)
- [uutils diffutils](https://github.com/uutils/diffutils)
- [sudo-rs](https://github.com/trifectatechfoundation/sudo-rs)

By default, the `coreutils` and `sudo-rs` experiments are enabled because they're the most complete, stable experiments. Others can be toggled using command line arguments shown below.

## Installation

<!-- prettier-ignore-start -->
> [!WARNING]
> `oxidizr` is an experimental tool to help developers and tinkerers play with relatively new alternatives to core system utilities. It may cause a loss of data, or prevent your system from booting, so use with caution!
<!-- prettier-ignore-end -->

You can install `oxidizr` by downloading binaries from the Github [releases](https://github.com/jnsgruk/oxidizr/releases/latest). Releases are currently published for `amd64` and `aarch64`.

The following will establish the latest released version, download the archive and extract the `oxidizr` binary to `/usr/bin/oxidizr`.

```bash
# Get the latest release
latest="$(curl -s "https://api.github.com/repos/jnsgruk/oxidizr/releases/latest" | jq -r '.name')"
# Download and install to /usr/bin/oxidizr
curl -sL "https://github.com/jnsgruk/oxidizr/releases/download/$latest/oxidizr_Linux_$(uname -m).tar.gz" | sudo tar -xvzf - -C /usr/bin oxidizr
```

Or you can build and install `oxidizr` using `cargo`:

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

  -a, --all
          Enable/disable all known experiments

  -e, --experiments <EXPERIMENTS>...
          Select experiments to enable or disable

          [default: coreutils sudo-rs]

  --no-compatibility-check
          Skip experiment compatibility checks (dangerous)
          This bypasses all system compatibility checks including Ubuntu distribution
          and version requirements. Likely to result in failure to complete, may lead
          to system instability

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Example

```bash
# Enable all experiments
sudo oxidizr enable --all
# Enable just coreutils and findutils experiments
sudo oxidizr enable --experiments coreutils findutils
# Enable just coreutils experiment without prompting with debug logging enabled
sudo oxidizr enable --experiments coreutils --yes -v
# Enable an experiment on an unsupported system (dangerous)
sudo oxidizr enable --no-compatibility-check
# Enable an experiment on an unsupported system without prompting (very dangerous)
sudo oxidizr enable --no-compatibility-check --yes
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

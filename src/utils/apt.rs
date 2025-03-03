use anyhow::Result;
use tracing::debug;

pub fn install_package(package: &str) -> Result<()> {
    debug!("Installing package {package}");
    let output = std::process::Command::new("apt-get")
        .arg("install")
        .arg("-y")
        .arg(package)
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "Failed to install package {}: {}",
            package,
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

pub fn remove_package(package: &str) -> Result<()> {
    debug!("Removing pacakge {package}");
    let output = std::process::Command::new("apt-get")
        .arg("remove")
        .arg("-y")
        .arg(package)
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "Failed to remove package {}: {}",
            package,
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

pub fn update_package_lists() -> Result<()> {
    debug!("Updating apt cache");
    let output = std::process::Command::new("apt-get")
        .arg("update")
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "Failed to update apt cache: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

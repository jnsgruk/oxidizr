use anyhow::Result;

pub fn release() -> Result<String> {
    let output = std::process::Command::new("lsb_release")
        .arg("-rs")
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "Failed to establish Ubuntu codename: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(stdout)
}

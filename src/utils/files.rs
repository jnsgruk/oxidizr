use anyhow::Result;
use std::path::PathBuf;
use tracing::trace;

pub fn list_files(directory: &str) -> Result<Vec<PathBuf>> {
    if !std::fs::exists(directory)? || !std::fs::metadata(directory)?.is_dir() {
        anyhow::bail!("{} is not a directory", directory);
    }

    let entries = std::fs::read_dir(directory)?;

    let files = entries
        .map(|entry| {
            let entry = entry?;
            let path = entry.path();
            Ok(path)
        })
        .collect::<Result<Vec<PathBuf>>>()?;

    Ok(files)
}

pub fn create_symlink(source: PathBuf, target: PathBuf) -> Result<()> {
    if std::fs::exists(&target)? {
        std::fs::remove_file(&target)?;
    }

    trace!("Symlinking {} -> {}", source.display(), target.display());
    std::os::unix::fs::symlink(source, target)?;
    Ok(())
}

pub fn backup_file(file: PathBuf) -> Result<()> {
    let mut backup_file = file.parent().unwrap_or(&PathBuf::from(".")).to_path_buf();
    backup_file.push(format!(
        ".{}.oxidizr.bak",
        file.file_name().unwrap().to_string_lossy()
    ));

    trace!("Backing up {} -> {}", file.display(), backup_file.display());
    std::fs::copy(&file, &backup_file)?;
    Ok(())
}

pub fn restore_file(file: PathBuf) -> Result<()> {
    let mut backup_file = file.parent().unwrap_or(&PathBuf::from(".")).to_path_buf();
    backup_file.push(format!(
        ".{}.oxidizr.bak",
        file.file_name().unwrap().to_string_lossy()
    ));

    if std::fs::exists(&file)? {
        std::fs::remove_file(&file)?;
    }

    if std::fs::exists(&backup_file)? {
        trace!("Restoring {} -> {}", backup_file.display(), file.display());
        std::fs::rename(&backup_file, &file)?;
    }

    Ok(())
}

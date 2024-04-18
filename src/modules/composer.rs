use std::process::Command;
use std::io;
use std::fs;

const COMPOSER_INSTALLER_URL: &str = "https://getcomposer.org/installer";
const COMPOSER_INSTALLER_PATH: &str = "composer_installer.php";

pub fn download_and_install_composer() -> io::Result<()> {
    // Download the installer
    let output = Command::new("curl")
        .arg("-L")
        .arg("-o")
        .arg(COMPOSER_INSTALLER_PATH)
        .arg(COMPOSER_INSTALLER_URL)
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to download Composer installer"));
    }

    // Execute the installer
    let output = Command::new("php")
        .arg(COMPOSER_INSTALLER_PATH)
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to install Composer"));
    }

    // Clean up the installer
    fs::remove_file(COMPOSER_INSTALLER_PATH)?;

    Ok(())
}
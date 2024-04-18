use std::process::Command;
use std::io;
use std::fs;

const SEVEN_ZIP_URL: &str = "https://www.7-zip.org/a/7z2404-x64.exe";
const SEVEN_ZIP_INSTALLER_PATH: &str = "7z_installer.exe";

pub fn download_and_install_seven_zip() -> io::Result<()> {
    // Download the installer
    let output = Command::new("curl")
        .arg("-L")
        .arg("-o")
        .arg(SEVEN_ZIP_INSTALLER_PATH)
        .arg(SEVEN_ZIP_URL)
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to download 7zip installer"));
    }

    // Execute the installer
    let output = Command::new(SEVEN_ZIP_INSTALLER_PATH)
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to install 7zip"));
    }

    // Clean up the installer
    fs::remove_file(SEVEN_ZIP_INSTALLER_PATH)?;

    Ok(())
}
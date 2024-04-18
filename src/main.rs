use std::io::{self, Write};
use std::process::Command;

const PHP_INSTALLER_URL: &str = "https://github.com/sympact06/PhpInstaller";

fn main() -> io::Result<()> {
    display_welcome_message();

    let version = loop {
        match get_user_choice()? {
            1 => break "7.4 LTS",
            2 => break "8.0",
            3 => break "8.1",
            _ => println!("Invalid input, please enter a number between 1 and 3."),
        }
    };

    install_php(version)
}

fn display_welcome_message() {
    println!("************************************");
    println!("PHP Installer for Windows (unofficial) with Rust");
    println!("{}", PHP_INSTALLER_URL);
    println!("************************************");

    println!("Please select the PHP version you would like to install:");
    println!("1. PHP 8.3 (Recommended)");
    println!("2. PHP 8.1");
    println!("3. PHP 8.2");
}

fn get_user_choice() -> io::Result<u32> {
    print!("Enter your choice (1-3): ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    Ok(choice.trim().parse().unwrap_or(0))
}

fn install_php(version: &str) -> io::Result<()> {
    println!("Installing PHP version {}", version);
    // Here you can add the logic to install PHP
    // For example, if you're using a command line installer, you can use the Command struct from std::process
    // Command::new("installer").arg(version).status()?;

    Ok(())
}
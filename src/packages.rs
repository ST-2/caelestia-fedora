use anyhow::{bail, Result};
use std::process::Command;

use crate::{log, ui};

const PACKAGES: &[&str] = &[
    // Hyprland and Wayland
    "hyprland",
    "xdg-desktop-portal-hyprland",
    "xdg-desktop-portal-gtk",
    // Terminal
    "foot",
    // Shell and tools
    "fish",
    // Greetd
    "greetd",
    "tuigreet",
    // Quickshell and Qt6
    "quickshell",
    "qt6-qtbase-devel",
    "qt6-qtdeclarative-devel",
    // Build tools
    "cmake",
    "ninja-build",
    "gcc-c++",
    "git",
    "curl",
    "tar",
    // Python build tools for caelestia-cli
    "python3-build",
    "python3-hatchling",
    "python3-pip",
    // caelestia-cli dependencies
    "libnotify",
    "fuzzel",
    "glib2-devel",
    // Theming
    "adw-gtk3-theme",
    "papirus-icon-theme",
    // Fonts
    "google-noto-fonts-common",
    "google-noto-sans-fonts",
    "fontawesome-fonts",
    // Utilities
    "fastfetch",
    "btop",
    "wl-clipboard",
    "grim",
    "slurp",
    "swappy",
    "brightnessctl",
    "playerctl",
    "pamixer",
    "NetworkManager",
    "lxpolkit",
];

pub fn install_all(dry_run: bool) -> Result<()> {
    ui::info("Installing packages via dnf...");

    let pkg_list = PACKAGES.join(" ");
    let cmd = format!("sudo dnf install -y {}", pkg_list);
    log::log_command(&cmd);

    if dry_run {
        ui::info("Would install the following packages:");
        for pkg in PACKAGES {
            println!("  - {}", pkg);
        }
        ui::success("Package installation (dry-run: skipped)");
        return Ok(());
    }

    let mut args = vec!["dnf", "install", "-y"];
    args.extend(PACKAGES.iter().copied());

    let output = Command::new("sudo").args(&args).output()?;

    log::log_output(&String::from_utf8_lossy(&output.stdout));

    if output.status.success() {
        ui::success("All packages installed");
        log::log("Package installation complete");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        bail!("Failed to install packages");
    }
}

pub fn install_starship(dry_run: bool) -> Result<()> {
    ui::info("Installing Starship prompt...");

    if dry_run {
        ui::success("Would install Starship (dry-run)");
        return Ok(());
    }

    // Check if already installed
    if which::which("starship").is_ok() {
        ui::success("Starship already installed");
        return Ok(());
    }

    let cmd = "curl -sS https://starship.rs/install.sh | sh -s -- -y";
    log::log_command(cmd);

    let output = Command::new("sh")
        .args(["-c", "curl -sS https://starship.rs/install.sh | sh -s -- -y"])
        .output()?;

    log::log_output(&String::from_utf8_lossy(&output.stdout));

    if output.status.success() {
        ui::success("Starship installed");
        log::log("Starship installation complete");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        bail!("Failed to install Starship");
    }
}

pub fn install_rust(dry_run: bool) -> Result<()> {
    ui::info("Installing Rust toolchain...");

    if dry_run {
        ui::success("Would install Rust (dry-run)");
        return Ok(());
    }

    // Check if already installed
    if which::which("rustc").is_ok() && which::which("cargo").is_ok() {
        ui::success("Rust already installed");
        return Ok(());
    }

    let cmd = "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y";
    log::log_command(cmd);

    let output = Command::new("sh")
        .args(["-c", "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"])
        .output()?;

    log::log_output(&String::from_utf8_lossy(&output.stdout));

    if output.status.success() {
        ui::success("Rust installed");
        log::log("Rust installation complete");
        ui::info("Note: You may need to restart your shell or run 'source ~/.cargo/env'");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        bail!("Failed to install Rust");
    }
}

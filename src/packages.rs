use anyhow::{bail, Result};
use std::process::Command;

use crate::{log, ui};

const PACKAGES: &[&str] = &[
    // Hyprland and Wayland
    "hyprland",
    "hyprland-qtutils",
    "xdg-desktop-portal-hyprland",
    "xdg-desktop-portal-gtk",
    // Terminal
    "foot",
    // Shell and tools
    "fish",
    // Greetd
    "greetd",
    "tuigreet",
    // Qt6 (for building quickshell)
    "qt6-qtbase-devel",
    "qt6-qtdeclarative-devel",
    "qt6-qtwayland-devel",
    "qt6-qtsvg-devel",
    "qt6-qtshadertools-devel",
    "spirv-tools",
    "cli11-devel",
    "jemalloc-devel",
    // Wayland
    "wayland-devel",
    "wayland-protocols-devel",
    "libdrm-devel",
    "mesa-libgbm-devel",
    "pipewire-devel",
    // Quickshell optional deps
    "polkit-devel",
    "pam-devel",
    "pkgconf-pkg-config",
    // Build tools
    "cmake",
    "ninja-build",
    "gcc-c++",
    "git",
    "curl",
    "tar",
    // Python build tools for caelestia-cli
    "python3-devel",
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

pub fn install_quickshell(dry_run: bool) -> Result<()> {
    ui::info("Installing Quickshell from source...");

    if dry_run {
        ui::success("Would build Quickshell from source (dry-run)");
        return Ok(());
    }

    // Check if already installed
    if which::which("quickshell").is_ok() {
        ui::success("Quickshell already installed");
        return Ok(());
    }

    let build_dir = std::path::PathBuf::from("/tmp/quickshell");

    // Clone repo
    if build_dir.exists() {
        std::fs::remove_dir_all(&build_dir).ok();
    }

    let cmd = "git clone --depth 1 https://github.com/outfoxxed/quickshell /tmp/quickshell";
    log::log_command(cmd);

    let output = Command::new("git")
        .args(["clone", "--depth", "1", "https://github.com/outfoxxed/quickshell", "/tmp/quickshell"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        bail!("Failed to clone Quickshell");
    }

    ui::success("Cloned Quickshell");

    // Configure with CMake
    ui::info("Configuring Quickshell...");
    let cmd = "cmake -B build -S /tmp/quickshell -G Ninja -DCMAKE_BUILD_TYPE=Release -DUSE_JEMALLOC=ON -DX11=OFF";
    log::log_command(cmd);

    let output = Command::new("cmake")
        .args([
            "-B", "/tmp/quickshell/build",
            "-S", "/tmp/quickshell",
            "-G", "Ninja",
            "-DCMAKE_BUILD_TYPE=Release",
            "-DUSE_JEMALLOC=ON",
            "-DX11=OFF",
            "-DCRASH_REPORTER=OFF",
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        bail!("Failed to configure Quickshell");
    }

    ui::success("Configured Quickshell");

    // Build
    ui::info("Building Quickshell (this may take a while)...");
    let cmd = "cmake --build /tmp/quickshell/build";
    log::log_command(cmd);

    let output = Command::new("cmake")
        .args(["--build", "/tmp/quickshell/build"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        bail!("Failed to build Quickshell");
    }

    ui::success("Built Quickshell");

    // Install
    ui::info("Installing Quickshell...");
    let cmd = "sudo cmake --install /tmp/quickshell/build";
    log::log_command(cmd);

    let output = Command::new("sudo")
        .args(["cmake", "--install", "/tmp/quickshell/build"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        bail!("Failed to install Quickshell");
    }

    ui::success("Quickshell installed");
    log::log("Quickshell installation complete");

    Ok(())
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

pub fn install_nerd_font(dry_run: bool) -> Result<()> {
    ui::info("Installing JetBrains Mono Nerd Font...");

    if dry_run {
        ui::success("Would install JetBrains Mono Nerd Font (dry-run)");
        return Ok(());
    }

    let font_dir = dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("~"))
        .join(".local/share/fonts");

    // Check if already installed
    let font_check = font_dir.join("JetBrainsMonoNerdFont-Regular.ttf");
    if font_check.exists() {
        ui::success("JetBrains Mono Nerd Font already installed");
        return Ok(());
    }

    std::fs::create_dir_all(&font_dir)?;

    let tmp_dir = std::path::PathBuf::from("/tmp/nerd-fonts");
    if tmp_dir.exists() {
        std::fs::remove_dir_all(&tmp_dir).ok();
    }
    std::fs::create_dir_all(&tmp_dir)?;

    // Download font
    ui::info("Downloading JetBrains Mono Nerd Font...");
    let url = "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/JetBrainsMono.tar.xz";
    let cmd = format!("curl -sL {} -o /tmp/nerd-fonts/JetBrainsMono.tar.xz", url);
    log::log_command(&cmd);

    let output = Command::new("curl")
        .args(["-sL", url, "-o", "/tmp/nerd-fonts/JetBrainsMono.tar.xz"])
        .output()?;

    if !output.status.success() {
        bail!("Failed to download JetBrains Mono Nerd Font");
    }

    // Extract
    ui::info("Extracting font...");
    let output = Command::new("tar")
        .args(["-xf", "/tmp/nerd-fonts/JetBrainsMono.tar.xz", "-C", "/tmp/nerd-fonts"])
        .output()?;

    if !output.status.success() {
        bail!("Failed to extract font archive");
    }

    // Copy ttf files
    for entry in std::fs::read_dir(&tmp_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map(|e| e == "ttf").unwrap_or(false) {
            let dest = font_dir.join(path.file_name().unwrap());
            std::fs::copy(&path, &dest)?;
        }
    }

    // Update font cache
    let _ = Command::new("fc-cache").args(["-fv"]).output();

    ui::success("JetBrains Mono Nerd Font installed");
    log::log("Nerd font installation complete");

    Ok(())
}

use anyhow::{bail, Result};
use std::process::Command;

use crate::{log, ui};

const PACKAGES: &[&str] = &[
    // Hyprland and Wayland
    "hyprland",
    // "hyprland-qtutils",
    "xdg-desktop-portal-hyprland",
    "xdg-desktop-portal-gtk",
    "hyprutils-devel",
    "hyprlang-devel",
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
    "qt6-qtdeclarative-static",
    "qt6-qtbase-static",
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
    "libqalculate-devel",
    "aubio-devel",
    // Cava build deps
    "alsa-lib-devel",
    "fftw-devel",
    "pulseaudio-libs-devel",
    "autoconf-archive",
    "iniparser-devel",
    "libtool",
    // Build tools
    "cmake",
    "ninja-build",
    "gcc-c++",
    "git",
    "curl",
    "tar",
    "unzip",
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
    "google-rubik-fonts",
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
        return Ok(());
    }

    let output = Command::new("sudo")
        .args(["dnf", "install", "-y"])
        .args(PACKAGES)
        .output()?;

    if output.status.success() {
        ui::success("Packages installed successfully");
        log::log("Package installation completed");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&format!("Package installation failed: {}", stderr));
        bail!("Failed to install packages: {}", stderr);
    }

    Ok(())
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
        .arg("-c")
        .arg(cmd)
        .output()?;

    if output.status.success() {
        ui::success("Starship installed successfully");
        log::log("Starship installation completed");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&format!("Starship installation failed: {}", stderr));
        bail!("Failed to install Starship: {}", stderr);
    }

    Ok(())
}

pub fn install_rust(dry_run: bool) -> Result<()> {
    ui::info("Installing Rust...");

    if dry_run {
        ui::success("Would install Rust (dry-run)");
        return Ok(());
    }

    // Check if already installed
    if which::which("cargo").is_ok() {
        ui::success("Rust already installed");
        return Ok(());
    }

    let cmd = "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y";
    log::log_command(cmd);

    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()?;

    if output.status.success() {
        ui::success("Rust installed successfully");
        log::log("Rust installation completed");
        ui::info("Note: You may need to restart your shell or run 'source ~/.cargo/env'");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&format!("Rust installation failed: {}", stderr));
        bail!("Failed to install Rust: {}", stderr);
    }

    Ok(())
}

pub fn install_hyprland_qt_support(dry_run: bool) -> Result<()> {
    ui::info("Installing hyprland-qt-support...");

    if dry_run {
        ui::success("Would install hyprland-qt-support (dry-run)");
        return Ok(());
    }

    let cmd = "sudo dnf install -y hyprland-qt-support";
    log::log_command(cmd);

    let output = Command::new("sudo")
        .args(["dnf", "install", "-y", "hyprland-qt-support"])
        .output()?;

    if output.status.success() {
        ui::success("hyprland-qt-support installed successfully");
        log::log("hyprland-qt-support installation completed");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&format!("hyprland-qt-support installation failed: {}", stderr));
        ui::warning("Failed to install hyprland-qt-support (may not be available in your repos)");
    }

    Ok(())
}

pub fn install_hyprland_qtutils(dry_run: bool) -> Result<()> {
    ui::info("Installing hyprland-qtutils...");

    if dry_run {
        ui::success("Would install hyprland-qtutils (dry-run)");
        return Ok(());
    }

    let cmd = "sudo dnf install -y hyprland-qtutils";
    log::log_command(cmd);

    let output = Command::new("sudo")
        .args(["dnf", "install", "-y", "hyprland-qtutils"])
        .output()?;

    if output.status.success() {
        ui::success("hyprland-qtutils installed successfully");
        log::log("hyprland-qtutils installation completed");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&format!("hyprland-qtutils installation failed: {}", stderr));
        ui::warning("Failed to install hyprland-qtutils (may not be available in your repos)");
    }

    Ok(())
}

pub fn install_quickshell(dry_run: bool) -> Result<()> {
    ui::info("Building Quickshell from source...");

    if dry_run {
        ui::success("Would build Quickshell (dry-run)");
        return Ok(());
    }

    // Clone Quickshell
    ui::info("Cloning Quickshell...");
    let cmd = "git clone --recursive https://github.com/qt6cn/quickshell /tmp/quickshell";
    log::log_command(cmd);

    let output = Command::new("git")
        .args([
            "clone",
            "--recursive",
            "https://github.com/qt6cn/quickshell",
            "/tmp/quickshell",
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&format!("Failed to clone Quickshell: {}", stderr));
        bail!("Failed to clone Quickshell: {}", stderr);
    }

    ui::success("Cloned Quickshell");

    // Configure with CMake
    ui::info("Configuring Quickshell...");
    let cmd = "cmake -B build -S /tmp/quickshell -G Ninja -DCMAKE_BUILD_TYPE=Release -DUSE_JEMALLOC=ON -DX11=OFF";
    log::log_command(cmd);

    let output = Command::new("cmake")
        .args([
            "-B",
            "/tmp/quickshell/build",
            "-S",
            "/tmp/quickshell",
            "-G",
            "Ninja",
            "-DCMAKE_BUILD_TYPE=Release",
            "-DUSE_JEMALLOC=ON",
            "-DX11=OFF",
            "-DCRASH_REPORTER=OFF",
            "-DQt6_DIR=/usr/lib64/cmake/Qt6",
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&format!("Failed to configure Quickshell: {}", stderr));
        bail!("Failed to configure Quickshell: {}", stderr);
    }

    // Build
    ui::info("Building Quickshell...");
    let cmd = "cmake --build /tmp/quickshell/build";
    log::log_command(cmd);

    let output = Command::new("cmake")
        .args(["--build", "/tmp/quickshell/build"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&format!("Failed to build Quickshell: {}", stderr));
        bail!("Failed to build Quickshell: {}", stderr);
    }

    // Install
    ui::info("Installing Quickshell...");
    let cmd = "sudo cmake --install /tmp/quickshell/build";
    log::log_command(cmd);

    let output = Command::new("sudo")
        .args(["cmake", "--install", "/tmp/quickshell/build"])
        .output()?;

    if output.status.success() {
        ui::success("Quickshell installed successfully");
        log::log("Quickshell installation completed");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&format!("Failed to install Quickshell: {}", stderr));
        bail!("Failed to install Quickshell: {}", stderr);
    }

    // Cleanup
    std::fs::remove_dir_all("/tmp/quickshell").ok();

    Ok(())
}

pub fn install_cava(dry_run: bool) -> Result<()> {
    ui::info("Installing Cava...");

    if dry_run {
        ui::success("Would install Cava (dry-run)");
        return Ok(());
    }

    // Clone Cava
    ui::info("Cloning Cava...");
    let cmd = "git clone https://github.com/karlstav/cava /tmp/cava";
    log::log_command(cmd);

    let output = Command::new("git")
        .args(["clone", "https://github.com/karlstav/cava", "/tmp/cava"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&format!("Failed to clone Cava: {}", stderr));
        bail!("Failed to clone Cava: {}", stderr);
    }

    ui::success("Cloned Cava");

    // Build and install
    ui::info("Building and installing Cava...");
    let cmd = "cd /tmp/cava && ./autogen.sh && ./configure && make && sudo make install";
    log::log_command(cmd);

    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()?;

    if output.status.success() {
        ui::success("Cava installed successfully");
        log::log("Cava installation completed");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&format!("Failed to install Cava: {}", stderr));
        bail!("Failed to install Cava: {}", stderr);
    }

    // Cleanup
    std::fs::remove_dir_all("/tmp/cava").ok();

    Ok(())
}

pub fn install_fonts(dry_run: bool) -> Result<()> {
    ui::info("Installing Fonts...");

    if dry_run {
        ui::success("Would install Fonts (dry-run)");
        return Ok(());
    }

    let font_dir = dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("~"))
        .join(".local/share/fonts");

    std::fs::create_dir_all(&font_dir)?;

    // 1. Material Symbols Rounded
    let mat_target = font_dir.join("MaterialSymbolsRounded.ttf");
    if !mat_target.exists() {
        ui::info("Downloading Material Symbols Rounded...");
        let url = "https://github.com/google/material-design-icons/raw/master/variablefont/MaterialSymbolsRounded%5BFILL,GRAD,opsz,wght%5D.ttf";
        let cmd = format!("curl -L -o {:?} {}", mat_target, url);
        log::log_command(&cmd);

        let output = Command::new("curl")
            .args(["-L", "-o", mat_target.to_str().unwrap(), url])
            .output()?;

        if output.status.success() {
            ui::success("Material Symbols Rounded downloaded");
        } else {
            ui::warning("Failed to download Material Symbols Rounded");
        }
    } else {
        ui::success("Material Symbols Rounded already installed");
    }

    // 2. Caskaydia Cove Nerd Font (via CaskaydiaCode.zip)
    // Check if one of the files exists as a proxy
    let cas_target = font_dir.join("CaskaydiaCoveNerdFont-Regular.ttf");
    if !cas_target.exists() {
        ui::info("Downloading Caskaydia Cove Nerd Font...");
        let url = "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.3.0/CascadiaCode.zip";
        let zip_path = "/tmp/CaskaydiaCove.zip";
        
        // Download
        let output = Command::new("curl")
            .args(["-L", "-o", zip_path, url])
            .output()?;
        
        if output.status.success() {
            ui::info("Extracting Caskaydia Cove...");
            // Extract all files first
            let output = Command::new("unzip")
                .args(["-o", zip_path, "-d", font_dir.to_str().unwrap()])
                .output()?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                log::log_error(&format!("Failed to extract Caskaydia Cove: {}", stderr));
                ui::warning("Failed to extract Caskaydia Cove");
            } else {
                // Clean up non-TTF files
                for entry in std::fs::read_dir(&font_dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    if let Some(fname) = path.file_name() {
                        let name = fname.to_str().unwrap_or("");
                        if name.starts_with("CaskaydiaCove") && !name.ends_with(".ttf") {
                            std::fs::remove_file(path).ok();
                        }
                    }
                }
                ui::success("Caskaydia Cove extracted successfully");
            }
            std::fs::remove_file(zip_path).ok();
        } else {
            ui::warning("Failed to download Caskaydia Cove");
        }
    } else {
        ui::success("Caskaydia Cove Nerd Font already installed");
    }
    
    // Update font cache
    let cache_output = Command::new("fc-cache").args(["-fv"]).output();
    if !cache_output.as_ref().map_or(true, |o| o.status.success()) {
        ui::warning("Font cache update failed, fonts may not be available immediately");
    }

    ui::success("Fonts installed");
    log::log("Font installation complete");

    Ok(())
}

pub fn verify_qt_packages() -> Result<()> {
    ui::info("Verifying Qt development packages...");
    
    let critical_packages = &[
        "qt6-qtbase-devel",
        "qt6-qtdeclarative-devel", 
        "qt6-qtwayland-devel",
        "qt6-qtbase-private-devel",
        "cmake",
        "ninja-build",
        "gcc-c++",
        "pkgconf",
    ];
    
    for pkg in critical_packages {
        let cmd = format!("rpm -q {}", pkg);
        log::log_command(&cmd);
        
        let output = Command::new("rpm")
            .args(["-q", pkg])
            .output()?;
            
        if !output.status.success() {
            ui::error(&format!("Critical package not found: {}", pkg));
            bail!("Missing required package: {}", pkg);
        }
    }
    
    ui::success("All critical Qt packages are installed");
    
    // Verify Qt6QuickPrivate component is available
    let quickprivate_path = "/usr/lib64/cmake/Qt6QuickPrivate/Qt6QuickPrivateConfig.cmake";
    if !std::path::Path::new(quickprivate_path).exists() {
        bail!("Qt6QuickPrivate component not found at {}. Please ensure qt6-qtdeclarative-devel is properly installed.", quickprivate_path);
    }
    ui::success("Qt6QuickPrivate component is available");
    
    // Verify Qt6WaylandClientPrivate component is available
    let wayland_private_path = "/usr/lib64/cmake/Qt6WaylandClientPrivate/Qt6WaylandClientPrivateConfig.cmake";
    if !std::path::Path::new(wayland_private_path).exists() {
        bail!("Qt6WaylandClientPrivate component not found at {}. Please ensure qt6-qtwayland-devel is properly installed.", wayland_private_path);
    }
    ui::success("Qt6WaylandClientPrivate component is available");
    
    Ok(())
}

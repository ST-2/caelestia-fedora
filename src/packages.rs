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

pub fn install_cava(dry_run: bool) -> Result<()> {
    ui::info("Installing Cava from source...");

    if dry_run {
        ui::success("Would build Cava from source (dry-run)");
        return Ok(());
    }

    // Check if already installed via pkg-config check
    // If /usr/lib64/pkgconfig/cava.pc exists, we assume it's done.
    if std::path::Path::new("/usr/lib64/pkgconfig/cava.pc").exists() {
        ui::success("Cava already installed (checked pkg-config)");
        return Ok(());
    }

    let build_dir = std::path::PathBuf::from("/tmp/cava-build");

    // Clone repo
    if build_dir.exists() {
        std::fs::remove_dir_all(&build_dir).ok();
    }

    let cmd = "git clone --depth 1 https://github.com/karlstav/cava /tmp/cava-build";
    log::log_command(cmd);

    let output = Command::new("git")
        .args(["clone", "--depth", "1", "https://github.com/karlstav/cava", "/tmp/cava-build"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        bail!("Failed to clone Cava");
    }

    ui::success("Cloned Cava");

    // Configure with CMake (builds cavacore static lib)
    ui::info("Configuring Cava...");
    // CAVACORE.md says to use root CMakeLists
    let cmd = "cmake -B build -S /tmp/cava-build -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_POSITION_INDEPENDENT_CODE=ON";
    log::log_command(cmd);

    let output = Command::new("cmake")
        .args([
            "-B", "/tmp/cava-build/build",
            "-S", "/tmp/cava-build",
            "-G", "Ninja",
            "-DCMAKE_BUILD_TYPE=Release",
            "-DCMAKE_POSITION_INDEPENDENT_CODE=ON",
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        bail!("Failed to configure Cava");
    }

    // Build
    ui::info("Building Cava...");
    let cmd = "cmake --build /tmp/cava-build/build";
    log::log_command(cmd);

    let output = Command::new("cmake")
        .args(["--build", "/tmp/cava-build/build"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        bail!("Failed to build Cava");
    }

    ui::success("Built Cava");

    // Manual Install
    ui::info("Installing Cava library and headers...");

    // Install header
    let cmd = "sudo cp /tmp/cava-build/cavacore.h /usr/include/";
    log::log_command(cmd);
    Command::new("sudo")
        .args(["cp", "/tmp/cava-build/cavacore.h", "/usr/include/"])
        .status()?;

    // Install library
    let cmd = "sudo cp /tmp/cava-build/build/libcavacore.a /usr/lib64/";
    log::log_command(cmd);
    Command::new("sudo")
        .args(["cp", "/tmp/cava-build/build/libcavacore.a", "/usr/lib64/"])
        .status()?;

    // Create pkg-config file
    ui::info("Creating cava.pc...");
    let pc_content = r#"prefix=/usr
exec_prefix=${prefix}
libdir=${exec_prefix}/lib64
includedir=${prefix}/include

Name: cava
Description: Cava Core Library
Version: 0.10.3
Libs: -L${libdir} -lcavacore -lfftw3 -lm -liniparser
Cflags: -I${includedir}
"#;

    let pc_path = "/tmp/cava-build/cava.pc";
    std::fs::write(pc_path, pc_content)?;

    let cmd = "sudo cp /tmp/cava-build/cava.pc /usr/lib64/pkgconfig/";
    log::log_command(cmd);
    Command::new("sudo")
        .args(["cp", pc_path, "/usr/lib64/pkgconfig/"])
        .status()?;

    ui::success("Cava installed");
    log::log("Cava installation complete");

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

        if !output.status.success() {
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
            // Unzip content
            let output = Command::new("unzip")
                .args(["-o", zip_path, "-d", font_dir.to_str().unwrap(), "CaskaydiaCoveNerdFont*.ttf"])
                .output()?;
            
            if !output.status.success() {
                 ui::warning("Failed to extract Caskaydia Cove");
            }
            std::fs::remove_file(zip_path).ok();
        } else {
            ui::warning("Failed to download Caskaydia Cove");
        }
    } else {
        ui::success("Caskaydia Cove Nerd Font already installed");
    }
    
    // Update font cache
    let _ = Command::new("fc-cache").args(["-fv"]).output();

    ui::success("Fonts installed");
    log::log("Font installation complete");

    Ok(())
}

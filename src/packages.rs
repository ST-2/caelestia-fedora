use anyhow::{bail, Result};
use std::path::PathBuf;
use std::process::Command;

use crate::{log, ui};

// Critical Qt packages required for building Quickshell
const CRITICAL_QT_PACKAGES: &[&str] = &[
    "qt6-qtbase-devel",
    "qt6-qtdeclarative-devel",
    "qt6-qtwayland-devel",
    "qt6-qtsvg-devel",
    "qt6-qtshadertools-devel",
    "qt6-qtbase-private-devel",
    "qt6-qtconnectivity-devel",  // For Bluetooth (required by Quickshell)
];

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
    "qt6-qtbase-private-devel",     // For Qt6 private APIs (QuickPrivate)
    "qt6-qtdeclarative-devel",
    "qt6-qtdeclarative-static",
    "qt6-qtbase-static",
    "qt6-qtwayland-devel",
    "qt6-qtsvg-devel",
    "qt6-qtshadertools-devel",
    "qt6-qtconnectivity-devel",     // For Bluetooth (required by Quickshell)
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
    "eza",
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
    // File Manager
    "Thunar",
    // App Store
    "plasma-discover",
    "plasma-discover-flatpak",
    // CLI Utilities
    "zoxide",
    "fzf",
];

pub fn install_all(dry_run: bool) -> Result<()> {
    ui::info("Installing packages via dnf...");

    let pkg_list = PACKAGES.join(" ");
    let cmd = format!("sudo dnf install -y --allowerasing {}", pkg_list);
    log::log_command(&cmd);

    if dry_run {
        ui::info("Would install the following packages:");
        for pkg in PACKAGES {
            println!("  - {}", pkg);
        }
        ui::success("Package installation (dry-run: skipped)");
        return Ok(());
    }

    // Use --allowerasing to resolve conflicts between COPR and official repos
    let mut args = vec!["dnf", "install", "-y", "--allowerasing"];
    args.extend(PACKAGES.iter().copied());

    let output = Command::new("sudo").args(&args).output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    log::log_output(&stdout);

    // Check for skipped packages due to conflicts or broken dependencies
    if stdout.contains("Skipping packages with conflicts") || stdout.contains("Skipping packages with broken dependencies") {
        ui::warning("Some packages were skipped due to conflicts or broken dependencies");
        log::log("WARNING: Some packages were skipped due to conflicts or broken dependencies");
        
        // Check if critical Qt packages were skipped by looking for package names
        // in the "Skipping packages" section of the output
        let mut skipped_critical = Vec::new();
        
        // Look for lines containing both "Skipping" context and package names
        let lines: Vec<&str> = stdout.lines().collect();
        let mut in_skipping_section = false;
        
        for line in &lines {
            if line.contains("Skipping packages") {
                in_skipping_section = true;
                continue;
            }
            // End of skipping section when we hit an empty line or new section
            if in_skipping_section && (line.trim().is_empty() || line.starts_with("Installing") || line.starts_with("Upgrading")) {
                in_skipping_section = false;
            }
            
            if in_skipping_section {
                for pkg in CRITICAL_QT_PACKAGES.iter().take(3) { // Check main Qt packages
                    if line.contains(pkg) {
                        skipped_critical.push(*pkg);
                    }
                }
            }
        }
        
        if !skipped_critical.is_empty() {
            ui::error("Critical Qt development packages were skipped:");
            for pkg in &skipped_critical {
                ui::error(&format!("  - {}", pkg));
            }
            ui::info("Attempting to install Qt packages with conflict resolution...");
            
            // Try to install Qt packages with allowerasing explicitly
            let mut qt_args = vec!["dnf", "install", "-y", "--allowerasing"];
            qt_args.extend(CRITICAL_QT_PACKAGES.iter().copied());
            
            let qt_output = Command::new("sudo")
                .args(&qt_args)
                .output()?;
            
            let qt_stdout = String::from_utf8_lossy(&qt_output.stdout);
            log::log_output(&qt_stdout);
            
            if !qt_output.status.success() || qt_stdout.contains("Skipping packages") {
                let qt_stderr = String::from_utf8_lossy(&qt_output.stderr);
                log::log_error(&qt_stderr);
                bail!("Failed to install critical Qt packages. You may need to manually resolve package conflicts.\n\
                       Try running: sudo dnf install --allowerasing qt6-qtbase-devel qt6-qtdeclarative-devel qt6-qtwayland-devel");
            }
            
            ui::success("Qt packages installed with conflict resolution");
        }
    }

    if output.status.success() {
        ui::success("Package installation complete");
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

    // Verify critical Qt packages are installed
    verify_qt_packages()?;

    let build_dir = std::path::PathBuf::from("/tmp/quickshell");

    // Clone repo
    if build_dir.exists() {
        std::fs::remove_dir_all(&build_dir).ok();
    }

    let cmd = "git clone --depth 1 https://git.outfoxxed.me/outfoxxed/quickshell.git /tmp/quickshell";
    log::log_command(cmd);

    let output = Command::new("git")
        .args(["clone", "--depth", "1", "https://git.outfoxxed.me/outfoxxed/quickshell.git", "/tmp/quickshell"])
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
            "-DQt6_DIR=/usr/lib64/cmake/Qt6",
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Log both stdout and stderr for CMake configure errors
        log::log("=== CMAKE CONFIGURE STDOUT ===");
        log::log_output(&stdout);
        log::log("=== CMAKE CONFIGURE STDERR ===");
        log::log_error(&stderr);
        bail!("Failed to configure Quickshell. Check ~/.cache/caelestia-installer/install.log for details.");
    }

    ui::success("Configured Quickshell");

    // Build
    ui::info("Building Quickshell (this may take a while)...");
    let jobs = crate::system::get_ninja_jobs();
    let mut build_args = vec!["--build", "/tmp/quickshell/build"];
    let jobs_str;
    if jobs > 0 {
        build_args.push("-j");
        jobs_str = jobs.to_string();
        build_args.push(&jobs_str);
    }

    let output = Command::new("cmake")
        .args(&build_args)
        .output()?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Always log both for debugging
    log::log("=== BUILD STDOUT ===");
    log::log_output(&stdout);
    log::log("=== BUILD STDERR ===");
    log::log_error(&stderr);

    if !output.status.success() {
        ui::error("Build failed!");
        
        // Print the last 2000 chars of stdout which likely contains the error
        if !stdout.is_empty() {
            let start = stdout.len().saturating_sub(2000);
            println!("STDOUT (last 2000 chars):\n{}", &stdout[start..]);
        }
        if !stderr.is_empty() {
            println!("STDERR:\n{}", stderr);
        }

        crate::system::check_oom_event();
        bail!("Failed to build Quickshell. Check ~/.cache/caelestia-installer/install.log for details.");
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
    let jobs = crate::system::get_ninja_jobs();
    let mut build_args = vec!["--build", "/tmp/cava-build/build"];
    let jobs_str;
    if jobs > 0 {
        build_args.push("-j");
        jobs_str = jobs.to_string();
        build_args.push(&jobs_str);
    }

    let output = Command::new("cmake")
        .args(&build_args)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        log::log("=== Cava BUILD STDOUT ===");
        log::log_output(&stdout);
        log::log_error(&stderr);
        crate::system::check_oom_event();
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

    // Create cava directory and symlink as cavacore.h for compatibility
    let cmd = "sudo mkdir -p /usr/include/cava";
    log::log_command(cmd);
    Command::new("sudo")
        .args(["mkdir", "-p", "/usr/include/cava"])
        .status()?;

    let cmd = "sudo ln -sf /usr/include/cavacore.h /usr/include/cava/cavacore.h";
    log::log_command(cmd);
    Command::new("sudo")
        .args(["ln", "-sf", "/usr/include/cavacore.h", "/usr/include/cava/cavacore.h"])
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
        let _stdout = String::from_utf8_lossy(&output.stdout);
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

    // 3. JetBrains Mono Nerd Font (required by foot.ini upstream)
    let jb_target = font_dir.join("JetBrainsMonoNerdFont-Regular.ttf");
    if !jb_target.exists() {
        ui::info("Downloading JetBrains Mono Nerd Font...");
        let url = "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.3.0/JetBrainsMono.zip";
        let zip_path = "/tmp/JetBrainsMono.zip";
        
        // Download
        let output = Command::new("curl")
            .args(["-L", "-o", zip_path, url])
            .output()?;
        
        if output.status.success() {
            ui::info("Extracting JetBrains Mono...");
            // Unzip content
            let output = Command::new("unzip")
                .args(["-o", zip_path, "-d", font_dir.to_str().unwrap(), "JetBrainsMonoNerdFont*.ttf"])
                .output()?;
            
            if !output.status.success() {
                 ui::warning("Failed to extract JetBrains Mono");
            }
            std::fs::remove_file(zip_path).ok();
        } else {
            ui::warning("Failed to download JetBrains Mono");
        }
    } else {
        ui::success("JetBrains Mono Nerd Font already installed");
    }
    
    // Update font cache
    let _ = Command::new("fc-cache").args(["-fv"]).output();

    ui::success("Fonts installed");
    log::log("Font installation complete");

    Ok(())
}

pub fn install_hyprland_qt_support(dry_run: bool) -> Result<()> {
    ui::info("Installing hyprland-qt-support...");
    
    if dry_run {
        ui::success("Would install hyprland-qt-support (dry-run)");
        return Ok(());
    }

    if std::path::Path::new("/usr/lib64/libhyprland-qt-support.so").exists() {
        ui::success("hyprland-qt-support already installed");
        return Ok(());
    }

    let tmp_dir = std::path::PathBuf::from("/tmp/hyprland-qt-support");
    if tmp_dir.exists() {
        std::fs::remove_dir_all(&tmp_dir).ok();
    }

    ui::info("Cloning hyprland-qt-support...");
    Command::new("git")
        .args(["clone", "https://github.com/hyprwm/hyprland-qt-support", "/tmp/hyprland-qt-support"])
        .output()?;

    ui::info("Configuring hyprland-qt-support...");
    let output = Command::new("cmake")
        .args([
            "-B", "/tmp/hyprland-qt-support/build",
            "-S", "/tmp/hyprland-qt-support",
            "-G", "Ninja",
            "-DCMAKE_BUILD_TYPE=Release",
            "-DCMAKE_INSTALL_PREFIX=/usr",
            "-DCMAKE_INSTALL_LIBDIR=lib64",
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let _stdout = String::from_utf8_lossy(&output.stdout);
        log::log_error(&stderr);
        bail!("Failed to configure hyprland-qt-support");
    }
    
    ui::info("Building hyprland-qt-support...");
    let jobs = crate::system::get_ninja_jobs();
    let mut build_args = vec!["--build", "/tmp/hyprland-qt-support/build"];
    let jobs_str;
    if jobs > 0 {
        build_args.push("-j");
        jobs_str = jobs.to_string();
        build_args.push(&jobs_str);
    }

    let output = Command::new("cmake")
        .args(&build_args)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        log::log("=== hyprland-qt-support BUILD STDOUT ===");
        log::log_output(&stdout);
        log::log_error(&stderr);
        crate::system::check_oom_event();
        bail!("Failed to build hyprland-qt-support");
    }

    ui::info("Installing hyprland-qt-support...");
    Command::new("sudo")
        .args(["cmake", "--install", "/tmp/hyprland-qt-support/build"])
        .status()?;

    ui::success("Installed hyprland-qt-support");
    Ok(())
}

pub fn install_hyprland_qtutils(dry_run: bool) -> Result<()> {
    ui::info("Installing hyprland-qtutils...");
    
    if dry_run {
        ui::success("Would install hyprland-qtutils (dry-run)");
        return Ok(());
    }

    if which::which("hyprland-dialog").is_ok() {
         ui::success("hyprland-qtutils already installed");
         return Ok(());
    }

    // Verify critical Qt packages are installed
    verify_qt_packages()?;

    let tmp_dir = std::path::PathBuf::from("/tmp/hyprland-qtutils");
    if tmp_dir.exists() {
        std::fs::remove_dir_all(&tmp_dir).ok();
    }

    ui::info("Cloning hyprland-qtutils...");
    Command::new("git")
        .args(["clone", "https://github.com/hyprwm/hyprland-qtutils", "/tmp/hyprland-qtutils"])
        .output()?;

    ui::info("Configuring hyprland-qtutils...");
    let output = Command::new("cmake")
        .args([
            "-B", "/tmp/hyprland-qtutils/build",
            "-S", "/tmp/hyprland-qtutils",
            "-G", "Ninja",
            "-DCMAKE_BUILD_TYPE=Release",
            "-DCMAKE_INSTALL_PREFIX=/usr",
            "-DQt6_DIR=/usr/lib64/cmake/Qt6",
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let _stdout = String::from_utf8_lossy(&output.stdout);
        log::log_error(&stderr);
        bail!("Failed to configure hyprland-qtutils");
    }
    
    ui::info("Building hyprland-qtutils...");
    let jobs = crate::system::get_ninja_jobs();
    let mut build_args = vec!["--build", "/tmp/hyprland-qtutils/build"];
    let jobs_str;
    if jobs > 0 {
        build_args.push("-j");
        jobs_str = jobs.to_string();
        build_args.push(&jobs_str);
    }

    let output = Command::new("cmake")
        .args(&build_args)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        log::log("=== hyprland-qtutils BUILD STDOUT ===");
        log::log_output(&stdout);
        log::log_error(&stderr);
        crate::system::check_oom_event();
        bail!("Failed to build hyprland-qtutils");
    }

    ui::info("Installing hyprland-qtutils...");
    Command::new("sudo")
        .args(["cmake", "--install", "/tmp/hyprland-qtutils/build"])
        .status()?;

    ui::success("Installed hyprland-qtutils");
    Ok(())
}

pub fn install_app2unit(dry_run: bool) -> Result<()> {
    ui::info("Installing app2unit...");
    
    if dry_run {
        ui::success("Would install app2unit (dry-run)");
        return Ok(());
    }
    
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    let bin_dir = home_dir.join(".local/bin");
    let app2unit_path = bin_dir.join("app2unit");
    
    // Check if already installed
    if app2unit_path.exists() {
        ui::success("app2unit already installed");
        return Ok(());
    }
    
    // Create .local/bin if it doesn't exist
    std::fs::create_dir_all(&bin_dir)?;
    
    // Download app2unit
    ui::info("Downloading app2unit...");
    let url = "https://raw.githubusercontent.com/VirtCode/app2unit/main/app2unit";
    let cmd = format!("curl -L -o {:?} {}", app2unit_path, url);
    log::log_command(&cmd);
    
    let output = Command::new("curl")
        .args(["-L", "-o", app2unit_path.to_str().unwrap(), url])
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        bail!("Failed to download app2unit");
    }
    
    // Make executable
    ui::info("Making app2unit executable...");
    let cmd = format!("chmod +x {:?}", app2unit_path);
    log::log_command(&cmd);
    
    let output = Command::new("chmod")
        .args(["+x", app2unit_path.to_str().unwrap()])
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        bail!("Failed to make app2unit executable");
    }
    
    ui::success("app2unit installed successfully");
    log::log("app2unit installation complete");
    
    Ok(())
}

fn verify_qt_packages() -> Result<()> {
    ui::info("Verifying Qt development packages...");
    
    // Additional build tools needed alongside Qt packages
    let build_tools = &[
        "cmake",
        "ninja-build",
        "gcc-c++",
        "pkgconf",
        "fftw-devel",
        "iniparser-devel",
        "libqalculate-devel",
        "pipewire-devel",
        "aubio-devel",
    ];
    
    let mut missing = Vec::new();
    
    // Check critical Qt packages
    for pkg in CRITICAL_QT_PACKAGES {
        let output = Command::new("rpm")
            .args(["-q", pkg])
            .output()?;
            
        if !output.status.success() {
            missing.push(*pkg);
        }
    }
    
    // Check build tools
    for pkg in build_tools {
        let output = Command::new("rpm")
            .args(["-q", pkg])
            .output()?;
            
        if !output.status.success() {
            missing.push(*pkg);
        }
    }
    
    if !missing.is_empty() {
        ui::warning("Missing critical packages:");
        for pkg in &missing {
            ui::warning(&format!("  - {}", pkg));
        }
        
        ui::info("Installing missing packages with conflict resolution...");
        let mut args = vec!["dnf", "install", "-y", "--allowerasing"];
        args.extend(missing.iter().copied());
        
        let output = Command::new("sudo").args(&args).output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        log::log_output(&stdout);
        
        // Check if packages were skipped even after using --allowerasing
        if stdout.contains("Skipping packages") {
            ui::error("Some packages could not be installed due to unresolvable conflicts.");
            ui::info("This may be caused by conflicting packages from COPR repositories.");
            ui::info("Try the following manual steps:");
            ui::info("  1. sudo dnf remove hyprland-qt-support hyprland-qtutils");
            ui::info("  2. sudo dnf install --allowerasing qt6-qtbase-devel qt6-qtdeclarative-devel");
            ui::info("  3. Re-run this installer");
            log::log_error(&format!("Package conflicts detected, stdout: {}", stdout));
            bail!("Failed to install Qt packages due to repository conflicts");
        }
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::log_error(&stderr);
            bail!("Failed to install missing packages");
        }
        
        // Verify the packages were actually installed after the install attempt
        let mut still_missing = Vec::new();
        for pkg in &missing {
            let verify_output = Command::new("rpm")
                .args(["-q", pkg])
                .output()?;
            if !verify_output.status.success() {
                still_missing.push(*pkg);
            }
        }
        
        if !still_missing.is_empty() {
            ui::error("The following packages are still missing after install attempt:");
            for pkg in &still_missing {
                ui::error(&format!("  - {}", pkg));
            }
            bail!("Failed to install required packages. Check for repository conflicts.");
        }
        
        ui::success("Missing packages installed");
    } else {
        ui::success("All critical packages are installed");
    }
    
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

use anyhow::{bail, Result};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use std::process::Command;

use crate::{log, ui};

const DOTFILES_REPO: &str = "https://github.com/caelestia-dots/caelestia.git";
const SHELL_REPO: &str = "https://github.com/caelestia-dots/shell.git";

/// Create user configuration files for Hyprland
pub fn create_user_configs(dry_run: bool) -> Result<()> {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
    let caelestia_dir = config_dir.join("caelestia");
    
    ui::info("Creating Hyprland user configuration files...");
    
    if dry_run {
        ui::success("Would create user configuration files (dry-run)");
        return Ok(());
    }
    
    // Create caelestia config directory
    fs::create_dir_all(&caelestia_dir)?;
    
    // Create hypr-user.conf with touchpad and window rules
    let hypr_user_conf = caelestia_dir.join("hypr-user.conf");
    let hypr_user_content = r#"# User-specific Hyprland configuration

# Touchpad settings
input {
    touchpad {
        natural_scroll = false
    }
}

# Windscribe window rules
windowrulev2 = float, class:Windscribe
windowrulev2 = center, class:Windscribe
"#;
    
    if !hypr_user_conf.exists() {
        fs::write(&hypr_user_conf, hypr_user_content)?;
        ui::success(&format!("Created {:?}", hypr_user_conf));
        log::log(&format!("Created hypr-user.conf at {:?}", hypr_user_conf));
    } else {
        ui::info("hypr-user.conf already exists, skipping...");
    }
    
    // Create hypr-vars.conf with gestures and window metrics
    let hypr_vars_conf = caelestia_dir.join("hypr-vars.conf");
    let hypr_vars_content = r#"# User-specific Hyprland variables

# Gesture settings
$workspaceSwipeFingers = 3

# Window metrics
$windowGapsOut = 10
$windowGapsIn = 5
$windowBorderSize = 2
"#;
    
    if !hypr_vars_conf.exists() {
        fs::write(&hypr_vars_conf, hypr_vars_content)?;
        ui::success(&format!("Created {:?}", hypr_vars_conf));
        log::log(&format!("Created hypr-vars.conf at {:?}", hypr_vars_conf));
    } else {
        ui::info("hypr-vars.conf already exists, skipping...");
    }
    
    ui::success("User configuration files created");
    
    Ok(())
}

/// Patch QML files to use absolute path for app2unit
pub fn patch_qml_app2unit(dry_run: bool) -> Result<()> {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
    let shell_dir = config_dir.join("quickshell/caelestia");
    
    ui::info("Patching QML files for app2unit absolute path...");
    
    if dry_run {
        ui::success("Would patch QML files (dry-run)");
        return Ok(());
    }
    
    if !shell_dir.exists() {
        ui::warning("Shell directory does not exist yet, skipping QML patching...");
        return Ok(());
    }
    
    // Get username for home directory
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    let username = home_dir.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("user");
    
    // Find all QML files
    let output = Command::new("find")
        .args([
            shell_dir.to_str().unwrap(),
            "-type", "f",
            "-name", "*.qml"
        ])
        .output()?;
    
    if !output.status.success() {
        ui::warning("Failed to find QML files");
        return Ok(());
    }
    
    let qml_files = String::from_utf8_lossy(&output.stdout);
    let mut patched_count = 0;
    
    for file_path in qml_files.lines() {
        if file_path.trim().is_empty() {
            continue;
        }
        
        let content = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        
        // Check if file contains app2unit reference without absolute path
        if content.contains("app2unit") && !content.contains(&format!("/home/{}", username)) {
            // Replace relative app2unit references with absolute path
            let patched_content = content
                .replace("\"app2unit\"", &format!("\"/home/{}/.local/bin/app2unit\"", username))
                .replace("'app2unit'", &format!("'/home/{}/.local/bin/app2unit'", username))
                .replace("app2unit ", &format!("/home/{}/.local/bin/app2unit ", username));
            
            if patched_content != content {
                fs::write(file_path, patched_content)?;
                ui::success(&format!("Patched {}", file_path));
                log::log(&format!("Patched app2unit path in {}", file_path));
                patched_count += 1;
            }
        }
    }
    
    if patched_count > 0 {
        ui::success(&format!("Patched {} QML file(s)", patched_count));
    } else {
        ui::info("No QML files needed patching");
    }
    
    Ok(())
}

pub fn clone_repos(dry_run: bool) -> Result<()> {
    let local_share = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("~/.local/share"));
    let dotfiles_dir = local_share.join("caelestia");

    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
    let shell_dir = config_dir.join("quickshell/caelestia");

    clone_repo(DOTFILES_REPO, &dotfiles_dir, dry_run)?;
    clone_repo(SHELL_REPO, &shell_dir, dry_run)?;

    // Patch deprecated gesture syntax in cloned dotfiles
    patch_gestures(&dotfiles_dir, dry_run)?;

    Ok(())
}

/// Upstream dotfiles already use Hyprland v0.51+ gesture syntax.
/// This function is kept as a no-op for compatibility but no patching is needed.
fn patch_gestures(_dotfiles_dir: &PathBuf, dry_run: bool) -> Result<()> {
    if dry_run {
        ui::info("Gesture syntax already compatible with Hyprland v0.51+ (dry-run)");
        return Ok(());
    }

    // Upstream caelestia dotfiles already use the modern gesture syntax:
    // gesture = $workspaceSwipeFingers, horizontal, workspace
    // No patching needed - the old deprecated `gestures { workspace_swipe }` block
    // is no longer present in upstream.
    
    ui::success("Gesture syntax already compatible with Hyprland v0.51+");
    Ok(())
}

fn clone_repo(url: &str, dest: &PathBuf, dry_run: bool) -> Result<()> {
    ui::info(&format!("Cloning {} to {:?}", url, dest));

    if dry_run {
        ui::success(&format!("Would clone to {:?} (dry-run)", dest));
        return Ok(());
    }

    if dest.exists() {
        ui::warning(&format!("{:?} already exists, pulling latest...", dest));
        let cmd = format!("git -C {:?} pull", dest);
        log::log_command(&cmd);

        let output = Command::new("git")
            .args(["-C", dest.to_str().unwrap(), "pull"])
            .output()?;

        if !output.status.success() {
            ui::warning("Pull failed, continuing anyway");
        }
        return Ok(());
    }

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }

    let cmd = format!("git clone {} {:?}", url, dest);
    log::log_command(&cmd);

    let output = Command::new("git")
        .args(["clone", url, dest.to_str().unwrap()])
        .output()?;

    log::log_output(&String::from_utf8_lossy(&output.stdout));

    if output.status.success() {
        ui::success(&format!("Cloned to {:?}", dest));
        log::log(&format!("Cloned {} to {:?}", url, dest));
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        bail!("Failed to clone repository: {}", url);
    }
}

pub fn build_shell(dry_run: bool) -> Result<()> {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
    let shell_dir = config_dir.join("quickshell/caelestia");

    ui::info("Building caelestia-shell...");

    if dry_run {
        ui::success("Would build caelestia-shell (dry-run)");
        return Ok(());
    }

    if !shell_dir.exists() {
        bail!("Shell directory does not exist: {:?}", shell_dir);
    }

    let build_dir = shell_dir.join("build");
    if build_dir.exists() {
        ui::info("Cleaning previous build...");
        fs::remove_dir_all(&build_dir)?;
    }
    fs::create_dir_all(&build_dir)?;

    // CMake configure
    ui::info("Configuring caelestia-shell...");
    let cmake_cmd = format!(
        "cmake -B {:?} -S {:?} -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr -DINSTALL_QMLDIR=/usr/lib64/qt6/qml -DINSTALL_LIBDIR=/usr/lib64/caelestia",
        build_dir, shell_dir
    );
    log::log_command(&cmake_cmd);

    let output = Command::new("cmake")
        .args([
            "-B",
            build_dir.to_str().unwrap(),
            "-S",
            shell_dir.to_str().unwrap(),
            "-G",
            "Ninja",
            "-DCMAKE_BUILD_TYPE=Release",
            "-DCMAKE_INSTALL_PREFIX=/usr",
            "-DINSTALL_QMLDIR=/usr/lib64/qt6/qml",
            "-DINSTALL_LIBDIR=/usr/lib64/caelestia",
        ])
        .output()?;

    // Always log both stdout and stderr for debugging
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    log::log("=== SHELL CMAKE CONFIGURE STDOUT ===");
    log::log_output(&stdout);
    log::log("=== SHELL CMAKE CONFIGURE STDERR ===");
    log::log_error(&stderr);

    if !output.status.success() {
        ui::error("CMake configure failed:");
        // Print both stdout and stderr - cmake errors often go to stdout
        if !stdout.is_empty() {
            println!("STDOUT:\n{}", stdout);
        }
        if !stderr.is_empty() {
            println!("STDERR:\n{}", stderr);
        }
        bail!("CMake configure failed. Check ~/.cache/caelestia-installer/install.log for details.");
    }

    // Ninja build
    ui::info("Compiling caelestia-shell...");
    let jobs = crate::system::get_ninja_jobs();
    let mut build_args = vec!["--build", build_dir.to_str().unwrap()];
    let jobs_str;
    if jobs > 0 {
        build_args.push("-j");
        jobs_str = jobs.to_string();
        build_args.push(&jobs_str);
    }

    let output = Command::new("cmake")
        .args(&build_args)
        .output()?;

    // Always log both stdout and stderr for debugging
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    log::log("=== SHELL BUILD STDOUT ===");
    log::log_output(&stdout);
    log::log("=== SHELL BUILD STDERR ===");
    log::log_error(&stderr);

    if !output.status.success() {
        ui::error("Shell build failed:");
        // Print both - ninja/cmake errors can be in either stream
        if !stdout.is_empty() {
            let start = stdout.len().saturating_sub(2000);
            println!("STDOUT (last 2000 chars):\n{}", &stdout[start..]);
        }
        if !stderr.is_empty() {
            println!("STDERR:\n{}", stderr);
        }
        crate::system::check_oom_event();
        bail!("Shell build failed. Check ~/.cache/caelestia-installer/install.log for details.");
    }

    ui::success("Built caelestia-shell");

    // Install (requires sudo)
    ui::info("Installing caelestia-shell...");
    let install_cmd = format!("sudo cmake --install {:?}", build_dir);
    log::log_command(&install_cmd);

    let output = Command::new("sudo")
        .args(["cmake", "--install", build_dir.to_str().unwrap()])
        .output()?;

    if output.status.success() {
        ui::success("Installed caelestia-shell");
        log::log("Shell installation complete");

        // Verification
        ui::info("Verifying installation...");
        // Verification (Check lib64 first, then lib)
        if std::path::Path::new("/usr/lib64/qt6/qml/Caelestia").exists() {
            let _ = Command::new("ls")
                .args(["-R", "/usr/lib64/qt6/qml/Caelestia"])
                .status();
        } else if std::path::Path::new("/usr/lib/qt6/qml/Caelestia").exists() {
            let _ = Command::new("ls")
                .args(["-R", "/usr/lib/qt6/qml/Caelestia"])
                .status();
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        ui::warning("Shell installation failed");
    }

    Ok(())
}

pub fn symlink_configs(dry_run: bool) -> Result<()> {
    let local_share = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("~/.local/share"));
    let dotfiles_dir = local_share.join("caelestia");

    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));

    let links = [
        ("hypr", "hypr"),
        ("foot", "foot"),
        ("fish", "fish"),
        ("fastfetch", "fastfetch"),
        ("btop", "btop"),
        ("uwsm", "uwsm"),
    ];

    for (src, dst) in links {
        let source = dotfiles_dir.join(src);
        let destination = config_dir.join(dst);

        create_symlink(&source, &destination, dry_run)?;
    }

    // Starship config
    let starship_src = dotfiles_dir.join("starship.toml");
    let starship_dst = config_dir.join("starship.toml");
    create_symlink(&starship_src, &starship_dst, dry_run)?;

    Ok(())
}

fn create_symlink(source: &PathBuf, destination: &PathBuf, dry_run: bool) -> Result<()> {
    ui::info(&format!("Linking {:?} -> {:?}", destination, source));

    if dry_run {
        ui::success(&format!("Would link {:?} (dry-run)", destination));
        return Ok(());
    }

    // Remove existing symlink or directory
    if destination.exists() || destination.is_symlink() {
        if destination.is_dir() && !destination.is_symlink() {
            // Backup existing directory
            let backup = destination.with_extension("bak");
            ui::warning(&format!("Backing up {:?} to {:?}", destination, backup));
            fs::rename(destination, &backup)?;
        } else {
            fs::remove_file(destination).ok();
            fs::remove_dir_all(destination).ok();
        }
    }

    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    if source.exists() {
        symlink(source, destination)?;
        ui::success(&format!("Linked {:?}", destination));
        log::log(&format!("Created symlink {:?} -> {:?}", destination, source));
    } else {
        ui::warning(&format!("Source {:?} does not exist, skipping", source));
    }

    Ok(())
}

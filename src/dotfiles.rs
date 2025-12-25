use anyhow::{bail, Result};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use std::process::Command;

use crate::{log, ui};

const DOTFILES_REPO: &str = "https://github.com/caelestia-dots/caelestia.git";
const SHELL_REPO: &str = "https://github.com/caelestia-dots/shell.git";

pub fn clone_repos(dry_run: bool) -> Result<()> {
    let local_share = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("~/.local/share"));
    let dotfiles_dir = local_share.join("caelestia");

    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
    let shell_dir = config_dir.join("quickshell/caelestia");

    clone_repo(DOTFILES_REPO, &dotfiles_dir, dry_run)?;
    clone_repo(SHELL_REPO, &shell_dir, dry_run)?;

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
    fs::create_dir_all(&build_dir)?;

    // CMake configure
    let cmake_cmd = format!(
        "cmake -B {:?} -S {:?} -G Ninja",
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
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        ui::warning("CMake configure failed (this may be expected if shell has no build system)");
        return Ok(());
    }

    // Ninja build
    let ninja_cmd = format!("ninja -C {:?}", build_dir);
    log::log_command(&ninja_cmd);

    let output = Command::new("ninja")
        .args(["-C", build_dir.to_str().unwrap()])
        .output()?;

    if output.status.success() {
        ui::success("Built caelestia-shell");
        log::log("Shell build complete");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        ui::warning("Shell build failed (continuing anyway)");
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

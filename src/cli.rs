use anyhow::{bail, Result};
use std::path::PathBuf;
use std::process::Command;

use crate::{log, ui};

const CLI_REPO: &str = "https://github.com/caelestia-dots/cli.git";

pub fn install_cli(dry_run: bool) -> Result<()> {
    ui::info("Installing caelestia-cli...");

    if dry_run {
        ui::success("Would install caelestia-cli (dry-run)");
        return Ok(());
    }

    // Check if already installed
    if which::which("caelestia").is_ok() {
        ui::success("caelestia-cli already installed");
        return Ok(());
    }

    let cli_dir = PathBuf::from("/tmp/caelestia-cli");

    // Clone repo
    if cli_dir.exists() {
        std::fs::remove_dir_all(&cli_dir).ok();
    }

    let cmd = format!("git clone {} {:?}", CLI_REPO, cli_dir);
    log::log_command(&cmd);

    let output = Command::new("git")
        .args(["clone", CLI_REPO, cli_dir.to_str().unwrap()])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        bail!("Failed to clone caelestia-cli");
    }

    ui::success("Cloned caelestia-cli");

    // Build wheel
    ui::info("Building caelestia-cli...");
    let cmd = "python3 -m build --wheel";
    log::log_command(cmd);

    let output = Command::new("python3")
        .args(["-m", "build", "--wheel"])
        .current_dir(&cli_dir)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        ui::warning("Failed to build caelestia-cli wheel, trying pip install from source");

        // Fallback: pip install from source
        let output = Command::new("pip3")
            .args(["install", "--break-system-packages", "."])
            .current_dir(&cli_dir)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::log_error(&stderr);
            bail!("Failed to install caelestia-cli");
        }
    } else {
        // Install wheel
        ui::info("Installing caelestia-cli...");

        let output = Command::new("sh")
            .args(["-c", "pip3 install --break-system-packages dist/*.whl"])
            .current_dir(&cli_dir)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::log_error(&stderr);
            bail!("Failed to install caelestia-cli wheel");
        }
    }

    ui::success("Installed caelestia-cli");
    log::log("caelestia-cli installation complete");

    // Copy fish completions
    install_fish_completions(&cli_dir)?;

    Ok(())
}

fn install_fish_completions(cli_dir: &PathBuf) -> Result<()> {
    let completions_src = cli_dir.join("completions/caelestia.fish");
    let completions_dst = PathBuf::from("/usr/share/fish/vendor_completions.d/caelestia.fish");

    if completions_src.exists() {
        ui::info("Installing fish completions...");

        let output = Command::new("sudo")
            .args([
                "cp",
                completions_src.to_str().unwrap(),
                completions_dst.to_str().unwrap(),
            ])
            .output()?;

        if output.status.success() {
            ui::success("Installed fish completions");
        } else {
            ui::warning("Could not install fish completions");
        }
    }

    Ok(())
}

pub fn init_scheme(dry_run: bool) -> Result<()> {
    ui::info("Initializing color scheme...");

    if dry_run {
        ui::success("Would initialize color scheme (dry-run)");
        return Ok(());
    }

    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    let dotfiles_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("~/.local/share"))
        .join("caelestia");

    let scheme_src = dotfiles_dir.join("hypr/scheme/default.conf");
    let scheme_dir = home.join(".config/hypr/scheme");
    let scheme_dst = scheme_dir.join("current.conf");

    // Create scheme directory
    std::fs::create_dir_all(&scheme_dir)?;

    // Copy default scheme if source exists
    if scheme_src.exists() {
        std::fs::copy(&scheme_src, &scheme_dst)?;
        ui::success("Initialized default color scheme");
        log::log("Color scheme initialized");
    } else {
        ui::warning("Default scheme file not found, skipping");
    }

    // Create caelestia config directory
    let caelestia_conf = home.join(".config/caelestia");
    std::fs::create_dir_all(&caelestia_conf)?;

    // Create empty user config files if they don't exist
    let hypr_vars = caelestia_conf.join("hypr-vars.conf");
    let hypr_user = caelestia_conf.join("hypr-user.conf");

    if !hypr_vars.exists() {
        std::fs::write(&hypr_vars, "# User Hyprland variables\n")?;
    }
    if !hypr_user.exists() {
        std::fs::write(&hypr_user, "# User Hyprland config\n")?;
    }

    ui::success("Created caelestia config directory");

    Ok(())
}

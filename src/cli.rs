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

    // Install hatch-vcs (required by pyproject.toml)
    ui::info("Installing build dependencies...");
    let cmd = "pip3 install --break-system-packages hatch-vcs";
    log::log_command(cmd);

    let output = Command::new("pip3")
        .args(["install", "--break-system-packages", "hatch-vcs"])
        .output()?;

    if !output.status.success() {
        ui::warning("Could not install hatch-vcs, continuing anyway");
    }

    // Install directly with pip (simpler than building wheel)
    ui::info("Installing caelestia-cli...");
    let cmd = "pip3 install --break-system-packages /tmp/caelestia-cli";
    log::log_command(cmd);

    let output = Command::new("pip3")
        .args(["install", "--break-system-packages", "/tmp/caelestia-cli"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        bail!("Failed to install caelestia-cli");
    }

    // Create wrapper script in /usr/local/bin (pip doesn't always add to PATH)
    ui::info("Creating caelestia wrapper script...");
    let wrapper = "#!/bin/bash\nexec python3 -m caelestia \"$@\"\n";

    let output = Command::new("sudo")
        .args(["tee", "/usr/local/bin/caelestia"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .spawn();

    if let Ok(mut child) = output {
        use std::io::Write;
        if let Some(ref mut stdin) = child.stdin {
            let _ = stdin.write_all(wrapper.as_bytes());
        }
        let _ = child.wait();
    }

    let _ = Command::new("sudo")
        .args(["chmod", "+x", "/usr/local/bin/caelestia"])
        .output();

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

    // The scheme directory should already exist via symlink to dotfiles
    // ~/.config/hypr -> ~/.local/share/caelestia/hypr
    let scheme_src = home.join(".config/hypr/scheme/default.conf");
    let scheme_dst = home.join(".config/hypr/scheme/current.conf");

    // Copy default scheme to current if source exists
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

    // Customize Starship prompt symbols (Override upstream dotfiles)
    // ~/.config/starship.toml is a symlink to ~/.local/share/caelestia/starship.toml
    // We should modify the target file.
    let starship_config = home.join(".local/share/caelestia/starship.toml");
    if starship_config.exists() {
        let content = std::fs::read_to_string(&starship_config)?;
        let new_content = content
            .replace("symbol = \"⋈┈\"", "symbol = \"➜ \"")
            .replace("success_symbol = \"[◎](bold italic bright-yellow)\"", "success_symbol = \"[✔](bold italic bright-green)\"")
            .replace("error_symbol = \"[○](italic purple)\"", "error_symbol = \"[✘](italic bold red)\"");
        
        std::fs::write(starship_config, new_content)?;
        ui::success("Customized Starship prompt symbols");
    }

    Ok(())
}

use anyhow::{bail, Result};
use std::process::Command;

use crate::{log, ui};

const GREETD_CONFIG: &str = r#"[terminal]
vt = 1

[default_session]
command = "tuigreet --time --remember --cmd Hyprland"
user = "greeter"
"#;

pub fn setup_all(dry_run: bool) -> Result<()> {
    write_config(dry_run)?;
    configure_services(dry_run)?;
    Ok(())
}

fn write_config(dry_run: bool) -> Result<()> {
    let config_path = "/etc/greetd/config.toml";

    ui::info("Writing greetd configuration...");

    if dry_run {
        ui::success("Would write greetd config (dry-run)");
        return Ok(());
    }

    // Write via sudo tee
    let cmd = format!("sudo tee {}", config_path);
    log::log_command(&cmd);

    // Ensure directory exists
    let _ = Command::new("sudo")
        .args(["mkdir", "-p", "/etc/greetd"])
        .output();

    let output = Command::new("sudo")
        .args(["tee", config_path])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .spawn();

    match output {
        Ok(mut child) => {
            use std::io::Write;
            if let Some(ref mut stdin) = child.stdin {
                stdin.write_all(GREETD_CONFIG.as_bytes())?;
            }
            let status = child.wait()?;
            if status.success() {
                ui::success("Wrote greetd config");
                log::log("Greetd config written");
            } else {
                log::log_error("Failed to write greetd config");
                bail!("Failed to write greetd configuration");
            }
        }
        Err(e) => {
            log::log_error(&format!("Failed to spawn tee: {}", e));
            bail!("Failed to write greetd configuration");
        }
    }

    Ok(())
}

fn configure_services(dry_run: bool) -> Result<()> {
    ui::info("Configuring greetd services...");

    if dry_run {
        ui::success("Would configure services (dry-run)");
        return Ok(());
    }

    // Disable getty on tty1
    run_systemctl("disable", "getty@tty1")?;

    // Enable greetd
    run_systemctl("enable", "greetd")?;

    // Set graphical target
    let cmd = "sudo systemctl set-default graphical.target";
    log::log_command(cmd);

    let output = Command::new("sudo")
        .args(["systemctl", "set-default", "graphical.target"])
        .output()?;

    if !output.status.success() {
        ui::warning("Could not set default target (may need to run manually)");
    }

    ui::success("Greetd services configured");
    log::log("Greetd service configuration complete");

    Ok(())
}

fn run_systemctl(action: &str, service: &str) -> Result<()> {
    let cmd = format!("sudo systemctl {} {}", action, service);
    log::log_command(&cmd);

    let output = Command::new("sudo")
        .args(["systemctl", action, service])
        .output()?;

    if !output.status.success() {
        ui::warning(&format!("systemctl {} {} may have failed", action, service));
    }

    Ok(())
}

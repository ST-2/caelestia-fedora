use anyhow::{bail, Result};
use std::process::Command;

use crate::{log, ui};

const GREETD_CONFIG: &str = r#"[terminal]
vt = 1

[default_session]
command = "tuigreet --time --remember --remember-session --sessions /usr/share/wayland-sessions"
user = "greeter"
"#;

pub fn setup_all(dry_run: bool) -> Result<()> {
    create_greeter_user(dry_run)?;
    create_cache_dir(dry_run)?;
    write_config(dry_run)?;
    configure_services(dry_run)?;
    Ok(())
}

fn create_greeter_user(dry_run: bool) -> Result<()> {
    ui::info("Creating greeter user...");

    if dry_run {
        ui::success("Would create greeter user (dry-run)");
        return Ok(());
    }

    // Check if user already exists
    let check = Command::new("id").arg("greeter").output();
    if let Ok(output) = check {
        if output.status.success() {
            ui::success("Greeter user already exists");
            return Ok(());
        }
    }

    let cmd = "sudo useradd -r -s /usr/sbin/nologin greeter";
    log::log_command(cmd);

    let output = Command::new("sudo")
        .args(["useradd", "-r", "-s", "/usr/sbin/nologin", "greeter"])
        .output()?;

    if output.status.success() {
        ui::success("Created greeter user");
        log::log("Greeter user created");
    } else {
        ui::warning("Could not create greeter user (may already exist)");
    }

    Ok(())
}

fn create_cache_dir(dry_run: bool) -> Result<()> {
    ui::info("Creating tuigreet cache directory...");

    if dry_run {
        ui::success("Would create cache directory (dry-run)");
        return Ok(());
    }

    // Create directory
    let _ = Command::new("sudo")
        .args(["mkdir", "-p", "/var/cache/tuigreet"])
        .output();

    // Set ownership
    let _ = Command::new("sudo")
        .args(["chown", "greeter:greeter", "/var/cache/tuigreet"])
        .output();

    // Set permissions
    let _ = Command::new("sudo")
        .args(["chmod", "0755", "/var/cache/tuigreet"])
        .output();

    ui::success("Created tuigreet cache directory");
    log::log("Cache directory created at /var/cache/tuigreet");

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

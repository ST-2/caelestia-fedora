use anyhow::{bail, Result};
use std::fs;
use std::process::Command;

use crate::{log, ui};

pub fn run_all(dry_run: bool) -> Result<()> {
    check_fedora()?;
    check_network(dry_run)?;
    check_sudo(dry_run)?;
    Ok(())
}

fn check_fedora() -> Result<()> {
    ui::info("Checking if running on Fedora...");

    let os_release = fs::read_to_string("/etc/os-release").unwrap_or_default();

    if !os_release.contains("ID=fedora") && !os_release.contains("ID=\"fedora\"") {
        log::log_error("Not running on Fedora");
        bail!("This installer only supports Fedora. Detected OS does not appear to be Fedora.");
    }

    ui::success("Running on Fedora");
    log::log("Fedora detected");
    Ok(())
}

fn check_network(dry_run: bool) -> Result<()> {
    ui::info("Checking network connectivity...");

    if dry_run {
        ui::success("Network check (dry-run: skipped)");
        return Ok(());
    }

    let cmd = "ping -c 1 -W 5 fedoraproject.org";
    log::log_command(cmd);

    let output = Command::new("ping")
        .args(["-c", "1", "-W", "5", "fedoraproject.org"])
        .output();

    match output {
        Ok(o) if o.status.success() => {
            ui::success("Network connectivity OK");
            log::log("Network check passed");
            Ok(())
        }
        _ => {
            log::log_error("Network check failed");
            bail!("No network connectivity. Please check your internet connection.");
        }
    }
}

fn check_sudo(dry_run: bool) -> Result<()> {
    ui::info("Checking sudo access...");

    if dry_run {
        ui::success("Sudo check (dry-run: skipped)");
        return Ok(());
    }

    let cmd = "sudo -v";
    log::log_command(cmd);

    let output = Command::new("sudo").arg("-v").status();

    match output {
        Ok(s) if s.success() => {
            ui::success("Sudo access granted");
            log::log("Sudo access verified");
            Ok(())
        }
        _ => {
            log::log_error("Sudo access denied");
            bail!("Could not get sudo access. Please run as a user with sudo privileges.");
        }
    }
}

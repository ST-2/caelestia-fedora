use anyhow::{bail, Result};
use std::process::Command;

use crate::{log, ui};

const COPR_REPOS: &[&str] = &[
    "solopasha/hyprland",
];

pub fn add_all(dry_run: bool) -> Result<()> {
    for repo in COPR_REPOS {
        add_copr(repo, dry_run)?;
    }
    Ok(())
}

fn add_copr(repo: &str, dry_run: bool) -> Result<()> {
    ui::info(&format!("Adding COPR repo: {}", repo));

    let cmd = format!("sudo dnf copr enable -y {}", repo);
    log::log_command(&cmd);

    if dry_run {
        ui::success(&format!("Would add COPR: {} (dry-run)", repo));
        return Ok(());
    }

    let output = Command::new("sudo")
        .args(["dnf", "copr", "enable", "-y", repo])
        .output()?;

    if output.status.success() {
        ui::success(&format!("Added COPR: {}", repo));
        log::log(&format!("COPR {} enabled", repo));
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::log_error(&stderr);
        bail!("Failed to enable COPR repo: {}", repo);
    }
}

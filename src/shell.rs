use anyhow::Result;
use std::process::Command;

use crate::{log, ui};

pub fn setup_all(dry_run: bool) -> Result<()> {
    set_default_shell(dry_run)?;
    Ok(())
}

fn set_default_shell(dry_run: bool) -> Result<()> {
    ui::info("Setting fish as default shell...");

    if dry_run {
        ui::success("Would set fish as default shell (dry-run)");
        return Ok(());
    }

    let cmd = "chsh -s /usr/bin/fish";
    log::log_command(cmd);

    let output = Command::new("chsh").args(["-s", "/usr/bin/fish"]).status();

    match output {
        Ok(s) if s.success() => {
            ui::success("Set fish as default shell");
            log::log("Default shell changed to fish");
            Ok(())
        }
        Ok(_) => {
            ui::warning("Could not set default shell (may need to run manually: chsh -s /usr/bin/fish)");
            Ok(())
        }
        Err(e) => {
            log::log_error(&format!("chsh failed: {}", e));
            ui::warning("Could not set default shell");
            Ok(())
        }
    }
}

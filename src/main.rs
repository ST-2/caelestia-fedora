use anyhow::Result;
use clap::Parser;

use caelestia_installer::{checks, cli, dotfiles, greetd, keybinds, log, packages, repos, shell, ui};

#[derive(Parser)]
#[command(name = "caelestia-installer")]
#[command(about = "Installer for Caelestia Hyprland dotfiles on Fedora")]
#[command(version)]
struct Cli {
    /// Show what would happen without making changes
    #[arg(long)]
    dry_run: bool,

    /// Skip all confirmation prompts
    #[arg(long)]
    noconfirm: bool,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        ui::error(&format!("Installation failed: {}", e));
        ui::info("Check the log for details:");
        log::show_recent_logs(20);
    ui::print_diagnostics();
    ui::print_troubleshooting();
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    ui::print_banner();

    // Initialize logging
    let log_path = log::init()?;
    ui::info(&format!("Logging to {:?}", log_path));
    log::log("Installation started");

    if cli.dry_run {
        ui::warning("DRY RUN MODE - No changes will be made");
    }

    // Confirmation
    if !cli.noconfirm && !cli.dry_run {
        if !ui::prompt("This will install Caelestia Hyprland dotfiles. Continue?") {
            ui::info("Installation cancelled");
            return Ok(());
        }
    }

    let mut progress = ui::Progress::new(13);

    // Step 1: Pre-flight checks
    progress.step("Running pre-flight checks...");
    checks::run_all(cli.dry_run)?;

    // Step 2: Add COPR repos
    progress.step("Adding COPR repositories...");
    repos::add_all(cli.dry_run)?;

    // Step 3: Install packages
    progress.step("Installing packages...");
    packages::install_all(cli.dry_run)?;
    packages::install_starship(cli.dry_run)?;
    packages::install_rust(cli.dry_run)?;
    // Step 3b: Install Hyprland Qt utils
    progress.step("Installing Hyprland Qt utils...");
    packages::install_hyprland_qt_support(cli.dry_run)?;
    packages::install_hyprland_qtutils(cli.dry_run)?;

    // Step 4: Build Quickshell from source
    progress.step("Building Quickshell...");
    packages::install_quickshell(cli.dry_run)?;

    // Step 4b: Installing Cava (Wait, let's just make it sequential)
    progress.step("Installing Cava...");
    packages::install_cava(cli.dry_run)?;

    // Step 5: Install Fonts
    progress.step("Installing Fonts...");
    packages::install_fonts(cli.dry_run)?;
    dotfiles::clone_repos(cli.dry_run)?;

    // Step 6: Install caelestia-cli
    progress.step("Installing caelestia-cli...");
    cli::install_cli(cli.dry_run)?;

    // Step 7: Symlink configs (before scheme init so paths exist)
    progress.step("Symlinking configurations...");
    dotfiles::symlink_configs(cli.dry_run)?;

    // Step 8: Initialize color scheme (after symlinks so ~/.config/hypr exists)
    progress.step("Initializing color scheme...");
    cli::init_scheme(cli.dry_run)?;

    // Step 9: Build shell widgets
    progress.step("Building caelestia-shell...");
    dotfiles::build_shell(cli.dry_run)?;

    // Step 10: Set up shell (fish)
    progress.step("Setting up Fish shell...");
    shell::setup_all(cli.dry_run)?;

    // Step 11: Set up keybinds
    progress.step("Setting up Hyprland keybinds...");
    keybinds::setup_keybinds(cli.dry_run)?;

    // Step 12: Set up greetd (optional, may need confirmation)
    if cli.noconfirm || ui::prompt("Set up greetd/tuigreet as display manager?") {
        greetd::setup_all(cli.dry_run)?;
    }

    log::log("Installation completed successfully");
    ui::print_completion();

    // Offer to reboot
    if !cli.dry_run && !cli.noconfirm {
        if ui::prompt("Reboot now to apply changes?") {
            ui::info("Rebooting...");
            std::process::Command::new("sudo")
                .args(["reboot"])
                .status()
                .ok();
        } else {
            ui::info("Please reboot to apply all changes");
        }
    }

    Ok(())
}

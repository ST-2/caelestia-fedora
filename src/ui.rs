use colored::Colorize;

const BANNER: &str = r#"
   ______           __          __  _
  / ____/___ ____  / /__  _____/ /_(_)___ _
 / /   / __ `/ _ \/ / _ \/ ___/ __/ / __ `/
/ /___/ /_/ /  __/ /  __(__  ) /_/ / /_/ /
\____/\__,_/\___/_/\___/____/\__/_/\__,_/

"#;

pub struct Progress {
    current: usize,
    total: usize,
}

impl Progress {
    pub fn new(total: usize) -> Self {
        Self { current: 0, total }
    }

    pub fn step(&mut self, message: &str) {
        self.current += 1;
        println!(
            "{} {}",
            format!("[{}/{}]", self.current, self.total).cyan().bold(),
            message
        );
    }
}

pub fn print_banner() {
    println!("{}", BANNER.magenta().bold());
    println!(
        "{}",
        "  Hyprland Dotfiles Installer for Fedora"
            .white()
            .bold()
    );
    println!();
}

pub fn success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

pub fn error(message: &str) {
    println!("{} {}", "✗".red().bold(), message);
}

pub fn warning(message: &str) {
    println!("{} {}", "!".yellow().bold(), message);
}

pub fn info(message: &str) {
    println!("{} {}", "→".blue().bold(), message);
}

pub fn prompt(message: &str) -> bool {
    use std::io::{self, Write};

    print!("{} {} [Y/n] ", "?".magenta().bold(), message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_lowercase();
    input.is_empty() || input == "y" || input == "yes"
}

pub fn print_keybinds_summary() {
    println!();
    println!("{}", "Keybinds Summary:".cyan().bold());
    println!("  {} - Open terminal (foot)", "Super + Return".white().bold());
    println!("  {} - Application launcher", "Super + D".white().bold());
    println!("  {} - Close window", "Super + Q".white().bold());
    println!("  {} - Switch workspaces", "Super + 1-9".white().bold());
    println!("  {} - Move window to workspace", "Super + Shift + 1-9".white().bold());
    println!();
}

pub fn print_completion() {
    println!();
    println!(
        "{}",
        "═══════════════════════════════════════════════════════════"
            .green()
    );
    println!(
        "{}",
        "  Installation complete! ".green().bold()
    );
    println!(
        "{}",
        "═══════════════════════════════════════════════════════════"
            .green()
    );
    print_keybinds_summary();
}

use colored::Colorize;
use which;

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

pub fn print_diagnostics() {
    println!();
    println!("{}", "Diagnostic Information:".cyan().bold());
    println!();
    
    // Check fonts
    println!("Fonts:");
    if let Some(home) = dirs::home_dir() {
        let font_dir = home.join(".local/share/fonts");
        if font_dir.exists() {
            println!("  Font directory exists: {}", font_dir.display());
            if let Ok(entries) = std::fs::read_dir(&font_dir) {
                let mut count = 0;
                for entry in entries.flatten() {
                    if entry.file_name().to_str().unwrap_or("").contains(".ttf") {
                        count += 1;
                    }
                }
                println!("  TTF files found: {}", count);
            }
        } else {
            println!("  Font directory missing: {}", font_dir.display());
        }
    }
    
    // Check quickshell
    println!();
    println!("Quickshell:");
    if which::which("quickshell").is_ok() {
        println!("  ✓ quickshell command found");
    } else {
        println!("  ✗ quickshell command not found");
    }
    
    // Check caelestia-shell
    println!();
    println!("Caelestia Shell:");
    let paths = ["/usr/lib64/qt6/qml/Caelestia", "/usr/lib/qt6/qml/Caelestia"];
    for path in &paths {
        if std::path::Path::new(path).exists() {
            println!("  ✓ Caelestia components found at {}", path);
            break;
        }
    }
    
    println!();
}

pub fn print_troubleshooting() {
    println!();
    println!("{}", "Troubleshooting Tips:".yellow().bold());
    println!("  1. If fonts are missing, run: fc-cache -fv");
    println!("  2. To test Quickshell: quickshell -c caelestia launcher");
    println!("  3. Check logs: ~/.cache/caelestia-installer/install.log");
    println!("  4. Rebuild caelestia-shell if needed:");
    println!("     cd ~/.config/quickshell/caelestia");
    println!("     rm -rf build && cmake -B build -S . && cmake --build build");
    println!("     sudo cmake --install build");
    println!("  5. If Quickshell build fails due to Qt package conflicts:");
    println!("     sudo dnf install --allowerasing qt6-qtbase-devel qt6-qtdeclarative-devel");
    println!();
}

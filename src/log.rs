use anyhow::Result;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::ui;

static LOG_FILE: Mutex<Option<PathBuf>> = Mutex::new(None);

pub fn init() -> Result<PathBuf> {
    let cache_dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("caelestia-installer");

    fs::create_dir_all(&cache_dir)?;

    let log_path = cache_dir.join("install.log");

    // Clear previous log
    fs::write(&log_path, "")?;

    *LOG_FILE.lock().unwrap() = Some(log_path.clone());

    Ok(log_path)
}

pub fn log(message: &str) {
    if let Some(ref path) = *LOG_FILE.lock().unwrap() {
        if let Ok(mut file) = OpenOptions::new().append(true).open(path) {
            let timestamp = chrono_lite_timestamp();
            let _ = writeln!(file, "[{}] {}", timestamp, message);
        }
    }
}

pub fn log_command(command: &str) {
    log(&format!("CMD: {}", command));
}

pub fn log_output(output: &str) {
    for line in output.lines() {
        log(&format!("OUT: {}", line));
    }
}

pub fn log_error(error: &str) {
    log(&format!("ERR: {}", error));
}

fn chrono_lite_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", duration.as_secs())
}

pub fn show_recent_logs(lines: usize) {
    if let Some(ref path) = *LOG_FILE.lock().unwrap() {
        if let Ok(content) = fs::read_to_string(path) {
            let log_lines: Vec<&str> = content.lines().collect();
            let start = log_lines.len().saturating_sub(lines);

            ui::info(&format!("Recent log entries (from {:?}):", path));
            for line in &log_lines[start..] {
                println!("  {}", line);
            }
        }
    }
}

pub fn get_log_path() -> Option<PathBuf> {
    LOG_FILE.lock().unwrap().clone()
}

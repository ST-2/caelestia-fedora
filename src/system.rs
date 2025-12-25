use std::fs;
use std::process::Command;
use crate::{log, ui};

pub fn get_ninja_jobs() -> usize {
    if let Ok(mem_info) = fs::read_to_string("/proc/meminfo") {
        let total_kb = mem_info
            .lines()
            .find(|line| line.starts_with("MemTotal:"))
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|num| num.parse::<usize>().ok())
            .unwrap_or(8000000); // Default to 8GB

        let total_gb = total_kb / 1024 / 1024;
        if total_gb < 2 {
            ui::warning(&format!("Low memory detected ({}GB), limiting build to 1 job", total_gb));
            return 1;
        } else if total_gb < 4 {
            ui::warning(&format!("Moderate memory detected ({}GB), limiting build to 2 jobs", total_gb));
            return 2;
        }
    }
    0 // Default (all cores)
}

pub fn check_oom_event() {
    if let Ok(output) = Command::new("dmesg").output() {
        let text = String::from_utf8_lossy(&output.stdout);
        if text.contains("out of memory") || text.contains("OOM-killer") || text.contains("Killed process") {
            ui::error("DETECTED: Build was likely killed by OOM (Out Of Memory) killer!");
            ui::info("Try increasing VM RAM to at least 4GB.");
            log::log("OOM event detected in dmesg");
        }
    }
}

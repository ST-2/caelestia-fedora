use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use crate::{log, ui};

const KEYBINDS_CONTENT: &str = r#"# Caelestia User Keybinds
# Edit this file to customize your keybindings
# This file is sourced by the main hyprland.conf

$mainMod = SUPER

# Applications
bind = $mainMod, Return, exec, foot
bind = $mainMod, Space, exec, quickshell -c caelestia launcher
bind = $mainMod, E, exec, foot -e yazi
bind = $mainMod, B, exec, firefox

# Window management
bind = $mainMod, W, killactive
bind = $mainMod, F, fullscreen
bind = $mainMod, T, togglefloating
bind = $mainMod, P, pseudo
bind = $mainMod, J, togglesplit

# Focus
bind = $mainMod, left, movefocus, l
bind = $mainMod, right, movefocus, r
bind = $mainMod, up, movefocus, u
bind = $mainMod, down, movefocus, d

bind = $mainMod, H, movefocus, l
# bind = $mainMod, L, movefocus, r # Removed to allow Super+L for locking
bind = $mainMod, K, movefocus, u
bind = $mainMod, J, movefocus, d

# Move windows
bind = $mainMod SHIFT, left, movewindow, l
bind = $mainMod SHIFT, right, movewindow, r
bind = $mainMod SHIFT, up, movewindow, u
bind = $mainMod SHIFT, down, movewindow, d

bind = $mainMod SHIFT, H, movewindow, l
bind = $mainMod SHIFT, L, movewindow, r
bind = $mainMod SHIFT, K, movewindow, u
bind = $mainMod SHIFT, J, movewindow, d

# Resize windows
bind = $mainMod CTRL, left, resizeactive, -20 0
bind = $mainMod CTRL, right, resizeactive, 20 0
bind = $mainMod CTRL, up, resizeactive, 0 -20
bind = $mainMod CTRL, down, resizeactive, 0 20

# Workspaces
bind = $mainMod, 1, workspace, 1
bind = $mainMod, 2, workspace, 2
bind = $mainMod, 3, workspace, 3
bind = $mainMod, 4, workspace, 4
bind = $mainMod, 5, workspace, 5
bind = $mainMod, 6, workspace, 6
bind = $mainMod, 7, workspace, 7
bind = $mainMod, 8, workspace, 8
bind = $mainMod, 9, workspace, 9
bind = $mainMod, 0, workspace, 10

# Move to workspace
bind = $mainMod SHIFT, 1, movetoworkspace, 1
bind = $mainMod SHIFT, 2, movetoworkspace, 2
bind = $mainMod SHIFT, 3, movetoworkspace, 3
bind = $mainMod SHIFT, 4, movetoworkspace, 4
bind = $mainMod SHIFT, 5, movetoworkspace, 5
bind = $mainMod SHIFT, 6, movetoworkspace, 6
bind = $mainMod SHIFT, 7, movetoworkspace, 7
bind = $mainMod SHIFT, 8, movetoworkspace, 8
bind = $mainMod SHIFT, 9, movetoworkspace, 9
bind = $mainMod SHIFT, 0, movetoworkspace, 10

# Scroll through workspaces
bind = $mainMod, mouse_down, workspace, e+1
bind = $mainMod, mouse_up, workspace, e-1

# Mouse bindings
bindm = $mainMod, mouse:272, movewindow
bindm = $mainMod, mouse:273, resizewindow

# Media keys
bind = , XF86AudioRaiseVolume, exec, pamixer -i 5
bind = , XF86AudioLowerVolume, exec, pamixer -d 5
bind = , XF86AudioMute, exec, pamixer -t
bind = , XF86AudioMicMute, exec, pamixer --default-source -t
bind = , XF86MonBrightnessUp, exec, brightnessctl set +5%
bind = , XF86MonBrightnessDown, exec, brightnessctl set 5%-
bind = , XF86AudioPlay, exec, playerctl play-pause
bind = , XF86AudioNext, exec, playerctl next
bind = , XF86AudioPrev, exec, playerctl previous

# Screenshot
bind = , Print, exec, grim -g "$(slurp)" - | swappy -f -
bind = SHIFT, Print, exec, grim - | swappy -f -

# Lock screen
bind = $mainMod, L, exec, hyprlock

# Exit Hyprland
bind = $mainMod SHIFT, E, exit

# Gestures (v0.51+)
gesture = 3, horizontal, workspace
"#;

pub fn setup_keybinds(dry_run: bool) -> Result<()> {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
    let hypr_dir = config_dir.join("hypr");
    let keybinds_path = hypr_dir.join("keybinds.conf");

    ui::info("Setting up user keybinds...");

    if dry_run {
        ui::success("Would create keybinds.conf (dry-run)");
        return Ok(());
    }

    fs::create_dir_all(&hypr_dir)?;

    // Don't overwrite existing keybinds
    if keybinds_path.exists() {
        ui::warning("keybinds.conf already exists, skipping");
        return Ok(());
    }

    fs::write(&keybinds_path, KEYBINDS_CONTENT)?;
    ui::success("Created keybinds.conf");
    log::log("Created user keybinds file");

    // Add source line to hyprland.conf if it exists and doesn't have it
    add_source_line(&hypr_dir)?;

    Ok(())
}

fn add_source_line(hypr_dir: &PathBuf) -> Result<()> {
    let hyprland_conf = hypr_dir.join("hyprland.conf");

    if !hyprland_conf.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&hyprland_conf)?;
    let source_line = "source = ~/.config/hypr/keybinds.conf";

    if content.contains(source_line) {
        return Ok(());
    }

    ui::info("Adding keybinds source to hyprland.conf...");

    let new_content = format!("{}\n\n# User keybinds\n{}\n", content, source_line);
    fs::write(&hyprland_conf, new_content)?;

    ui::success("Added keybinds source to hyprland.conf");
    log::log("Added source line to hyprland.conf");

    Ok(())
}

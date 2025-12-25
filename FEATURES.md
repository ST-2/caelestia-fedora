# Caelestia Installer Spec

A simple installer for Caelestia Hyprland dotfiles on Fedora.

## What it should do

- Install all required packages for Caelestia (Hyprland, foot, fish, etc.)
- Clone the caelestia dotfiles repo to `~/.local/share/caelestia`
- Clone and build caelestia-shell (Quickshell widgets)
- Symlink configs from the repo to `~/.config/`
- Set up tuigreet as the login manager
- Create a user keybinds file at `~/.config/hypr/keybinds.conf`

## UI

- Show a nice banner at the start
- Show step progress like `[1/6] Installing packages`
- Use colors and checkmarks for feedback
- Keep it simple - no fancy TUI library needed, just ANSI colors

## Steps

1. **Pre-flight checks**
   - Make sure we're on Fedora
   - Check network connectivity
   - Get sudo access

2. **Add COPR repos**
   - `solopasha/hyprland` for Hyprland packages
   - `errornointernet/quickshell` for Quickshell

3. **Install packages**
   - Hyprland and portals
   - foot terminal
   - fish shell
   - greetd + tuigreet
   - Quickshell + Qt6 dev packages
   - Build tools (cmake, ninja, gcc)
   - Theming (adw-gtk3, papirus icons, fonts)

4. **Clone repos**
   - Main dotfiles to `~/.local/share/caelestia`
   - Shell widgets to `~/.config/quickshell/caelestia`

5. **Build caelestia-shell**
   - CMake + Ninja build
   - Handle libcava dependency if needed

6. **Symlink configs**
   - Link hypr, foot, fish, fastfetch, btop from dotfiles repo
   - Link starship.toml

7. **Set up keybinds**
   - Create `~/.config/hypr/keybinds.conf` with sensible defaults
   - Add source line to hyprland.conf

8. **Set up tuigreet**
   - Create `/etc/greetd/config.toml`
   - Disable getty on tty1
   - Enable greetd service
   - Set graphical.target as default

## CLI flags

- `--dry-run` - show what would happen without doing it
- `--noconfirm` - skip all prompts
- `--help` - show usage

## Error handling

- If something fails, show what went wrong
- Log everything to `~/.cache/caelestia-installer/install.log`
- Offer to show recent log lines on error

## After install

- Show a completion message with basic keybinds
- Ask if user wants to reboot

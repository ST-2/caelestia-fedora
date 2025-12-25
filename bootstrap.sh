#!/bin/bash
# Caelestia Installer Bootstrap
# Run with: bash <(curl -sL https://raw.githubusercontent.com/ST-2/caelestia-fedora/main/bootstrap.sh)

set -e

echo "Installing Caelestia Hyprland dotfiles..."

# Install minimal dependencies
sudo dnf install -y git curl gcc

# Install Rust
if ! command -v cargo &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Clone and build
INSTALL_DIR="/tmp/caelestia-fedora"
rm -rf "$INSTALL_DIR"
git clone https://github.com/ST-2/caelestia-fedora.git "$INSTALL_DIR"
cd "$INSTALL_DIR"
cargo build --release

# Run installer
./target/release/caelestia-installer "$@"

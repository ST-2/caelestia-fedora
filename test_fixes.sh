#!/bin/bash
echo "=== Testing Caelestia Fixes ==="
echo

# Run the installer
echo "🚀 Running caelestia installer..."
bash <(curl -sL https://raw.githubusercontent.com/ST-2/caelestia-fedora/main/bootstrap.sh) --dry-run

echo
echo "=== Check if fixes are in place ==="
echo

# Download and check the source
cd /tmp
git clone https://github.com/ST-2/caelestia-fedora.git
cd caelestia-fedora

echo "🔍 Checking font extraction fix..."
if grep -q "args(\[\"-o\", zip_path, \"-d\", font_dir.to_str().unwrap()\])" src/packages.rs; then
    echo "✓ Font extraction fix is present"
else
    echo "✗ Font extraction fix missing"
fi

echo
echo "🔍 Checking font cache error handling..."
if grep -q "Font cache update failed" src/packages.rs; then
    echo "✓ Font cache error handling is present"
else
    echo "✗ Font cache error handling missing"
fi

echo
echo "🔍 Checking build failure handling..."
if grep -q "bail!(\"CMake configure failed" src/dotfiles.rs; then
    echo "✓ Build failure handling is present"
else
    echo "✗ Build failure handling missing"
fi

echo
echo "�� Checking diagnostic output..."
if grep -q "print_diagnostics" src/ui.rs; then
    echo "✓ Diagnostic output is present"
else
    echo "✗ Diagnostic output missing"
fi

echo
echo "=== Done ==="

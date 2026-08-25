#!/bin/bash
set -e

echo "Building Fred Runtime..."
cargo build --release

if [ -f "$HOME/.cargo/bin/fred" ]; then
    echo "Removing legacy cargo binary from ~/.cargo/bin..."
    rm -f "$HOME/.cargo/bin/fred"
fi

echo "Installing binary to /usr/local/bin/fred..."
sudo cp target/release/fred /usr/local/bin/fred
sudo chmod +x /usr/local/bin/fred

hash -r 2>/dev/null || true

echo ""
echo "Successfully updated! Run 'fred -v' to check your version."
#!/bin/bash
set -e

echo "Building Fred Runtime..."
cargo build --release

echo "Installing binary to /usr/local/bin/fred..."
sudo cp target/release/fred /usr/local/bin/fred
sudo chmod +x /usr/local/bin/fred

echo ""
echo "Successfully installed! You can now type 'fred' from anywhere."
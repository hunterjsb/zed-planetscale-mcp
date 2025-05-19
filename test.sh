#!/bin/bash
set -e

echo "Building PlanetScale Context Server extension for Zed..."

# Make sure we have Rust WASM target
rustup target add wasm32-unknown-unknown

# Build the Zed extension
echo "Building the Zed extension..."
cd planetscale
cargo build --release --target wasm32-unknown-unknown

# Create directories if they don't exist
echo "Setting up directories..."
mkdir -p ~/.config/zed/extensions/dev/planetscale

# Copy the extension files
echo "Copying extension files..."
cp extension.toml ~/.config/zed/extensions/dev/planetscale/
cp target/wasm32-unknown-unknown/release/planetscale_extension.wasm ~/.config/zed/extensions/dev/planetscale/

echo "Installation complete!"
echo "Please make sure you have the PlanetScale CLI installed and then add the context server configuration to your Zed settings.json:"
echo ""
echo '{
  "context_servers": {
    "planetscale": {
      "settings": {
        "database_url": "your-planetscale-connection-string"
      }
    }
  }
}'
echo ""
echo "To install PlanetScale CLI, run: "
echo "  brew install planetscale/tap/pscale (on macOS)"
echo "  or download from GitHub for Linux/Windows"
echo ""
echo "Restart Zed for the changes to take effect."
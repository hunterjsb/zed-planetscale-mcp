#!/bin/bash
set -e

echo "Building PlanetScale Context Server extension for Zed..."

# Make sure we have Rust WASM target
rustup target add wasm32-unknown-unknown

# Build the context server
echo "Building the context server..."
cd planetscale-context-server
cargo build --release

# Build the extension
echo "Building the Zed extension..."
cd ../planetscale
cargo build --release --target wasm32-unknown-unknown

# Create directories if they don't exist
echo "Setting up directories..."
mkdir -p ~/.config/zed/extensions/dev/planetscale

# Copy the extension files
echo "Copying extension files..."
cp extension.toml ~/.config/zed/extensions/dev/planetscale/
cp target/wasm32-unknown-unknown/release/*.wasm ~/.config/zed/extensions/dev/planetscale/

# Copy the context server executable
echo "Copying the context server executable..."
cp ../planetscale-context-server/target/release/planetscale-context-server ~/.config/zed/extensions/dev/

echo "Installation complete!"
echo "Please add the context server configuration to your Zed settings.json:"
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
echo "Restart Zed for the changes to take effect."
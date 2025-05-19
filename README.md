# PlanetScale Context Server for Zed

This extension provides a PlanetScale context server for the Zed editor, allowing the Zed AI assistant to run queries against PlanetScale databases.

## Structure

The extension consists of two main components:

1. The Zed extension (`planetscale/`) - This is the extension that integrates with Zed
2. The context server executable (`planetscale-context-server/`) - This is the executable that implements the Model Context Protocol for PlanetScale

## Installation

### Prerequisites

- Rust installed via `rustup`
- Zed editor
- PlanetScale CLI installed and configured

### Building the Extension

Run the provided installation script:

```bash
./test.sh
```

This will:
1. Add the WASM target to Rust
2. Build the context server executable
3. Build the Zed extension
4. Copy the extension and context server to the proper Zed directories

### Manual Installation

If you prefer to install manually:

1. Build the context server executable:
```bash
cd planetscale-context-server
cargo build --release
```

2. Build the Zed extension:
```bash
cd ../planetscale
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

3. Copy the extension files to Zed:
```bash
mkdir -p ~/.config/zed/extensions/dev/planetscale
cp extension.toml ~/.config/zed/extensions/dev/planetscale/
cp target/wasm32-unknown-unknown/release/*.wasm ~/.config/zed/extensions/dev/planetscale/
cp ../planetscale-context-server/target/release/planetscale-context-server ~/.config/zed/extensions/dev/
```

### Configuration

In your Zed settings.json, add:

```json
{
  "context_servers": {
    "planetscale": {
      "settings": {
        "database_url": "your-planetscale-connection-string"
      }
    }
  }
}
```

## Usage

Once installed, you can use the `/ps` slash command in Zed to run PlanetScale database operations.

## Development

This extension is written in Rust and follows the Zed extension model for context servers. It uses the Model Context Protocol (MCP) to communicate with the PlanetScale API.

## License

MIT
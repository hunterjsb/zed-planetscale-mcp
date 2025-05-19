# PlanetScale Context Server for Zed

This extension integrates PlanetScale's built-in Model Context Protocol (MCP) server with Zed editor, allowing the Zed AI assistant to run queries against PlanetScale databases.

## Prerequisites

- [PlanetScale CLI](https://github.com/planetscale/cli) installed and configured
- Zed editor

## Installation

### Installing PlanetScale CLI

Make sure you have the PlanetScale CLI installed and available in your PATH:

```bash
# macOS (with Homebrew)
brew install planetscale/tap/pscale

# Linux (using .deb package)
curl -L https://github.com/planetscale/cli/releases/download/v0.147.0/pscale_0.147.0_linux_amd64.deb -o pscale.deb
sudo dpkg -i pscale.deb

# Linux (using .rpm package)
curl -L https://github.com/planetscale/cli/releases/download/v0.147.0/pscale_0.147.0_linux_amd64.rpm -o pscale.rpm
sudo rpm -i pscale.rpm

# Windows (with Scoop)
scoop bucket add pscale https://github.com/planetscale/scoop-bucket.git
scoop install pscale
```

### Installing the Extension

1. Clone this repository:
```bash
git clone https://github.com/hunterjsb/zed-planetscale-mcp.git
cd zed-planetscale-mcp
```

2. Build and install the extension:
```bash
./test.sh
```

3. Configure Zed to use the extension by adding the following to your settings.json:
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

4. Restart Zed for the changes to take effect.

## Usage

Once installed, you can use PlanetScale-related commands in Zed's AI assistant. The exact commands available will depend on what the PlanetScale MCP server supports, but typically include:

- Listing databases
- Listing branches
- Getting database schemas
- Running queries

## How it Works

This extension leverages the built-in MCP server in the PlanetScale CLI by running `pscale mcp server` and connecting it to Zed's context server system. The MCP server provides functionality for interacting with PlanetScale databases, and Zed's AI assistant can use this to execute commands on your behalf.

## License

MIT
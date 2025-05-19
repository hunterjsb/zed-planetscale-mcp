# Testing the PlanetScale Context Server Extension

This document provides instructions for testing the PlanetScale context server extension for Zed.

## Prerequisites

Ensure you have:
- Rust installed via `rustup`
- Zed editor
- PlanetScale CLI installed and configured

## Building for Testing

1. Build the context server executable:
```bash
cd planetscale-context-server
cargo build --release
```

2. Build the Zed extension:
```bash
cd ../planetscale
cargo build --release
```

## Installing for Testing

1. Create a link in your Zed extensions directory:
```bash
mkdir -p ~/.config/zed/extensions/dev
ln -s /full/path/to/planetscale ~/.config/zed/extensions/dev/planetscale
```

2. Copy the context server executable to the proper location:
```bash
cp target/release/planetscale-context-server ~/.config/zed/extensions/dev/
```

3. Update your Zed settings.json to include the context server configuration:
```json
{
  "context_servers": {
    "planetscale-context-server": {
      "settings": {
        "database_url": "your-planetscale-connection-string"
      }
    }
  }
}
```

## Test Cases

1. **List Databases**
   - Use the slash command `/ps-list-dbs` in Zed
   - Verify that it returns a list of databases in your PlanetScale account

2. **List Branches**
   - Use the slash command `/ps-list-branches example_db` in Zed
   - Verify that it returns a list of branches for the specified database

3. **Get Schema**
   - Use the slash command `/ps-schema example_db main` in Zed
   - Verify that it returns the schema for the specified database and branch

4. **Run Query**
   - Use the slash command `/ps-query example_db main "SELECT * FROM users LIMIT 2"` in Zed
   - Verify that it returns the query results

## Troubleshooting

If you encounter issues:

1. Check Zed logs for error messages
2. Verify that the context server executable is in the correct location
3. Ensure your PlanetScale CLI is correctly configured
4. Check that your database URL is correctly specified in settings.json
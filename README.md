# PlanetScale Context Server for Zed

This extension integrates PlanetScale's built-in Model Context Protocol (MCP) server with Zed editor, allowing the Zed AI assistant to interact with your PlanetScale databases.

## Features

The extension provides these MCP tools:
- `list_orgs` - List available organizations
- `list_databases` - List databases in an organization
- `list_branches` - List branches for a database
- `list_keyspaces` - List keyspaces in a branch
- `list_tables` - List tables in a keyspace
- `get_schema` - Get table schemas
- `run_query` - Execute read-only SQL queries
- `get_insights` - Get database performance insights

## Prerequisites

- PlanetScale CLI (automatically downloaded by extension)
- Authenticated PlanetScale session: `pscale auth login`
- Zed editor

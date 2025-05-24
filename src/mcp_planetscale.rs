use schemars::JsonSchema;
use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, Command, ContextServerConfiguration, ContextServerId, Project, Result, serde_json,
};

struct PlanetScaleModelContextExtension;

#[derive(Debug, Deserialize, JsonSchema)]
struct PlanetScaleContextServerSettings {
    #[serde(default)]
    organization: Option<String>,
    #[serde(default)]
    database: Option<String>,
}

impl zed::Extension for PlanetScaleModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = ContextServerSettings::for_project("planetscale-context-server", project)?;
        let settings: PlanetScaleContextServerSettings = if let Some(settings) = settings.settings {
            serde_json::from_value(settings).map_err(|e| e.to_string())?
        } else {
            PlanetScaleContextServerSettings {
                organization: None,
                database: None,
            }
        };

        let mut env_vars = vec![];

        // Ensure environment variables are available for pscale config
        if let Ok(home) = std::env::var("HOME") {
            env_vars.push(("HOME".to_string(), home));
        }
        if let Ok(path) = std::env::var("PATH") {
            env_vars.push(("PATH".to_string(), path));
        }
        if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
            env_vars.push(("XDG_CONFIG_HOME".to_string(), xdg_config));
        }

        // Add optional organization and database as environment variables
        if let Some(org) = &settings.organization {
            env_vars.push(("PLANETSCALE_ORG".to_string(), org.clone()));
        }
        if let Some(db) = &settings.database {
            env_vars.push(("PLANETSCALE_DATABASE".to_string(), db.clone()));
        }

        Ok(Command {
            command: "pscale".to_string(),
            args: vec!["mcp".to_string(), "server".to_string()],
            env: env_vars,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions = r#"# This context server provides PlanetScale database management capabilities through the Model Context Protocol.
### Prerequisites
1. Install the PlanetScale CLI: https://planetscale.com/cli
2. Authenticate with PlanetScale: `pscale auth login`
"#.to_string();

        let default_settings = r#"{
  "context_servers": {
    "planetscale-context-server": {
      "settings": {
        // optional params:
        // "organization": "your-org-name",
        // "database": "your-database-name"
      }
    }
  }
}"#
        .to_string();

        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(PlanetScaleContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }

    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        _args: Vec<String>,
        _worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput> {
        match command.name.as_str() {
            "hello" => Ok(zed::SlashCommandOutput {
                text: "Hello from PlanetScale MCP! 🚀".to_string(),
                sections: vec![],
            }),
            other => Err(format!("unknown slash command: {other}")),
        }
    }
}

zed::register_extension!(PlanetScaleModelContextExtension);

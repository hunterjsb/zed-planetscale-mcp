use schemars::JsonSchema;
use serde::Deserialize;
use std::fs;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const REPO_NAME: &str = "planetscale/cli";
const BINARY_NAME: &str = "pscale";

#[derive(Debug, Deserialize, JsonSchema)]
struct PlanetScaleContextServerSettings {
    #[serde(default)]
    organization: Option<String>,
    #[serde(default)]
    database: Option<String>,
}

struct PlanetScaleModelContextExtension {
    cached_binary_path: Option<String>,
}

impl PlanetScaleModelContextExtension {
    fn context_server_binary_path(
        &mut self,
        _context_server_id: &ContextServerId,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        let release = zed::latest_github_release(
            REPO_NAME,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "{BINARY_NAME}_{version}_{os}_{arch}.tar.gz",
            version = release.version,
            arch = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X86 => "386",
                zed::Architecture::X8664 => "amd64",
            },
            os = match platform {
                zed::Os::Mac => "darwin",
                zed::Os::Linux => "linux",
                zed::Os::Windows => return Err("Windows is not supported".into()),
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("{BINARY_NAME}-{}", release.version);
        fs::create_dir_all(&version_dir)
            .map_err(|err| format!("failed to create directory '{version_dir}': {err}"))?;
        let binary_path = format!("{version_dir}/{BINARY_NAME}");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::download_file(&asset.download_url, &version_dir, zed::DownloadedFileType::GzipTar)
                .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            // Removes old versions
            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for PlanetScaleModelContextExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
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

        let binary_path = self.context_server_binary_path(context_server_id)?;

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
            command: binary_path,
            args: vec!["mcp".to_string(), "server".to_string()],
            env: env_vars,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions = r#"# PlanetScale MCP Server

Provides database access through the Model Context Protocol.

## Setup
1. Extension automatically downloads the PlanetScale CLI
2. Authenticate: `pscale auth login` (you'll need to do this in terminal)
"#
        .to_string();

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
}

zed::register_extension!(PlanetScaleModelContextExtension);
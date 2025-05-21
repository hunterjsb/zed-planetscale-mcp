use zed_extension_api as zed;

/// A stateless hello-world extension.
struct PlanetscaleContextServerExtension;

impl zed::Extension for PlanetscaleContextServerExtension {
    /// Mandatory constructor.
    fn new() -> Self {
        PlanetscaleContextServerExtension
    }

    /// Handle `/hello`.
    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        _args: Vec<String>,
        _worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput, String> {
        match command.name.as_str() {
            "hello" => Ok(zed::SlashCommandOutput {
                text: "Hello, world!".into(),
                sections: vec![],
            }),
            other => Err(format!("unknown slash command: {other}")),
        }
    }
    
    /// Configure the mini_mcp context server
    fn context_server_command(
        &mut self,
        context_server_id: &zed::ContextServerId,
        _project: &zed::Project,
    ) -> Result<zed::Command, String> {
        match context_server_id.as_ref() {
            // Use the context server ID as defined in extension.toml: "mini-mcp"
            "mini-mcp" => {
                Ok(zed::Command {
                    command: "python3".to_string(),
                    // Launch the mini-mcp server script from the project workspace
                    args: vec!["mini-mcp/server.py".to_string()],
                    env: vec![],
                })
            },
            _ => Err(format!("unknown context server: {context_server_id}")),
        }
    }
}

zed::register_extension!(PlanetscaleContextServerExtension);

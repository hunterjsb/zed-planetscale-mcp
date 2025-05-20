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
}

zed::register_extension!(PlanetscaleContextServerExtension);

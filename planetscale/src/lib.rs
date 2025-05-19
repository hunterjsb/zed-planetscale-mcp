use zed_extension_api as zed;
use std::collections::HashMap;

struct PlanetScaleExtension;

impl zed::Extension for PlanetScaleExtension {
    fn new() -> Self {
        Self
    }
    
    fn context_server_command(
        &mut self,
        context_server_id: &zed::ContextServerId,
        _project: &zed::Project,
    ) -> Result<zed_extension_api::process::Command, String> {
        // Only handle our specific context server
        if context_server_id.as_ref() != "planetscale" {
            return Err(format!("Unknown context server: {}", context_server_id.as_ref()));
        }
        
        // Use the PlanetScale CLI's built-in MCP server
        // The command is: pscale mcp server
        let mut env = HashMap::new();
        
        // Add any necessary environment variables
        // env.insert("PS_VAR".to_string(), "value".to_string());
        
        Ok(zed_extension_api::process::Command {
            command: "pscale".to_string(),
            args: vec!["mcp".to_string(), "server".to_string()],
            env: vec![],
        })
    }
}

// Register the extension
zed::register_extension!(PlanetScaleExtension);
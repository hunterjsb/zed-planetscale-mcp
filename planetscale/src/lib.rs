use zed_extension_api as zed;

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
        
        // Get path to our context server executable
        let executable_path = match std::env::current_exe() {
            Ok(path) => {
                match path.parent() {
                    Some(parent) => parent.join("planetscale-context-server"),
                    None => return Err("Failed to get parent directory".to_string())
                }
            },
            Err(e) => return Err(e.to_string())
        };
        
        Ok(zed_extension_api::process::Command {
            command: executable_path.to_string_lossy().to_string(),
            args: vec![],
            env: vec![],
        })
    }
}

// Register the extension
zed::register_extension!(PlanetScaleExtension);
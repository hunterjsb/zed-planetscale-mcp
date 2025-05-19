use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};
use std::process;

// Main MCP protocol structures
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    id: String,
    #[serde(flatten)]
    content: MessageContent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
enum MessageContent {
    #[serde(rename = "function_call")]
    FunctionCall {
        name: String,
        arguments: serde_json::Value,
    },
    #[serde(rename = "function_response")]
    FunctionResponse {
        result: serde_json::Value,
    },
    #[serde(rename = "error")]
    Error { message: String },
}

// Tool definitions
#[derive(Debug, Serialize, Deserialize)]
struct Tool {
    name: String,
    description: String,
    parameters: ToolParameters,
}

#[derive(Debug, Serialize, Deserialize)]
struct ToolParameters {
    #[serde(rename = "$schema")]
    schema: String,
    r#type: String,
    properties: serde_json::Value,
    required: Vec<String>,
    additional_properties: bool,
}

// Slash Command definitions
#[derive(Debug, Serialize, Deserialize)]
struct SlashCommand {
    name: String,
    description: String,
}

// Service to handle interactions with PlanetScale
struct PlanetScaleService {
    // This would contain connection details, etc.
}

impl PlanetScaleService {
    fn new() -> Self {
        PlanetScaleService {}
    }

    // List databases
    fn list_databases(&self) -> Result<serde_json::Value> {
        // Implementation would interact with planetscale CLI or API
        Ok(serde_json::json!({
            "databases": [
                {
                    "name": "example_db",
                    "organization": "example_org"
                }
            ]
        }))
    }

    // List branches
    fn list_branches(&self, database: &str) -> Result<serde_json::Value> {
        // Implementation would interact with planetscale CLI or API
        Ok(serde_json::json!({
            "branches": [
                {
                    "name": "main",
                    "database": database,
                    "production": true
                },
                {
                    "name": "dev",
                    "database": database,
                    "production": false
                }
            ]
        }))
    }

    // Get schema
    fn get_schema(&self, _database: &str, _branch: &str) -> Result<serde_json::Value> {
        // Implementation would interact with planetscale CLI or API
        Ok(serde_json::json!({
            "tables": [
                {
                    "name": "users",
                    "columns": [
                        {
                            "name": "id",
                            "type": "INT",
                            "primary": true
                        },
                        {
                            "name": "name",
                            "type": "VARCHAR(255)"
                        },
                        {
                            "name": "email",
                            "type": "VARCHAR(255)"
                        }
                    ]
                },
                {
                    "name": "posts",
                    "columns": [
                        {
                            "name": "id",
                            "type": "INT",
                            "primary": true
                        },
                        {
                            "name": "user_id",
                            "type": "INT"
                        },
                        {
                            "name": "title",
                            "type": "VARCHAR(255)"
                        },
                        {
                            "name": "content",
                            "type": "TEXT"
                        }
                    ]
                }
            ]
        }))
    }

    // Run query
    fn run_query(&self, _database: &str, _branch: &str, query: &str) -> Result<serde_json::Value> {
        // Implementation would interact with planetscale CLI or API
        // This is a placeholder
        if query.to_lowercase().contains("select") {
            Ok(serde_json::json!({
                "rows": [
                    {
                        "id": 1,
                        "name": "John Doe",
                        "email": "john@example.com"
                    },
                    {
                        "id": 2,
                        "name": "Jane Smith",
                        "email": "jane@example.com"
                    }
                ]
            }))
        } else {
            Ok(serde_json::json!({
                "affected_rows": 1
            }))
        }
    }
}

// Main function to handle the MCP protocol
fn main() -> Result<()> {
    // Initialize the service
    let service = PlanetScaleService::new();

    // Define slash commands - this format is specifically for the Zed MCP protocol
    let slash_commands = serde_json::json!([
        {
            "name": "ps",
            "description": "PlanetScale database operations",
            "documentation": "Run operations against PlanetScale databases"
        }
    ]);

    // Define available tools
    let tools = vec![
        Tool {
            name: "list_databases".to_string(),
            description: "Lists all databases in the connected PlanetScale account".to_string(),
            parameters: ToolParameters {
                schema: "http://json-schema.org/draft-07/schema#".to_string(),
                r#type: "object".to_string(),
                properties: serde_json::json!({}),
                required: vec![],
                additional_properties: false,
            },
        },
        Tool {
            name: "list_branches".to_string(),
            description: "Lists all branches for a specific database".to_string(),
            parameters: ToolParameters {
                schema: "http://json-schema.org/draft-07/schema#".to_string(),
                r#type: "object".to_string(),
                properties: serde_json::json!({
                    "database": {
                        "type": "string",
                        "description": "The name of the database"
                    }
                }),
                required: vec!["database".to_string()],
                additional_properties: false,
            },
        },
        Tool {
            name: "get_schema".to_string(),
            description: "Gets the schema for a specific database and branch".to_string(),
            parameters: ToolParameters {
                schema: "http://json-schema.org/draft-07/schema#".to_string(),
                r#type: "object".to_string(),
                properties: serde_json::json!({
                    "database": {
                        "type": "string",
                        "description": "The name of the database"
                    },
                    "branch": {
                        "type": "string",
                        "description": "The name of the branch"
                    }
                }),
                required: vec!["database".to_string(), "branch".to_string()],
                additional_properties: false,
            },
        },
        Tool {
            name: "run_query".to_string(),
            description: "Runs a SQL query against a specific database and branch".to_string(),
            parameters: ToolParameters {
                schema: "http://json-schema.org/draft-07/schema#".to_string(),
                r#type: "object".to_string(),
                properties: serde_json::json!({
                    "database": {
                        "type": "string",
                        "description": "The name of the database"
                    },
                    "branch": {
                        "type": "string",
                        "description": "The name of the branch"
                    },
                    "query": {
                        "type": "string",
                        "description": "The SQL query to execute"
                    }
                }),
                required: vec![
                    "database".to_string(),
                    "branch".to_string(),
                    "query".to_string(),
                ],
                additional_properties: false,
            },
        },
    ];

    // Send capabilities message
    let capabilities = serde_json::json!({
        "functions": tools,
        "slash_commands": slash_commands,
        "name": "PlanetScale",
        "description": "PlanetScale database operations for Zed"
    });

    let capabilities_message = Message {
        id: "server".to_string(),
        content: MessageContent::FunctionResponse {
            result: capabilities,
        },
    };

    // Serialize and send capabilities
    let capabilities_json = serde_json::to_string(&capabilities_message)?;
    println!("{}", capabilities_json);

    // Read input line by line
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();

    while stdin_lock.read_line(&mut buffer)? > 0 {
        // Parse incoming message
        let message: Message = match serde_json::from_str(&buffer) {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Error parsing message: {}", e);
                buffer.clear();
                continue;
            }
        };

        // Process the message
        let response = match message.content {
            MessageContent::FunctionCall { name, arguments } => {
                // Handle function calls
                let result = match name.as_str() {
                    "list_databases" => service.list_databases(),
                    "list_branches" => {
                        let database = arguments["database"]
                            .as_str()
                            .ok_or_else(|| anyhow!("database parameter is required"))?;
                        service.list_branches(database)
                    }
                    "get_schema" => {
                        let database = arguments["database"]
                            .as_str()
                            .ok_or_else(|| anyhow!("database parameter is required"))?;
                        let branch = arguments["branch"]
                            .as_str()
                            .ok_or_else(|| anyhow!("branch parameter is required"))?;
                        service.get_schema(database, branch)
                    }
                    "run_query" => {
                        let database = arguments["database"]
                            .as_str()
                            .ok_or_else(|| anyhow!("database parameter is required"))?;
                        let branch = arguments["branch"]
                            .as_str()
                            .ok_or_else(|| anyhow!("branch parameter is required"))?;
                        let query = arguments["query"]
                            .as_str()
                            .ok_or_else(|| anyhow!("query parameter is required"))?;
                        service.run_query(database, branch, query)
                    }
                    _ => Err(anyhow!("Unknown function: {}", name)),
                };

                match result {
                    Ok(value) => Message {
                        id: message.id,
                        content: MessageContent::FunctionResponse { result: value },
                    },
                    Err(e) => Message {
                        id: message.id,
                        content: MessageContent::Error {
                            message: e.to_string(),
                        },
                    },
                }
            }
            _ => Message {
                id: message.id,
                content: MessageContent::Error {
                    message: "Unsupported message type".to_string(),
                },
            },
        };

        // Send the response
        let response_json = serde_json::to_string(&response)?;
        println!("{}", response_json);

        // Clear buffer for next message
        buffer.clear();
    }

    Ok(())
}
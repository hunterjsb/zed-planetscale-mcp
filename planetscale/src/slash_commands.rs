use anyhow::Result;
use zed_extension_api::assistant::{AssistantDocumentation, SlashCommand};

pub fn list_dbs_command() -> Result<SlashCommand> {
    Ok(SlashCommand {
        name: "ps-list-dbs".to_string(),
        documentation: AssistantDocumentation::MarkdownString(
            r#"
## /ps-list-dbs

Lists all databases in your PlanetScale account.

### Usage
```
/ps-list-dbs
```
"#
            .to_string(),
        ),
    })
}

pub fn list_branches_command() -> Result<SlashCommand> {
    Ok(SlashCommand {
        name: "ps-list-branches".to_string(),
        documentation: AssistantDocumentation::MarkdownString(
            r#"
## /ps-list-branches

Lists all branches for a specific database.

### Usage
```
/ps-list-branches <database>
```

### Parameters
- `database`: The name of the database
"#
            .to_string(),
        ),
    })
}

pub fn schema_command() -> Result<SlashCommand> {
    Ok(SlashCommand {
        name: "ps-schema".to_string(),
        documentation: AssistantDocumentation::MarkdownString(
            r#"
## /ps-schema

Gets the schema for a specific database and branch.

### Usage
```
/ps-schema <database> <branch>
```

### Parameters
- `database`: The name of the database
- `branch`: The name of the branch
"#
            .to_string(),
        ),
    })
}

pub fn query_command() -> Result<SlashCommand> {
    Ok(SlashCommand {
        name: "ps-query".to_string(),
        documentation: AssistantDocumentation::MarkdownString(
            r#"
## /ps-query

Runs a SQL query against a specific database and branch.

### Usage
```
/ps-query <database> <branch> <query>
```

### Parameters
- `database`: The name of the database
- `branch`: The name of the branch
- `query`: The SQL query to execute
"#
            .to_string(),
        ),
    })
}
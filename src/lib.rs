use std::env;
use zed_extension_api::{
    self as zed, Command, ContextServerConfiguration, ContextServerId, Project, Result,
    settings::ContextServerSettings,
};

const PACKAGE_NAME: &str = "mcp-remote";
const PACKAGE_VERSION: &str = "0.1.37";
const PACKAGE_PATH: &str = "node_modules/mcp-remote/dist/proxy.js";
const STORYBLOK_MCP_URL: &str = "http://localhost:3016/mcp";

struct StoryblokMcpServer;

impl zed::Extension for StoryblokMcpServer {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        // Install mcp-remote package if not already installed or wrong version
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        // Get the path to the proxy.js file
        let proxy_path = env::current_dir()
            .unwrap()
            .join(PACKAGE_PATH)
            .to_string_lossy()
            .to_string();

        // Get user settings
        let settings = ContextServerSettings::for_project(context_server_id.as_ref(), project)?;

        // Extract settings and build env vars
        let mut env_vars: Vec<(String, String)> = vec![];
        if let Some(settings_value) = settings.settings {
            if let Some(token) = settings_value.get("management_token").and_then(|v| v.as_str()) {
                env_vars.push(("STORYBLOK_MANAGEMENT_TOKEN".to_string(), token.to_string()));
            }
            if let Some(space_id) = settings_value.get("space_id").and_then(|v| v.as_str()) {
                env_vars.push(("STORYBLOK_SPACE_ID".to_string(), space_id.to_string()));
            }
        }

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![proxy_path, STORYBLOK_MCP_URL.to_string()],
            env: env_vars,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let settings_schema = serde_json::json!({
            "type": "object",
            "properties": {
                "management_token": {
                    "type": "string",
                    "description": "Your Storyblok Management API Token"
                },
                "space_id": {
                    "type": "string",
                    "description": "Your Storyblok Space ID (optional)"
                }
            },
            "required": ["management_token"]
        });

        let default_settings = serde_json::json!({
            "management_token": "",
            "space_id": ""
        });

        Ok(Some(ContextServerConfiguration {
            installation_instructions: r#"# Storyblok MCP Server

This extension connects Zed to a locally running Storyblok MCP Server for headless CMS integration.

## Requirements

- Node.js v18 or later
- Storyblok MCP Server running locally on port 3016
- Storyblok account with Management API access

## Setup

1. Start the Storyblok MCP Server:
   cd path/to/storyblok-mcp-server
   pnpm run dev

2. Add your credentials to Zed settings (Settings > Open Settings):
   {
     "context_servers": {
       "storyblok-mcp-server": {
         "settings": {
           "management_token": "your-storyblok-management-token",
           "space_id": "your-space-id"
         }
       }
     }
   }

3. Enable the context server in Zed Agent Panel settings

## Getting Your Management Token

1. Log in to Storyblok (https://app.storyblok.com)
2. Go to Settings -> Access Tokens
3. Create a Management API token with appropriate permissions

## Available Tools

### Story Management
- storyblok_list_stories: List stories with pagination and filtering
- storyblok_get_story: Get a story by ID or slug
- storyblok_create_story: Create a new story
- storyblok_update_story: Update an existing story
- storyblok_delete_story: Delete a story
- storyblok_publish_story: Publish a story
- storyblok_unpublish_story: Unpublish a story

### Component Management
- storyblok_list_components: List all components
- storyblok_get_component: Get component schema
- storyblok_create_component: Create a component
- storyblok_update_component: Update component schema

### Asset Management
- storyblok_list_assets: List assets
- storyblok_get_asset: Get asset details
- storyblok_delete_asset: Delete an asset

### Other Tools
- storyblok_get_space: Get space info
- storyblok_list_datasources: List datasources
- storyblok_list_folders: List folders
- storyblok_list_tags: List tags

## Troubleshooting

If tools do not appear:
1. Check that your management_token is correctly set in Zed settings
2. Ensure Storyblok MCP Server is running on port 3016
3. Restart Zed and re-enable the context server
"#.to_string(),
            default_settings: default_settings.to_string(),
            settings_schema: settings_schema.to_string(),
        }))
    }
}

zed::register_extension!(StoryblokMcpServer);

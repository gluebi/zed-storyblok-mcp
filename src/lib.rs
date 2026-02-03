use std::fs;
use std::env;
use zed_extension_api::{
    self as zed, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "mcp-remote";
const PACKAGE_VERSION: &str = "0.1.37";
const PACKAGE_PATH: &str = "node_modules/mcp-remote/dist/proxy.js";
const STORYBLOK_MCP_URL: &str = "http://localhost:3016/mcp";

struct StoryblokMcpServer;

fn read_env_file() -> Vec<(String, String)> {
    let mut env_vars = Vec::new();
    
    if let Ok(content) = fs::read_to_string(".env") {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();
                let value = value.trim().trim_matches('"').trim_matches(''').to_string();
                env_vars.push((key, value));
            }
        }
    }
    
    env_vars
}

impl zed::Extension for StoryblokMcpServer {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
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

        // Read environment variables from .env file
        let env_vars = read_env_file();

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

2. Create a .env file in your project root (optional, for reference):
   STORYBLOK_MANAGEMENT_TOKEN=your-token
   STORYBLOK_SPACE_ID=your-space-id

3. Enable the context server in Zed Agent Panel settings

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

If connection fails:
1. Ensure Storyblok MCP Server is running on port 3016
2. Check the server logs for errors
3. Restart Zed and re-enable the context server
"#.to_string(),
            default_settings: "{}".to_string(),
            settings_schema: "{}".to_string(),
        }))
    }
}

zed::register_extension!(StoryblokMcpServer);

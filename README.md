# Storyblok MCP Server for Zed

A Zed extension that connects to a locally running [Storyblok MCP Server](https://github.com/myposter-de/mcp) for headless CMS integration.

## Features

- **Story Management:** List, create, update, publish, and delete stories
- **Component Management:** Manage component schemas
- **Asset Management:** Browse and manage media assets
- **Datasources & Tags:** Organize content with datasources and tags

## Prerequisites

- [Node.js](https://nodejs.org/) v18 or later
- Storyblok MCP Server running locally
- A [Storyblok](https://www.storyblok.com/) account with Management API access

## Installation

Install from Zed Extensions: `zed://extension/storyblok-mcp-server`

Or search for "Storyblok MCP Server" in Zed's extension panel.

## Setup

1. **Start the Storyblok MCP Server:**
   ```bash
   cd path/to/storyblok-mcp-server
   pnpm run dev
   ```
   The server runs on port 3016 by default.

2. **Enable the context server** in Zed's Agent Panel settings

3. Use natural language to interact with your Storyblok content

## Available Tools

| Tool | Description |
|------|-------------|
| `storyblok_list_stories` | List stories with pagination and filtering |
| `storyblok_get_story` | Get a story by ID or slug with full content |
| `storyblok_create_story` | Create a new story |
| `storyblok_update_story` | Update an existing story |
| `storyblok_delete_story` | Delete a story |
| `storyblok_publish_story` | Publish a story |
| `storyblok_unpublish_story` | Unpublish a story |
| `storyblok_list_components` | List all components in the space |
| `storyblok_get_component` | Get a component with its schema |
| `storyblok_create_component` | Create a new component |
| `storyblok_update_component` | Update a component schema |
| `storyblok_list_assets` | List assets with pagination |
| `storyblok_get_asset` | Get asset details and URL |
| `storyblok_delete_asset` | Delete an asset |
| `storyblok_get_space` | Get current space info and settings |
| `storyblok_list_datasources` | List all datasources |
| `storyblok_list_folders` | List all folders |
| `storyblok_list_tags` | List all tags with usage counts |

## Example Queries

- "List all published stories"
- "Get the content of the homepage story"
- "Create a new blog post called My First Post"
- "List all components in my space"
- "Publish the about-us story"

## How It Works

This extension uses [mcp-remote](https://www.npmjs.com/package/mcp-remote) to bridge Zed's local MCP protocol with your locally running Storyblok MCP Server at `http://localhost:3016/mcp`.

## Troubleshooting

If tools don't appear or connection fails:
1. Ensure the Storyblok MCP Server is running on port 3016
2. Check `curl http://localhost:3016/health` returns OK
3. Restart Zed and re-enable the context server

## License

MIT

# Telegram Notification MCP Server

An MCP (Model Context Protocol) server that sends notifications to Telegram when Claude Code completes tasks. Built with Rust and deployable on Cloudflare Workers.

## Setup

### Prerequisites

1. **Telegram Bot**: Create a bot via [@BotFather](https://t.me/botfather) and get your bot token
2. **Chat ID**: Get your chat ID by sending a message to your bot and visiting:
   ```
   https://api.telegram.org/bot<YOUR_BOT_TOKEN>/getUpdates
   ```
3. **Cloudflare Account**: Sign up at [cloudflare.com](https://cloudflare.com)
4. **Wrangler CLI**: Install with `npm install -g wrangler`

### Installation

1. Clone this repository
2. Install dependencies:
   ```bash
   cargo build
   ```

### Configuration

1. Set up Cloudflare secrets:
   ```bash
   wrangler secret put TELEGRAM_BOT_TOKEN
   wrangler secret put TELEGRAM_CHAT_ID
   ```

2. Update `wrangler.toml` with your worker name if desired

### Deployment

Deploy to Cloudflare Workers:
```bash
wrangler deploy
```

### Claude Code Configuration

Add to your Claude Code MCP configuration:

```bash
cp mcp.example.json mcp.json
```

```json
{
  "mcpServers": {
    "telegram-notification": {
      "url": "https://your-worker-name.workers.dev/mcp",
      "transport": {
        "type": "http"
      }
    }
  }
}
```

## Usage

Once configured, this MCP server integrates with Claude Code to send Telegram notifications. Here's how it works:

### How Claude Code Uses This Server

1. **MCP Integration**: Claude Code connects to your deployed Cloudflare Worker via the MCP protocol
2. **Available Tool**: A new `notify_telegram` tool becomes available to Claude Code
3. **Notification Trigger**: Claude Code can call this tool at any time during task execution

### Types of Notifications

#### 1. Manual Notifications (Explicit)
Claude Code can explicitly send notifications by calling the `notify_telegram` tool:

**Example scenarios:**
- When a long-running task completes
- When an important milestone is reached
- When user intervention might be needed

**How Claude Code calls it:**
```
// Claude Code will automatically use this tool when appropriate
// Example: After completing a deployment
notify_telegram({ "message": "Deployment completed successfully! The app is now live." })

// Example: After running tests
notify_telegram({ "message": "All tests passed! 52/52 tests successful." })
```

#### 2. Automatic Notifications (Future Feature)
*Note: Automatic notifications for events like "user input required" are planned but not yet implemented in this version.*

### Notification Format

All notifications appear in your Telegram chat with:
- 🤖 **Claude Code Notification** prefix
- The custom message content
- Timestamp of when the notification was sent

### Example Workflow

1. **You ask Claude Code**: "Deploy my app and let me know when it's done"
2. **Claude Code performs the deployment**: Runs build, tests, and deploy commands
3. **Claude Code sends notification**: Calls `notify_telegram` with a completion message
4. **You receive on Telegram**: "🤖 Claude Code Notification: Deployment completed successfully!"

### Common Use Cases

- **Long-running builds**: Get notified when compilation or bundling completes
- **Test suites**: Know immediately when tests finish and whether they passed
- **Deployments**: Receive confirmation when your app is live
- **Data processing**: Get alerts when batch jobs or migrations complete
- **Error handling**: Be notified if something goes wrong during execution

### Tips for Best Results

- Ask Claude Code explicitly to "notify me when done" or "send a Telegram message when complete"
- Claude Code will intelligently decide when notifications are appropriate
- Notifications work best for tasks that take more than a few seconds

## Development

Run locally:
```bash
wrangler dev
```

Run clippy:
```bash
cargo clippy
```
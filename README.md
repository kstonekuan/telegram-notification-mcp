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
4. **Wrangler CLI**: Install with `pnpm install -g wrangler`

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

Once configured, Claude Code can send notifications to your Telegram whenever you need them.

### How It Works

1. This MCP server gives Claude Code a `notify_telegram` tool
2. Claude Code uses this tool to send messages to your Telegram
3. You'll receive notifications with a 🤖 prefix in your configured chat

### When You'll Get Notifications

Claude Code sends notifications when:
- You explicitly ask: "notify me when done" or "let me know on Telegram"
- Errors occur during execution
- Important milestones are reached
- User input or intervention is needed

### Example Scenarios

```bash
# You say: "Deploy to production and notify me when done"
# Result: 🤖 Claude Code Notification
#         Deployment completed successfully! The app is now live.

# You say: "Run all tests and let me know the results"
# Result: 🤖 Claude Code Notification
#         All tests passed! 52/52 tests successful.

# You say: "Process this data and notify me if there are any errors"
# Result: 🤖 Claude Code Notification
#         Error: Failed to process row 451 - invalid date format
```

### CLAUDE.md Examples

To encourage Claude Code to use Telegram notifications effectively, add these to your CLAUDE.md:

```markdown
# Telegram Notifications

- Always send a Telegram notification when:
  - A task is fully complete
  - You need user input to continue
  - An error occurs that requires user attention

- Include relevant details in notifications:
  - For builds/tests: success/failure status and counts
  - For errors: the specific error message and file location

- Use concise, informative messages like:
  - "✅ Build completed successfully (2m 34s)"
  - "❌ Tests failed: 3/52 failing in auth.test.ts"
  - "⚠️ Need permission to modify /etc/hosts"
```

## Development

Run locally:
```bash
wrangler dev
```

Run clippy:
```bash
cargo clippy
```
# Telegram Notification MCP Server

An MCP (Model Context Protocol) server that sends notifications to Telegram when Claude Code completes tasks. Built with Rust and deployable on Cloudflare Workers.

## Features

- 🤖 **MCP Tools**: Provides a `notify_telegram` tool that Claude Code can use
- 📢 **Automatic Notifications**: Handles MCP notification events (errors, progress, task completion)
- 🚀 **Cloudflare Workers**: Runs serverless with global distribution
- 🔐 **Secure**: Uses Cloudflare secrets for credentials
- 🌐 **SSE Transport**: Supports Server-Sent Events for real-time communication

## Architecture

This server implements the MCP specification (version 2024-11-05) with SSE transport:
- **GET /mcp**: SSE endpoint that returns the connection endpoint
- **POST /mcp**: Handles MCP JSON-RPC requests and returns SSE-formatted responses
- Full lifecycle support with `initialize` and `initialized` handling
- Proper JSON-RPC 2.0 error codes

## Setup

### Prerequisites

1. **Telegram Bot**: Create a bot via [@BotFather](https://t.me/botfather) and get your bot token
2. **Chat ID**: Get your chat ID by sending a message to your bot and visiting:
   ```
   https://api.telegram.org/bot<YOUR_BOT_TOKEN>/getUpdates
   ```
3. **Cloudflare Account**: Sign up at [cloudflare.com](https://cloudflare.com)

### Installation

1. Clone this repository
2. Install dependencies:
   ```bash
   pnpm install
   cargo build
   ```

### Configuration

1. For local development, create a `.env` file:
   ```bash
   cp .env.example .env
   ```
   Then edit `.env` with your bot token and chat ID.

2. For production deployment, set up Cloudflare secrets:
   ```bash
   pnpm exec wrangler secret put TELEGRAM_BOT_TOKEN
   pnpm exec wrangler secret put TELEGRAM_CHAT_ID
   ```
   
   **Note**: The TELEGRAM_CHAT_ID determines who receives notifications. Each deployment can only notify one chat/user.

3. Update `wrangler.toml` with your worker name if desired

### Deployment

Deploy to Cloudflare Workers:

**Option 1: Using .env file (Recommended)**
```bash
# This will read your .env file and set secrets automatically
./deploy.sh
```

**Option 2: Manual deployment**
```bash
# First set secrets manually
pnpm exec wrangler secret put TELEGRAM_BOT_TOKEN
pnpm exec wrangler secret put TELEGRAM_CHAT_ID

# Then deploy
pnpm exec wrangler deploy
```

### Claude Code Configuration

Add the MCP server to Claude Code using the CLI:

```bash
# For production deployment
claude mcp add telegram-notify https://your-worker-name.workers.dev/mcp -t sse

# For local development
claude mcp add telegram-notify http://localhost:8787/mcp -t sse
```

**Note**: This server uses SSE (Server-Sent Events) transport for MCP communication.

You can verify the configuration with:
```bash
claude mcp list
```

## Usage

Once configured, Claude Code can send notifications to your Telegram whenever you need them.

### Available Tool

- **notify_telegram**: Send a custom notification message to Telegram
  - Parameter: `message` (string) - The notification message to send

### Automatic Notifications

The server also handles these MCP notification events automatically:
- `$/userInputRequired` - ⚠️ When user input is needed
- `$/progress` - ⏳ Task progress updates
- `$/error` - ❌ When errors occur
- `$/taskComplete` - ✅ When tasks complete

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
# Create .env file if you haven't already
cp .env.example .env
# Edit .env with your credentials

# Start local development server
pnpm run dev
```

Wrangler will automatically load variables from your `.env` file during local development.

Run clippy:
```bash
cargo clippy
```

Test the server:
```bash
# Test SSE connection
curl http://localhost:8787/mcp

# Test health endpoint
curl http://localhost:8787/health
```

## Debugging

### Testing the SSE Connection

1. **Using the test script:**
   ```bash
   ./test-sse.sh
   ```

2. **Using the Node.js debug client:**
   ```bash
   pnpm add -D eventsource  # Install dependency
   node debug-client.js http://localhost:8787/mcp
   ```

3. **Using the browser test client:**
   Open `test-client.html` in a web browser and connect to your worker URL.

### Common Issues

1. **Connection closes immediately**: Check that your worker is running and accessible at the specified URL.

2. **No endpoint event received**: Ensure the SSE headers are being sent correctly and the stream is properly formatted.

3. **Telegram notifications not sent**: Verify your `TELEGRAM_BOT_TOKEN` and `TELEGRAM_CHAT_ID` are correctly set in the worker environment.

## Technical Details

- **Language**: Rust
- **Runtime**: Cloudflare Workers (WASM)
- **Protocol**: MCP (Model Context Protocol) v2024-11-05
- **Transport**: SSE (Server-Sent Events)
- **Dependencies**: 
  - `worker` - Cloudflare Workers Rust SDK
  - `serde` - JSON serialization
  - `serde_json` - JSON parsing
  - `futures` - Async streams

## License

MIT
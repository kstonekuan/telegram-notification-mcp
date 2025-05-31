#!/bin/bash

# Test SSE connection
echo "Testing SSE connection..."
echo "========================"
echo ""

# Test 1: Check if SSE endpoint responds
echo "Test 1: Checking SSE endpoint..."
curl -i -N -H "Accept: text/event-stream" http://localhost:8787/mcp &
SSE_PID=$!

# Wait a bit for the connection to establish
sleep 2

# Test 2: Send initialize request
echo ""
echo "Test 2: Sending initialize request..."
curl -X POST http://localhost:8787/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "initialize",
    "params": {
      "protocolVersion": "2024-11-05",
      "capabilities": {},
      "clientInfo": {
        "name": "test-client",
        "version": "1.0.0"
      }
    },
    "id": 1
  }' | jq .

# Test 3: Send initialized notification
echo ""
echo "Test 3: Sending initialized notification..."
curl -X POST http://localhost:8787/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "initialized",
    "params": {}
  }'

# Test 4: List tools
echo ""
echo "Test 4: Listing tools..."
curl -X POST http://localhost:8787/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/list",
    "id": 2
  }' | jq .

# Test 5: Call tool
echo ""
echo "Test 5: Calling notify_telegram tool..."
curl -X POST http://localhost:8787/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
      "name": "notify_telegram",
      "arguments": {
        "message": "Test notification from SSE test script"
      }
    },
    "id": 3
  }' | jq .

# Kill the SSE connection
echo ""
echo "Stopping SSE connection..."
kill $SSE_PID 2>/dev/null

echo ""
echo "Tests completed!"
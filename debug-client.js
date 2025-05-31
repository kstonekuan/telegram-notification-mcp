#!/usr/bin/env node

const EventSource = require('eventsource');

console.log('MCP SSE Debug Client');
console.log('===================\n');

const sseUrl = process.argv[2] || 'http://localhost:8787/mcp';
console.log(`Connecting to SSE endpoint: ${sseUrl}`);

const eventSource = new EventSource(sseUrl);

let endpointUrl = null;

eventSource.onopen = () => {
  console.log('✅ SSE connection opened');
};

eventSource.onerror = (error) => {
  console.error('❌ SSE error:', error);
  if (eventSource.readyState === EventSource.CLOSED) {
    console.log('Connection closed');
  }
};

eventSource.onmessage = (event) => {
  console.log('📨 Message (no event type):', event.data);
};

eventSource.addEventListener('endpoint', (event) => {
  endpointUrl = event.data;
  console.log('🔗 Endpoint event received:', endpointUrl);
  
  // After receiving endpoint, send initialize request
  setTimeout(() => sendInitialize(), 1000);
});

eventSource.addEventListener('ping', (event) => {
  console.log('🏓 Ping received');
});

// Handle all events
const originalAddEventListener = eventSource.addEventListener;
eventSource.addEventListener = function(type, listener, options) {
  console.log(`📎 Registered listener for event: ${type}`);
  return originalAddEventListener.call(this, type, listener, options);
};

async function sendRequest(method, params, id) {
  if (!endpointUrl) {
    console.error('No endpoint URL available yet');
    return;
  }
  
  const body = {
    jsonrpc: '2.0',
    method,
    params,
    ...(id !== undefined && { id })
  };
  
  console.log(`\n📤 Sending ${method} request to ${endpointUrl}`);
  console.log('Request body:', JSON.stringify(body, null, 2));
  
  try {
    const response = await fetch(endpointUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(body)
    });
    
    const text = await response.text();
    console.log('📥 Response status:', response.status);
    console.log('Response headers:', Object.fromEntries(response.headers.entries()));
    
    if (text) {
      try {
        const json = JSON.parse(text);
        console.log('Response body:', JSON.stringify(json, null, 2));
      } catch {
        console.log('Response body (text):', text);
      }
    } else {
      console.log('Response body: (empty)');
    }
  } catch (error) {
    console.error('Request error:', error);
  }
}

async function sendInitialize() {
  await sendRequest('initialize', {
    protocolVersion: '2024-11-05',
    capabilities: {},
    clientInfo: {
      name: 'debug-client',
      version: '1.0.0'
    }
  }, 1);
  
  // Send initialized notification
  setTimeout(() => {
    sendRequest('initialized', {});
  }, 500);
  
  // List tools
  setTimeout(() => {
    sendRequest('tools/list', {}, 2);
  }, 1000);
}

// Keep process alive
process.on('SIGINT', () => {
  console.log('\nClosing connection...');
  eventSource.close();
  process.exit(0);
});
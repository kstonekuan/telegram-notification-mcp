#!/bin/bash

# Load .env file if it exists
if [ -f .env ]; then
    echo "Loading secrets from .env file..."
    
    # Read .env file and set secrets
    while IFS='=' read -r key value; do
        # Skip comments and empty lines
        if [[ ! "$key" =~ ^#.*$ ]] && [[ -n "$key" ]]; then
            # Remove quotes from value if present
            value="${value%\"}"
            value="${value#\"}"
            
            echo "Setting secret: $key"
            echo "$value" | pnpm exec wrangler secret put "$key" --name telegram-notification-mcp
        fi
    done < .env
    
    echo "Deploying worker..."
    pnpm exec wrangler deploy
else
    echo "No .env file found. Using existing secrets."
    pnpm exec wrangler deploy
fi
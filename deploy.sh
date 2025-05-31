#!/bin/bash

# Check which file to use for secrets
if [ -f .dev.vars ]; then
    SECRET_FILE=".dev.vars"
elif [ -f .env ]; then
    SECRET_FILE=".env"
    echo "Note: .env is deprecated. Please use .dev.vars instead."
else
    echo "No .dev.vars or .env file found. Deploying with existing secrets..."
    npx wrangler deploy
    exit 0
fi

echo "Loading secrets from $SECRET_FILE..."

# Read file and set secrets
while IFS='=' read -r key value; do
    # Skip comments and empty lines
    if [[ ! "$key" =~ ^#.*$ ]] && [[ -n "$key" ]]; then
        # Remove quotes from value if present
        value="${value%\"}"
        value="${value#\"}"
        
        echo "Setting secret: $key"
        echo "$value" | npx wrangler secret put "$key" --name telegram-notification-mcp
    fi
done < "$SECRET_FILE"

echo "Deploying worker..."
npx wrangler deploy
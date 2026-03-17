#!/bin/bash
docker run --rm \
  -v "$(pwd):/app" \
  -v gochat-pnpm:/root/.local/share/pnpm \
  -w /app \
  -e PNPM_HOME=/root/.local/share/pnpm \
  -e PATH="/root/.local/share/pnpm:$PATH" \
  node:20-bookworm-slim \
  sh -c "corepack enable && corepack prepare pnpm@latest --activate && pnpm $@"

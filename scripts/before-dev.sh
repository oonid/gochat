#!/bin/bash
#
# before-dev.sh - Start vite dev server for Tauri beforeDevCommand
#
# CONTEXT:
#   By default, tauri.conf.json uses "beforeDevCommand": "pnpm dev" which runs
#   pnpm directly on the host machine. However, this project uses Docker to run
#   pnpm and vite (see scripts/dpnpm.sh) to avoid requiring Node.js installation
#   on the host.
#
#   Since the host may not have pnpm installed, we replace:
#     "beforeDevCommand": "pnpm dev"
#   with:
#     "beforeDevCommand": "./scripts/before-dev.sh"
#
#   This script starts the vite dev server in a Docker container if not already
#   running, ensuring the frontend is ready before Tauri launches.
#
# USAGE:
#   Called automatically by `cargo tauri dev` via beforeDevCommand.
#   Can also be run manually: ./scripts/before-dev.sh
#
# SEE ALSO:
#   - scripts/vite.sh - Manual vite container management (start/stop/status/logs)
#   - scripts/dev.sh  - Full development workflow
#   - scripts/dpnpm.sh - Run arbitrary pnpm commands in Docker
#
set -e

CONTAINER_NAME="gochat-vite"
PORT=1420

if docker ps --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
    echo "Vite dev server already running"
    exit 0
fi

docker rm -f ${CONTAINER_NAME} 2>/dev/null || true

docker run -d --name ${CONTAINER_NAME} \
    -v "$(pwd):/app" \
    -w /app \
    -p ${PORT}:${PORT} \
    node:20-bookworm-slim \
    sh -c "corepack enable && corepack prepare pnpm@latest --activate && pnpm dev --host 0.0.0.0"

echo "Waiting for vite to start..."
for i in {1..30}; do
    if curl -s "http://localhost:${PORT}" > /dev/null 2>&1; then
        echo "Vite dev server ready at http://localhost:${PORT}"
        exit 0
    fi
    sleep 1
done

echo "Timeout waiting for vite"
docker logs ${CONTAINER_NAME}
exit 1

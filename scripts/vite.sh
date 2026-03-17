#!/bin/bash
set -e

CONTAINER_NAME="gochat-vite"
IMAGE="node:20-bookworm-slim"
PORT=1420

start() {
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
}

stop() {
    docker rm -f ${CONTAINER_NAME} 2>/dev/null || true
    echo "Vite dev server stopped"
}

status() {
    if docker ps --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
        echo "Vite dev server is running"
        docker logs --tail 5 ${CONTAINER_NAME} 2>&1
    else
        echo "Vite dev server is not running"
    fi
}

logs() {
    docker logs -f ${CONTAINER_NAME}
}

case "${1:-}" in
    start)   start ;;
    stop)    stop ;;
    status)  status ;;
    logs)    logs ;;
    *)       echo "Usage: $0 {start|stop|status|logs}"; exit 1 ;;
esac

#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

echo "Starting vite dev server..."
./scripts/vite.sh start

echo "Starting Tauri dev..."
cargo tauri dev "$@"

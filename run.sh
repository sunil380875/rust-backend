#!/usr/bin/env bash
set -euo pipefail

# Create .env from .env.example if it doesn't exist
if [ ! -f .env ]; then
    echo "[run.sh] .env not found — copying from .env.example"
    cp .env.example .env
    echo "[run.sh] Please edit .env and set your MONGO_URI and JWT_SECRET, then re-run."
    exit 1
fi

# Ensure uploads directory exists
mkdir -p uploads

echo "[run.sh] Starting rust-backend..."
cargo run

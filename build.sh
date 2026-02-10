#!/usr/bin/env bash

# Production deployment script for Censor
# Run this to prepare the application for release

set -e

echo "🔨 Building Censor for production..."

# Check requirements
command -v cargo >/dev/null 2>&1 || { echo "Rust/Cargo not found"; exit 1; }
command -v npm >/dev/null 2>&1 || { echo "Node.js/npm not found"; exit 1; }

# Build frontend
echo "📦 Building Vue.js frontend..."
npm run build

# Build Tauri application
echo "🚀 Building Tauri application..."
npm run tauri:build

echo "✅ Build complete!"
echo ""
echo "Output locations:"
echo "  Windows: src-tauri/target/release/censor.exe"
echo "  macOS:   src-tauri/target/release/censor.app"
echo "  Linux:   src-tauri/target/release/censor"
echo ""
echo "Next steps:"
echo "  1. Test the built application"
echo "  2. Sign application if necessary"
echo "  3. Create installer/DMG"
echo "  4. Push to release repository"

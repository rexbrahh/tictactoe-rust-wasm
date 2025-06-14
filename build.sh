#!/bin/bash

echo "🚀 Building Tic-Tac-Toe with Rust/WASM..."

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "📦 Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WASM module
echo "🔧 Building WASM module..."
wasm-pack build --target web --out-dir pkg

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo ""
    echo "🌐 Starting web server..."
    echo "Open http://localhost:8000 in your browser"
    echo ""
    python3 -m http.server 8000
else
    echo "❌ Build failed. Please check the error messages above."
    exit 1
fi 
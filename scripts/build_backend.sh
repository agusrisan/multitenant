#!/bin/bash
set -e

echo "Building backend..."
cargo build --release
echo "Backend build complete!"
echo "Binary location: target/release/multitenant"

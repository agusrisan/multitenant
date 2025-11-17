#!/bin/bash
set -e

echo "Building frontend..."
cd resources
npm run build
echo "Frontend build complete!"

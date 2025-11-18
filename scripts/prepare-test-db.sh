#!/usr/bin/env bash
# Script to prepare test database for running tests

set -e

echo "📦 Loading test environment..."
set -a
source .env.test
set +a

echo "🔧 Running migrations on test database: $DATABASE_URL"
sqlx migrate run --database-url "$DATABASE_URL"

echo "✅ Test database is ready!"

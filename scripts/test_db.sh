#!/bin/bash
set -e

echo "Setting up test database..."

# Create test database
echo "Creating test database..."
psql -U postgres -c "DROP DATABASE IF EXISTS multitenant_test;" || true
psql -U postgres -c "CREATE DATABASE multitenant_test;"

# Run migrations
echo "Running migrations..."
DATABASE_URL=postgres://postgres:password@localhost:5432/multitenant_test \
  sqlx migrate run

echo "Test database setup complete!"

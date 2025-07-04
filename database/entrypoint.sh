#!/bin/sh
set -e

# Wait for database to be ready
until sqlx database create 2>/dev/null; do
  echo "Waiting for database to be ready..."
  sleep 2
done

# Run migrations
sqlx migrate run

# Execute any passed command
exec "$@"

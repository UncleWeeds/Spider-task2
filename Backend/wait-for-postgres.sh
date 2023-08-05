#!/bin/bash

# Wait for PostgreSQL to be ready
until PGPASSWORD="abhiramsai" psql -h "postgres" -U "abhi" -p "5432" -d "rust_server" -c '\q' >/dev/null 2>&1; do
  echo "Postgres is unavailable - sleeping"
  sleep 5
done

echo "Postgres is up - executing migrations"
diesel migration run

echo "Running backend application"
cargo run --release



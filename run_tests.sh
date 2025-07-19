#!/bin/bash

# Ensure the script is executable
chmod +x run_tests.sh

# Set environment variables
export RUST_BACKTRACE=1
export RUST_LOG=debug
export DATABASE_URL=mysql://slack_attendance_user:strong_password@localhost:3306/slack_attendance_db

# Run unit tests
echo "🧪 Running Unit Tests..."
cargo test --lib

# Run integration tests
echo "🔍 Running Integration Tests..."
cargo test --test integration_tests

# Run clippy for linting
echo "🕵️ Running Clippy Linter..."
cargo clippy

# Check formatting
echo "✨ Checking Code Formatting..."
cargo fmt -- --check

echo "🎉 All Tests Completed!"

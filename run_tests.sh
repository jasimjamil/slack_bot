#!/bin/bash

# Ensure the script is executable
chmod +x run_tests.sh

# Start the server in the background
cargo run &
SERVER_PID=$!

# Wait for the server to start
sleep 5

# Run Postman tests
newman run SlackAttendanceBackend_Postman_Collection.json

# Run curl tests
./test_backend.sh

# Run Rust integration tests
cargo test

# Kill the server
kill $SERVER_PID

echo "All tests completed!"

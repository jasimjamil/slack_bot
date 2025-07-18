#!/bin/bash

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Base URL
BASE_URL="http://localhost:8080"

# Function to test endpoints
test_endpoint() {
    local method=$1
    local endpoint=$2
    local data=$3
    local expected_status=${4:-200}

    echo -e "\n🔍 Testing ${method} ${endpoint}"
    
    if [ "$method" == "POST" ]; then
        response=$(curl -s -X POST "${BASE_URL}${endpoint}" \
            -H "Content-Type: application/json" \
            -d "$data")
        status_code=$(curl -s -o /dev/null -w "%{http_code}" -X POST "${BASE_URL}${endpoint}" \
            -H "Content-Type: application/json" \
            -d "$data")
    elif [ "$method" == "GET" ]; then
        response=$(curl -s "${BASE_URL}${endpoint}")
        status_code=$(curl -s -o /dev/null -w "%{http_code}" "${BASE_URL}${endpoint}")
    fi

    if [ "$status_code" -eq "$expected_status" ]; then
        echo -e "${GREEN}✅ Success: Status Code ${status_code}${NC}"
        echo "Response: $response"
    else
        echo -e "${RED}❌ Failed: Expected ${expected_status}, Got ${status_code}${NC}"
        exit 1
    fi
}

# Start backend in background
cargo run &
BACKEND_PID=$!

# Wait for backend to start
sleep 5

# Test User Registration
test_endpoint "POST" "/register" '{
    "name": "Test User",
    "email": "testuser@example.com",
    "password": "StrongPassword123!",
    "role": "Employee"
}'

# Test User Login
LOGIN_RESPONSE=$(test_endpoint "POST" "/login" '{
    "email": "testuser@example.com",
    "password": "StrongPassword123!"
}')

# Extract JWT Token
JWT_TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.token')

# Test Slack User Verification
test_endpoint "POST" "/slack/verify-user" '{
    "slack_user_id": "U12345",
    "email": "testuser@example.com"
}'

# Test Slack Event
test_endpoint "POST" "/slack/events" '{
    "event": {
        "type": "message",
        "user": "U12345",
        "channel": "#daily-standup",
        "text": "Yesterday: Worked on backend integration\nToday: Implementing Slack bot\nBlockers: None"
    }
}'

# Test Attendance Retrieval (requires JWT)
test_endpoint "GET" "/attendance"

# Test Monthly Summary (requires JWT)
test_endpoint "GET" "/dashboard/summary"

# Kill backend
kill $BACKEND_PID

echo -e "\n🎉 ${GREEN}Backend Testing Complete Successfully!${NC}"

#!/bin/bash

# Base URL
BASE_URL="http://localhost:8080"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to test an endpoint
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
    fi
}

# Test User Registration
test_endpoint "POST" "/register" '{
    "name": "Test User",
    "email": "testuser@example.com",
    "password": "StrongPassword123!",
    "role": "Employee"
}'

# Test User Login
test_endpoint "POST" "/login" '{
    "email": "testuser@example.com",
    "password": "StrongPassword123!"
}'

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

# Note: For GET endpoints, you would need to replace YOUR_JWT_TOKEN with an actual token

echo -e "\n🏁 Backend Testing Complete"

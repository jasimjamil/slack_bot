#!/bin/bash

# Ensure script is executable
chmod +x setup.sh

# Color codes
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to check command success
check_success() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ $1 Successful${NC}"
    else
        echo -e "${YELLOW}❌ $1 Failed${NC}"
        exit 1
    fi
}

# Update system packages
echo "🔄 Updating system packages..."
sudo apt update
check_success "Package Update"

# Install dependencies
echo "📦 Installing system dependencies..."
sudo apt install -y \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    libmysqlclient-dev \
    mysql-server
check_success "Dependency Installation"

# Install Rust
if ! command -v rustc &> /dev/null; then
    echo "🦀 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    check_success "Rust Installation"
fi

# Install SQLx CLI
echo "🛠️ Installing SQLx CLI..."
cargo install sqlx-cli --no-default-features --features mysql
check_success "SQLx CLI Installation"

# MySQL Setup
echo "💾 Setting up MySQL Database..."
./mysql_setup.sh
check_success "MySQL Setup"

# Set environment variables
export DATABASE_URL=mysql://slack_attendance_user:strong_password@localhost:3306/slack_attendance_db

# Run database migrations
echo "🗄️ Running Database Migrations..."
sqlx migrate run
check_success "Database Migration"

# Build the project
echo "🏗️ Building Slack Attendance Backend..."
cargo build --release
check_success "Project Build"

# Run tests
echo "🧪 Running Project Tests..."
cargo test
check_success "Project Testing"

echo -e "${GREEN}�� Slack Attendance Backend Setup Complete!${NC}"
echo "To start the server, run: cargo run"

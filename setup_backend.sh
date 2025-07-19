#!/bin/bash

# Update package list
sudo apt update

# Install MySQL
sudo apt install -y mysql-server

# Secure MySQL installation
sudo mysql_secure_installation

# Create database and user
sudo mysql <<MYSQL_SCRIPT
CREATE DATABASE slack_attendance_db;
CREATE USER 'slack_attendance_user'@'localhost' IDENTIFIED BY 'strong_password';
GRANT ALL PRIVILEGES ON slack_attendance_db.* TO 'slack_attendance_user'@'localhost';
FLUSH PRIVILEGES;
MYSQL_SCRIPT

# Install Rust and Cargo if not installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Load Rust environment
source $HOME/.cargo/env

# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features mysql

# Run database migrations
sqlx migrate run

echo "Backend setup complete!"

#!/bin/bash

# Update package list
sudo apt update

# Install MySQL
sudo apt install -y mysql-server

# Start MySQL service
sudo systemctl start mysql
sudo systemctl enable mysql

# Secure MySQL installation
sudo mysql_secure_installation

# Create database and user
sudo mysql <<MYSQL_SCRIPT
# Drop existing user if it exists
DROP USER IF EXISTS 'slack_attendance_user'@'localhost';

# Create new user with proper authentication
CREATE USER 'slack_attendance_user'@'localhost' IDENTIFIED WITH mysql_native_password BY 'strong_password';

# Create database
CREATE DATABASE IF NOT EXISTS slack_attendance_db;

# Grant all privileges
GRANT ALL PRIVILEGES ON slack_attendance_db.* TO 'slack_attendance_user'@'localhost';
FLUSH PRIVILEGES;
MYSQL_SCRIPT

# Verify setup
echo "MySQL Setup Complete!"
sudo mysql -e "SELECT User, Host FROM mysql.user WHERE User='slack_attendance_user';"
sudo mysql -e "SHOW DATABASES LIKE 'slack_attendance_db';"

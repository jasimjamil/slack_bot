# Slack Standup Attendance Backend 🤖📅

## 🚀 Project Overview

A robust Rust backend for automating daily standup attendance tracking via Slack, with comprehensive reporting and role-based access control.

## 🛠 Tech Stack

- **Language**: Rust
- **Web Framework**: Actix Web
- **Database**: MySQL + SQLx
- **Authentication**: JWT, Bcrypt
- **Async Runtime**: Tokio
- **Serialization**: Serde

## ✨ Features

- 🔐 Secure JWT-based authentication
- 🤖 Slack webhook event processing
- 📊 Attendance tracking
- 🔍 Role-Based Access Control (RBAC)
- 📧 Automated notifications
- 📈 Monthly reporting system

## 🏗 Project Structure

```
slack-attendance-backend/
├── src/
│   ├── main.rs         # Application entry point
│   ├── lib.rs          # Library configuration
│   ├── routes/         # HTTP route handlers
│   ├── db/             # Database models & queries
│   ├── services/       # Business logic services
│   └── utils/          # Utility functions
├── tests/              # Integration tests
├── migrations/         # Database migration scripts
├── .env                # Environment configuration
└── Cargo.toml          # Dependency management
```

## 🚦 Getting Started

### Prerequisites

- Rust (stable)
- MySQL
- Slack App Credentials

### Installation

1. Clone the repository
2. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3. Install SQLx CLI: `cargo install sqlx-cli --no-default-features --features mysql`
4. Copy `.env.example` to `.env` and configure
5. Set up MySQL database
6. Run migrations: `sqlx migrate run`
7. `cargo build`
8. `cargo run`

## 🔐 Environment Variables

- `DATABASE_URL`: MySQL connection string
- `SLACK_BOT_TOKEN`: Slack Bot OAuth Token
- `SLACK_SIGNING_SECRET`: Slack webhook verification
- `JWT_SECRET`: Token signing secret

## 🧪 Testing

- `./run_tests.sh`: Run comprehensive test suite
- `cargo test`: Run unit tests
- `cargo clippy`: Lint checks
- `cargo fmt`: Code formatting

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch
3. Commit changes
4. Push to the branch
5. Create a Pull Request

## 📄 License

[Specify your license]

## 📞 Contact

[Your contact information] 
# Slack Standup Attendance Automation System 🤖📅

## 🚀 Project Overview

A robust Rust backend for automating daily standup attendance tracking via Slack, with comprehensive reporting and role-based access control.

## 🛠 Tech Stack

- **Web Framework**: Actix Web
- **Database**: PostgreSQL + SQLx
- **Async Runtime**: Tokio
- **Authentication**: JWT, Bcrypt
- **Serialization**: Serde
- **Email**: Lettre
- **Scheduling**: Cron

## 📦 Features

- 🔐 Secure JWT-based authentication
- 🤖 Slack webhook event processing
- 📊 Attendance tracking
- 🔍 Role-Based Access Control (RBAC)
- 📧 Automated email notifications
- 📈 Monthly reporting system

## 🏗 Project Structure

```
slack-attendance-backend/
├── src/
│   ├── main.rs         # Application entry point
│   ├── routes/         # HTTP route handlers
│   ├── db/             # Database models & queries
│   ├── services/       # Business logic services
│   └── utils/          # Utility functions
├── .env                # Environment configuration
└── Cargo.toml          # Dependency management
```

## 🚦 Getting Started

### Prerequisites

- Rust (stable)
- PostgreSQL
- Slack App Credentials
- SMTP Server for Email

### Installation

1. Clone the repository
2. Copy `.env.example` to `.env` and configure
3. `cargo build`
4. `cargo run`

## 🔐 Environment Variables

- `DATABASE_URL`: PostgreSQL connection string
- `SLACK_SIGNING_SECRET`: Slack webhook verification
- `JWT_SECRET`: Token signing secret
- `SMTP_USERNAME`: Email service credentials
- `SMTP_PASSWORD`: Email service credentials

## 🧪 Testing

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

[Your License Here]

## 📞 Contact

[Your Contact Information] 
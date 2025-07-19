use sqlx::MySqlPool;
use crate::db::models::{User, Attendance, Report, UserRole, AttendanceStatus};
use chrono::{NaiveDate, Utc};


pub struct DatabaseQueries;

impl DatabaseQueries {
    // --- User Queries ---
    pub async fn create_user(
        pool: &MySqlPool,
        name: &str,
        email: &str,
        password_hash: &str,
        role: UserRole,
        slack_user_id: Option<String>
    ) -> Result<u64, sqlx::Error> {
        let user_id = sqlx::query!(
            "INSERT INTO users (name, email, password_hash, role, slack_user_id) VALUES (?, ?, ?, ?, ?)",
            name, email, password_hash, role.to_string(), slack_user_id
        )
        .execute(pool)
        .await?
        .last_insert_id();

        Ok(user_id)
    }

    pub async fn find_user_by_email(
        pool: &MySqlPool,
        email: &str
    ) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, name, email, password_hash, role, slack_user_id, team_id 
             FROM users WHERE email = ?",
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.id as i64,
            name: r.name,
            email: r.email,
            password_hash: r.password_hash,
            role: UserRole::from(r.role),
            slack_user_id: r.slack_user_id,
            team_id: r.team_id.map(|v| v as i64),
        }))
    }

    pub async fn find_user_by_slack_id(
        pool: &MySqlPool,
        slack_user_id: &str
    ) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, name, email, password_hash, role, slack_user_id, team_id 
             FROM users WHERE slack_user_id = ?",
            slack_user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.id as i64,
            name: r.name,
            email: r.email,
            password_hash: r.password_hash,
            role: UserRole::from(r.role),
            slack_user_id: r.slack_user_id,
            team_id: r.team_id.map(|v| v as i64),
        }))
    }

    // --- Attendance Queries ---

    pub async fn log_attendance(
        pool: &MySqlPool,
        user_id: i64,
        date: NaiveDate,
        status: AttendanceStatus,
        slack_message_timestamp: Option<String>
    ) -> Result<u64, sqlx::Error> {
        let attendance_id = sqlx::query!(
            "INSERT INTO attendance (user_id, date, status, slack_message_timestamp) 
             VALUES (?, ?, ?, ?) 
             ON DUPLICATE KEY UPDATE 
             status = VALUES(status), 
             slack_message_timestamp = VALUES(slack_message_timestamp)",
            user_id,
            date,
            status.to_string(),
            slack_message_timestamp
        )
        .execute(pool)
        .await?
        .last_insert_id();

        Ok(attendance_id)
    }

    pub async fn get_user_attendance(
        pool: &MySqlPool,
        user_id: i64,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>
    ) -> Result<Vec<Attendance>, sqlx::Error> {
        match (start_date, end_date) {
            (Some(start), Some(end)) => {
                let rows = sqlx::query!(
                    "SELECT id, user_id, date, status, slack_message_timestamp 
                     FROM attendance 
                     WHERE user_id = ? AND date BETWEEN ? AND ?",
                    user_id, start, end
                )
                .fetch_all(pool)
                .await?;

                Ok(rows.into_iter().map(|r| Attendance {
                    id: r.id as i64,
                    user_id: r.user_id as i64,
                    date: r.date,
                    status: AttendanceStatus::from(r.status),
                    slack_message_timestamp: r.slack_message_timestamp,
                }).collect())
            },
            (Some(start), None) => {
                let rows = sqlx::query!(
                    "SELECT id, user_id, date, status, slack_message_timestamp 
                     FROM attendance 
                     WHERE user_id = ? AND date >= ?",
                    user_id, start
                )
                .fetch_all(pool)
                .await?;

                Ok(rows.into_iter().map(|r| Attendance {
                    id: r.id as i64,
                    user_id: r.user_id as i64,
                    date: r.date,
                    status: AttendanceStatus::from(r.status),
                    slack_message_timestamp: r.slack_message_timestamp,
                }).collect())
            },
            (None, Some(end)) => {
                let rows = sqlx::query!(
                    "SELECT id, user_id, date, status, slack_message_timestamp 
                     FROM attendance 
                     WHERE user_id = ? AND date <= ?",
                    user_id, end
                )
                .fetch_all(pool)
                .await?;

                Ok(rows.into_iter().map(|r| Attendance {
                    id: r.id as i64,
                    user_id: r.user_id as i64,
                    date: r.date,
                    status: AttendanceStatus::from(r.status),
                    slack_message_timestamp: r.slack_message_timestamp,
                }).collect())
            },
            (None, None) => {
                let rows = sqlx::query!(
                    "SELECT id, user_id, date, status, slack_message_timestamp 
                     FROM attendance 
                     WHERE user_id = ?",
                    user_id
                )
                .fetch_all(pool)
                .await?;

                Ok(rows.into_iter().map(|r| Attendance {
                    id: r.id as i64,
                    user_id: r.user_id as i64,
                    date: r.date,
                    status: AttendanceStatus::from(r.status),
                    slack_message_timestamp: r.slack_message_timestamp,
                }).collect())
            }
        }
    }

    // --- Report Queries ---

    pub async fn create_monthly_report(
        pool: &MySqlPool,
        month: NaiveDate,
        file_path: &str,
        total_employees: u32,
        present_count: u32,
        absent_count: u32
    ) -> Result<u64, sqlx::Error> {
        let report_id = sqlx::query!(
            "INSERT INTO reports 
            (month, file_path, total_employees, present_count, absent_count) 
            VALUES (?, ?, ?, ?, ?)",
            month,
            file_path,
            total_employees,
            present_count,
            absent_count
        )
        .execute(pool)
        .await?
        .last_insert_id();

        Ok(report_id)
    }

    pub async fn get_monthly_report(
        pool: &MySqlPool,
        month: NaiveDate
    ) -> Result<Option<Report>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, month, file_path, created_at, total_employees, present_count, absent_count 
             FROM reports 
             WHERE month = ?",
            month
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| Report {
            id: r.id as i64,
            month: NaiveDate::parse_from_str(&r.month.to_string(), "%Y-%m-%d").unwrap_or(month),
            file_path: r.file_path,
            created_at: r.created_at.unwrap_or_else(|| Utc::now().naive_utc()),
            total_employees: r.total_employees,
            present_count: r.present_count,
            absent_count: r.absent_count,
        }))
    }
}

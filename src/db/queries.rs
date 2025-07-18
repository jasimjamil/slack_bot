use sqlx::MySqlPool;
use crate::db::models::{User, Attendance, Report, UserRole, AttendanceStatus};
use chrono::NaiveDate;

pub struct DatabaseQueries;

impl DatabaseQueries {
    // User-related queries
    pub async fn create_user(
        pool: &MySqlPool, 
        name: &str, 
        email: &str, 
        password_hash: &str, 
        role: UserRole
    ) -> Result<u64, sqlx::Error> {
        let user_id = sqlx::query!(
            "INSERT INTO users (name, email, password_hash, role) VALUES (?, ?, ?, ?)",
            name, email, password_hash, role as _
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
        sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = ?",
            email
        )
        .fetch_optional(pool)
        .await
    }

    // Attendance-related queries
    pub async fn log_attendance(
        pool: &MySqlPool,
        user_id: u64,
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
            status as _, 
            slack_message_timestamp
        )
        .execute(pool)
        .await?
        .last_insert_id();

        Ok(attendance_id)
    }

    pub async fn get_user_attendance(
        pool: &MySqlPool,
        user_id: u64,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>
    ) -> Result<Vec<Attendance>, sqlx::Error> {
        let query = match (start_date, end_date) {
            (Some(start), Some(end)) => {
                sqlx::query_as!(
                    Attendance,
                    "SELECT * FROM attendance 
                     WHERE user_id = ? AND date BETWEEN ? AND ?",
                    user_id, start, end
                )
                .fetch_all(pool)
                .await
            },
            (Some(start), None) => {
                sqlx::query_as!(
                    Attendance,
                    "SELECT * FROM attendance 
                     WHERE user_id = ? AND date >= ?",
                    user_id, start
                )
                .fetch_all(pool)
                .await
            },
            (None, Some(end)) => {
                sqlx::query_as!(
                    Attendance,
                    "SELECT * FROM attendance 
                     WHERE user_id = ? AND date <= ?",
                    user_id, end
                )
                .fetch_all(pool)
                .await
            },
            (None, None) => {
                sqlx::query_as!(
                    Attendance,
                    "SELECT * FROM attendance WHERE user_id = ?",
                    user_id
                )
                .fetch_all(pool)
                .await
            }
        };

        query
    }

    // Report-related queries
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
        sqlx::query_as!(
            Report,
            "SELECT * FROM reports WHERE month = ?",
            month
        )
        .fetch_optional(pool)
        .await
    }
} 
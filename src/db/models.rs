use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
    pub team_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM")]
pub enum UserRole {
    Admin,
    Manager,
    Employee,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Team {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Attendance {
    pub id: u64,
    pub user_id: u64,
    pub date: chrono::NaiveDate,
    pub status: AttendanceStatus,
    pub slack_message_timestamp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM")]
pub enum AttendanceStatus {
    Present,
    Absent,
    Late,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Report {
    pub id: u64,
    pub month: chrono::NaiveDate,
    pub file_path: String,
    pub created_at: chrono::NaiveDateTime,
    pub total_employees: u32,
    pub present_count: u32,
    pub absent_count: u32,
} 
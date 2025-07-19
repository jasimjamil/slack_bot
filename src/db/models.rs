use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDate, NaiveDateTime};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
    pub team_id: Option<i64>,
    pub slack_user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Manager,
    Employee,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let role_str = match self {
            UserRole::Admin => "Admin",
            UserRole::Manager => "Manager",
            UserRole::Employee => "Employee",
        };
        write!(f, "{}", role_str)
    }
}

impl From<String> for UserRole {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Admin" => UserRole::Admin,
            "Manager" => UserRole::Manager,
            "Employee" => UserRole::Employee,
            _ => UserRole::Employee,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Team {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Attendance {
    pub id: i64,
    pub user_id: i64,
    pub date: NaiveDate,
    pub status: AttendanceStatus,
    pub slack_message_timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[serde(rename_all = "lowercase")]
pub enum AttendanceStatus {
    Present,
    Absent,
    Late,
}

impl fmt::Display for AttendanceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = match self {
            AttendanceStatus::Present => "Present",
            AttendanceStatus::Absent => "Absent",
            AttendanceStatus::Late => "Late",
        };
        write!(f, "{}", status)
    }
}

impl From<String> for AttendanceStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Present" => AttendanceStatus::Present,
            "Absent" => AttendanceStatus::Absent,
            "Late" => AttendanceStatus::Late,
            _ => AttendanceStatus::Absent,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Report {
    pub id: i64,
    pub month: NaiveDate,
    pub file_path: String,
    pub created_at: NaiveDateTime,
    pub total_employees: Option<i32>,
    pub present_count: Option<i32>,
    pub absent_count: Option<i32>,
}

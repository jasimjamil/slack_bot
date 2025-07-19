use sqlx::MySqlPool;
use crate::db::models::{Attendance, AttendanceStatus};
use crate::db::queries::DatabaseQueries;
use chrono::NaiveDate;
use std::convert::TryInto; // Required for user_id conversion

pub struct AttendanceService;

impl AttendanceService {
    pub async fn log_attendance(
        pool: &MySqlPool,
        user_id: u64,
        date: NaiveDate,
        status: AttendanceStatus,
        message: Option<String>
    ) -> Result<(), sqlx::Error> {
        // Convert u64 to i64 safely
        let user_id_i64 = user_id.try_into().unwrap(); // Panic if out of bounds

        DatabaseQueries::log_attendance(pool, user_id_i64, date, status, message).await?;
        Ok(())
    }

    pub async fn get_user_attendance(
        pool: &MySqlPool,
        user_id: u64,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>
    ) -> Result<Vec<Attendance>, sqlx::Error> {
        let user_id_i64 = user_id.try_into().unwrap();

        DatabaseQueries::get_user_attendance(pool, user_id_i64, start_date, end_date).await
    }
}

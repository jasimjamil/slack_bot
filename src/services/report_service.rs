use sqlx::MySqlPool;
use crate::db::models::Report;
use crate::db::queries::DatabaseQueries;
use chrono::NaiveDate;

pub struct ReportService;

impl ReportService {
    pub async fn generate_monthly_report(
        pool: &MySqlPool,
        month: NaiveDate,
        file_path: &str,
        total_employees: u32,
        present_count: u32,
        absent_count: u32
    ) -> Result<u64, sqlx::Error> {
        DatabaseQueries::create_monthly_report(
            pool, 
            month, 
            file_path, 
            total_employees, 
            present_count, 
            absent_count
        ).await
    }

    pub async fn get_monthly_report(
        pool: &MySqlPool,
        month: NaiveDate
    ) -> Result<Option<Report>, sqlx::Error> {
        DatabaseQueries::get_monthly_report(pool, month).await
    }
}

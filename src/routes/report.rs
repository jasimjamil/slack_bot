use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct MonthlyReport {
    month: String,
    total_employees: u32,
    present_count: u32,
    absent_count: u32,
}

#[get("/dashboard/summary")]
pub async fn get_monthly_summary() -> impl Responder {
    // TODO: Implement actual report generation logic
    let report = MonthlyReport {
        month: "January 2024".to_string(),
        total_employees: 50,
        present_count: 45,
        absent_count: 5,
    };
    HttpResponse::Ok().json(report)
} 
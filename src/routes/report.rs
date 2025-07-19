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
    let report = MonthlyReport {
        month: "July 2025".to_string(),
        total_employees: 10,
        present_count: 8,
        absent_count: 2,
    };
    HttpResponse::Ok().json(report)
}

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::MySqlPool;
use std::env;

use slack_attendance_backend::routes::{
    auth::{login, register},
    attendance::get_attendance,
    report::get_monthly_summary,
    slack::handle_slack_event,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file
    dotenv().ok();

    // Read the database URL from .env
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    // Create MySQL connection pool
    let pool = MySqlPool::connect(&db_url)
        .await
        .expect("❌ Failed to connect to MySQL");

    // Read server port (optional fallback to 8080)
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT must be a valid u16 number");

    println!("✅ Server listening on http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // ✅ Pass DB pool to handlers
            .service(login)
            .service(register)
            .service(get_attendance)
            .service(handle_slack_event)
            .service(get_monthly_summary)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}

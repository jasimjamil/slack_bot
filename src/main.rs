use actix_web::{web, App, HttpServer};
use sqlx::mysql::MySqlPool;
use std::env;

mod routes;
mod db;
mod services;
mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Setup logging
    env_logger::init();

    // Database connection pool
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create database pool");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // Authentication routes
            .service(routes::login)
            .service(routes::register)
            
            // Slack routes
            .service(routes::handle_slack_event)
            .service(routes::verify_slack_user)
            
            // Other routes
            .service(routes::attendance::get_attendance)
            .service(routes::report::get_monthly_summary)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

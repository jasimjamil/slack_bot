use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::utils::auth::{generate_jwt, verify_password, hash_password};
use crate::db::queries::DatabaseQueries;
use crate::db::models::UserRole;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[post("/login")]
pub async fn login(
    pool: web::Data<MySqlPool>, 
    credentials: web::Json<LoginRequest>
) -> impl Responder {
    match DatabaseQueries::find_user_by_email(&pool, &credentials.email).await {
        Ok(Some(user)) => {
            match verify_password(&credentials.password, &user.password_hash) {
                Ok(true) => {
                    match generate_jwt(&user.id.to_string(), &user.role.to_string()) {
                        Ok(token) => HttpResponse::Ok().json(LoginResponse { token }),
                        Err(_) => HttpResponse::InternalServerError().body("Token generation failed"),
                    }
                },
                Ok(false) => HttpResponse::Unauthorized().body("Invalid credentials"),
                Err(_) => HttpResponse::InternalServerError().body("Password verification error"),
            }
        },
        Ok(None) => HttpResponse::Unauthorized().body("User not found"),
        Err(e) => {
            log::error!("❌ Login DB error: {:?}", e);
            HttpResponse::InternalServerError().body("Database error")
        }
    }
}

#[post("/register")]
pub async fn register(
    pool: web::Data<MySqlPool>, 
    user_data: web::Json<RegisterRequest>
) -> impl Responder {
    let password_hash = match hash_password(&user_data.password) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("Password hashing failed"),
    };

    let role = user_data.role.clone();

    match DatabaseQueries::create_user(
        &pool, 
        &user_data.name, 
        &user_data.email, 
        &password_hash, 
        role,
        None
    ).await {
        Ok(_) => HttpResponse::Created().body("User registered successfully"),
        Err(e) => {
            log::error!("❌ Registration failed: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Registration failed: {}", e))
        }
    }
}

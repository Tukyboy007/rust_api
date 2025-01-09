use actix_web::{get, post, web, HttpResponse, Responder};
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;
static NEXT_ID: AtomicU32 = AtomicU32::new(1);
use chrono;
use serde_json::json;

use crate::event::user::{
    check_connection, mng_count_day, weekend_each_day, weeknd_total_mng, yesterday_total,
};
use crate::models::{CreateUser, User};
static USERS: Lazy<Mutex<Vec<User>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[get("/api")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Hello, World!"
    }))
}

#[get("/api/users")]
pub async fn get_users() -> impl Responder {
    let users = USERS.lock().unwrap();
    HttpResponse::Ok().json(&*users)
}

#[post("/api/users")]
pub async fn create_user(new_user: web::Json<CreateUser>) -> impl Responder {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

    let user = User {
        id,
        name: new_user.name.clone(),
        email: new_user.email.clone(),
    };

    USERS.lock().unwrap().push(user.clone());
    HttpResponse::Created().json(user)
}

#[get("/api/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy"
    }))
}

#[get("/api/check")]
pub async fn test_db_connection() -> impl Responder {
    match check_connection().await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Internal server error",
            "error": e.to_string(),
            "timestamp": chrono::Utc::now()
        })),
    }
}

#[get("/api/user-count")]
pub async fn user_count() -> impl Responder {
    match mng_count_day().await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Internal server error",
            "error": e.to_string(),
            "timestamp": chrono::Utc::now()
        })),
    }
}

#[get("/api/user-weekend")]
pub async fn weekend_counts() -> impl Responder {
    match weekend_each_day().await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Internal server error",
            "error": e.to_string(),
            "timestamp": chrono::Utc::now()
        })),
    }
}

#[get("/api/user-weekend-total-mng")]
pub async fn weekend_total() -> impl Responder {
    match weeknd_total_mng().await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Internal server error",
            "error": e.to_string(),
            "timestamp": chrono::Utc::now()
        })),
    }
}

#[get("/api/user-yesterday_count")]
pub async fn count_yesterday() -> impl Responder {
    match yesterday_total().await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Internal server error",
            "error": e.to_string(),
            "timestamp": chrono::Utc::now()
        })),
    }
}

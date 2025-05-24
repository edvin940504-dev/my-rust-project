
use actix_web::{post, web, HttpResponse, Responder};
use uuid::Uuid;
use serde::Deserialize;
use sqlx::PgPool;
use argon2::{Argon2, PasswordHasher, PasswordVerifier}; // Add at the top
use password_hash::{PasswordHash, SaltString, rand_core::OsRng};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[post("/register")]
async fn register_user(
    pool: web::Data<PgPool>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    //let id = 0;
    //Uuid::new_v4();

    // Hash password securely with Argon2
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    // Save to database
    let result = sqlx::query!(
        r#"
        INSERT INTO users (email, password)
        VALUES ($1, $2)
        "#,
        req.email,
        password_hash
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "email": req.email })),
        Err(e) => {
            eprintln!("Database insert error: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "User registration failed",
                "details": e.to_string()
            }))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("/login")]
async fn login_user(
    pool: web::Data<PgPool>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    let user = sqlx::query!(
        "SELECT id, email, password FROM users WHERE email = $1",
        req.email
    )
    .fetch_optional(pool.get_ref())
    .await;

    match user {
        Ok(Some(user)) => {
            let parsed_hash = PasswordHash::new(&user.password).unwrap();
            if Argon2::default()
                .verify_password(req.password.as_bytes(), &parsed_hash)
                .is_ok()
            {
                HttpResponse::Ok().json(serde_json::json!({
                    "id": user.id,
                    "email": user.email
                }))
            } else {
                HttpResponse::Unauthorized().body("Invalid password")
            }
        }
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            eprintln!("Login query error: {:?}", e);
            HttpResponse::InternalServerError().body("Login failed")
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user);
    cfg.service(login_user); // ADD THIS
}
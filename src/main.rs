mod models;
mod routes;

use actix_web::{web, App, HttpServer};
use routes::{user_routes, wallet_routes, transaction_routes};
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
// or: use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use crate::models::wallet::WALLETS;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // <- Load .env variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let wallets_data = web::Data::new(&*WALLETS); // Add this above HttpServer::new

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(&*WALLETS)) // 👈 Add this line
            .configure(user_routes::init_routes)
            .configure(wallet_routes::init_routes)
            .configure(transaction_routes::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
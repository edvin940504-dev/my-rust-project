use actix_web::{post, web, HttpResponse, Responder};
use uuid::Uuid;
use chrono::Utc;

use crate::models::transaction::Transaction;

#[derive(serde::Deserialize)]
pub struct TransactionRequest {
    pub from_wallet: Uuid,
    pub to_wallet: Uuid,
    pub amount: f64,
}

#[post("/transactions")]
async fn create_transaction(req: web::Json<TransactionRequest>) -> impl Responder {
    let req = req.into_inner();

    let transaction = Transaction {
        id: Uuid::new_v4(),
        from_wallet: req.from_wallet,
        to_wallet: req.to_wallet,
        amount: req.amount,
        timestamp: Utc::now().to_rfc3339(),
    };

    // Note: Balance checks, wallet updates will be added in next step

    HttpResponse::Ok().json(transaction)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_transaction);
}
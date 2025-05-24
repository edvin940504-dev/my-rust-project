use actix_web::{post, get, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::wallet::{Wallet, WALLETS};

use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct CreateWalletRequest {
    pub user_id: Uuid,
}

pub async fn create_wallet(
    wallet_req: web::Json<CreateWalletRequest>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let new_wallet = Wallet {
        id: Uuid::new_v4(),
        user_id: wallet_req.user_id,
        balance: 0.0,
    };

    // ✅ INSERT INTO database
    let result = sqlx::query!(
        "INSERT INTO wallets (id, user_id, balance) VALUES ($1, $2, $3)",
        new_wallet.id,
        new_wallet.user_id,
        new_wallet.balance
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(&new_wallet),
        Err(e) => {
            eprintln!("DB insert error: {:?}", e);
            HttpResponse::InternalServerError().body("Error saving wallet")
        }
    }
}

/*
#[post("/wallets")]
async fn create_wallet(req: web::Json<CreateWalletRequest>) -> impl Responder {
    let wallet = Wallet {
        id: Uuid::new_v4(),
        user_id: req.user_id,
        balance: 0.0,
    };

    WALLETS.lock().unwrap().insert(wallet.id, wallet.clone());

    HttpResponse::Ok().json(wallet)
}
*/

#[get("/wallets/{user_id}")]
async fn list_wallets(
    path: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let user_id = path.into_inner();

    let result = sqlx::query_as!(
        Wallet,
        "SELECT * FROM wallets WHERE user_id = $1",
        user_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(wallets) => HttpResponse::Ok().json(wallets),
        Err(e) => {
            eprintln!("DB query error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to load wallets")
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/wallets", web::post().to(create_wallet));
//  cfg.service(create_wallet);
    cfg.service(list_wallets);
}
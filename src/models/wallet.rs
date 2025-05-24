use serde::{Deserialize, Serialize};
use uuid::Uuid;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: f64,
}

lazy_static! {
    pub static ref WALLETS: Mutex<HashMap<Uuid, Wallet>> = Mutex::new(HashMap::new());
}
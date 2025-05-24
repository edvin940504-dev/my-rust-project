use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub from_wallet: Uuid,
    pub to_wallet: Uuid,
    pub amount: f64,
    pub timestamp: String,
}
//use serde::Serialize;
use uuid::Uuid;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Debug, serde::Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
}

lazy_static! {
    pub static ref USERS: Mutex<HashMap<Uuid, User>> = Mutex::new(HashMap::new());
}
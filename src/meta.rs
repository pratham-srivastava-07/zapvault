use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Vault {
    pub password_hash: String,
    // add other fields here if needed, e.g., creation_date: String
}
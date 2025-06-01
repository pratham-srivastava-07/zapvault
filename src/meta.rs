use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Vault {
    pub password_hash: String,
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
}
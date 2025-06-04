use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Vault {
    pub password_hash: String,
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
}
#[derive(Serialize, Deserialize, Debug)]
pub struct VaultWrite {
    pub password_hash: String,
    pub entries: Vec<Vault>,
}
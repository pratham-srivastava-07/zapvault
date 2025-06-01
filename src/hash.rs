use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let argon = Argon2::default();

    let hashed_password = argon
                                      .hash_password(password.as_bytes(), &salt)
                                      .unwrap()
                                      .to_string();

    hashed_password
}
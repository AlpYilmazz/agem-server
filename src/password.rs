use bcrypt::{DEFAULT_COST, hash, verify};

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    match verify(password, hashed_password) {
        Ok(r) => r,
        Err(_err) => false,
    }
}

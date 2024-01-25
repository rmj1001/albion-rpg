use bcrypt::{hash, DEFAULT_COST};

pub fn generate_hash(text: String) -> String {
    let hashed_result = hash(text, DEFAULT_COST);

    match hashed_result {
        Ok(password_hash) => password_hash,
        Err(error) => panic!("Failed to generate password hash: {}", error),
    }
}

pub fn verify_hash(text: String, hash: String) -> bool {
    let verified_result = bcrypt::verify(text, &hash);

    match verified_result {
        Ok(result) => result,
        Err(error) => panic!("Failed to verify password hash: {}", error),
    }
}

#[cfg(test)]
mod tests {
    use crate::misc::crypt::{generate_hash, verify_hash};

    #[test]
    fn check_hashing() {
        const PASSWORD: &str = "1234";
        let hashed = generate_hash(PASSWORD.to_string());

        assert!(
            verify_hash(PASSWORD.to_string(), hashed),
            "The hashes of {} didn't match.",
            PASSWORD
        );
    }
}

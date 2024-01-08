use bcrypt::{hash, DEFAULT_COST};

pub fn generate(password: String) -> String {
    let hashed_result = hash(password, DEFAULT_COST);

    match hashed_result {
        Ok(password_hash) => password_hash,
        Err(error) => panic!("Failed to generate password hash: {}", error),
    }
}

pub fn verify(tried_password: String, user_password: String) -> bool {
    let verified_result = bcrypt::verify(tried_password, &user_password);

    match verified_result {
        Ok(result) => result,
        Err(error) => panic!("Failed to verify password hash: {}", error),
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::crypt::{generate, verify};

    #[test]
    fn check_hashing() {
        const PASSWORD: &str = "1234";
        let hashed = generate(PASSWORD.to_string());

        assert!(
            verify(PASSWORD.to_string(), hashed),
            "The hashes of {} didn't match.",
            PASSWORD
        );
    }
}

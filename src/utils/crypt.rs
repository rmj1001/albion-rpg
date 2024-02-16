use bcrypt::{hash, DEFAULT_COST};

/// Generate a Hash.
///
/// # Examples
///
/// ```
/// use albion_terminal_rpg::utils::crypt::generate_hash;
///
/// let password: String = String::from("test");
///
/// let hashed: String = generate_hash(password);
/// ```
///
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
    #[test]
    fn check_hashing() {
        use super::{generate_hash, verify_hash};

        const PASSWORD: &str = "1234";
        let hashed = generate_hash(PASSWORD.to_string());

        assert!(
            verify_hash(PASSWORD.to_string(), hashed),
            "The hashes of {} didn't match.",
            PASSWORD
        );
    }
}

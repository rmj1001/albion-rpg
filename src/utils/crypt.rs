/*!
# Cryptography

Generate and verify hash string signatures for protecting secrets.
*/

use crate::panic_screen;
use bcrypt::{hash, DEFAULT_COST};

/**
Generate a hashed string for secrets

# Examples

```
use albion_terminal_rpg::prelude::*;

let password: String = String::from("test");
let hashed: String = generate_hash(password.clone());
```
*/

pub fn generate_hash(text: String) -> String {
    let hashed_result = hash(text, DEFAULT_COST);

    match hashed_result {
        Ok(password_hash) => password_hash,
        Err(error) => panic_screen!("Failed to generate password hash: {}", error),
    }
}

/**
Verify that a string matches a hash

# Examples
```
use albion_terminal_rpg::prelude::*;

let password: String = String::from("test");
let hashed: String = generate_hash(password.clone());

assert!(verify_hash(password, hashed));
```
*/
pub fn verify_hash(text: String, hash: String) -> bool {
    let verified_result = bcrypt::verify(text, &hash);

    match verified_result {
        Ok(result) => result,
        Err(error) => panic_screen!("Failed to verify password hash: {}", error),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_hashing() {
        use crate::prelude::{generate_hash, verify_hash};

        const PASSWORD: &str = "1234";
        let hashed = generate_hash(PASSWORD.to_string());

        assert!(
            verify_hash(PASSWORD.to_string(), hashed),
            "The hashes of {} didn't match.",
            PASSWORD
        );
    }
}

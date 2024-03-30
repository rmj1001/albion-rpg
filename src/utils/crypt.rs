/*!
# Cryptography

Generate and verify hash string signatures for protecting secrets.
*/

use std::fmt::Display;

use crate::panic_menu;
use bcrypt::{hash, DEFAULT_COST};

/**
Generate a hashed string for secrets

# Examples

```
use albion_terminal_rpg::prelude::*;

let password: String = String::from("test");
let hashed: String = generate_hash(&password);
```
*/

pub fn generate_hash<T: Display>(text: &T) -> String {
    match hash(text.to_string(), DEFAULT_COST) {
        Ok(password_hash) => password_hash,
        Err(error) => panic_menu!("Failed to generate password hash: {}", error),
    }
}

/**
Verify that a string matches a hash

# Examples
```
use albion_terminal_rpg::prelude::*;

let password: String = String::from("test");
let hashed: String = generate_hash(&password);

assert!(verify_hash(&password, &hashed));
```
*/
pub fn verify_hash<T: Display, U: Display>(text: &T, hash: &U) -> bool {
    match bcrypt::verify(text.to_string(), &hash.to_string()) {
        Ok(result) => result,
        Err(error) => panic_menu!("Failed to verify password hash: {}", error),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_hashing() {
        use crate::prelude::{generate_hash, verify_hash};

        const PASSWORD: &str = "1234";
        let hashed = generate_hash(&PASSWORD);

        assert!(
            verify_hash(&PASSWORD, &hashed),
            "The hashes of {} didn't match.",
            PASSWORD
        );
    }
}

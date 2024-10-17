use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub async fn generate_hash(input_string: &String) -> Result<String, argon2::password_hash::Error>{

    let byte_input = input_string.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let hash = argon2.hash_password(byte_input, &salt)?.to_string();

    Ok(hash)
}

pub async fn validate_hash(stored_hash: &String, input_string: &String) -> Result<bool, argon2::password_hash::Error> {
    // Verify password against PHC string.
    //
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // `Argon2` instance.
    let parsed_hash = PasswordHash::new(&stored_hash)?;

    Ok(Argon2::default().verify_password(input_string.as_bytes(), &parsed_hash).is_ok())
}
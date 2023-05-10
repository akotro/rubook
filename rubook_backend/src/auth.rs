use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{web, HttpRequest, HttpResponse};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rubook_lib::user::UserClaims;

pub fn generate_password_hash(password: String) -> Result<String, argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

pub fn validate_password(stored_hash: &str, password: &str) -> bool {
    let parsed_hash = match PasswordHash::new(stored_hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    let argon2 = Argon2::default();
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => true,
        Err(_) => false,
    }
}

pub fn generate_token(req: &HttpRequest, username: String) -> String {
    let claims = UserClaims {
        sub: username,
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let secret_key = req
        .app_data::<web::Data<String>>()
        .expect("Missing app data: secret key")
        .as_ref();

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )
    .unwrap()
}

pub fn validate_token(req: &HttpRequest) -> Result<(), HttpResponse> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or_else(|| HttpResponse::Unauthorized().finish())?;

    let secret_key = req
        .app_data::<web::Data<String>>()
        .expect("Missing app data: secret key")
        .as_ref();

    let validation = Validation::default();
    let user_claims = decode::<UserClaims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes().as_ref()),
        &validation,
    )
    .map_err(|_| HttpResponse::Unauthorized().finish())?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;
    if user_claims.claims.exp < now {
        return Err(HttpResponse::Unauthorized().finish());
    }

    Ok(())
}

pub fn validate_ip(req: &HttpRequest) -> Result<(), HttpResponse> {
    let connection_info = req.connection_info();
    let ip = connection_info
        .realip_remote_addr()
        .ok_or_else(|| HttpResponse::Unauthorized().finish())?;

    let ip_blacklist = req
        .app_data::<web::Data<Vec<String>>>()
        .expect("Missing app data: ip blacklist")
        .as_ref();

    if ip_blacklist.contains(&ip.to_string()) {
        println!("Blocked ip: {ip}");
        return Err(HttpResponse::Unauthorized().finish());
    }

    Ok(())
}

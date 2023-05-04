use actix_web::{web, HttpRequest, HttpResponse};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rubook_lib::user::UserClaims;

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

    decode::<UserClaims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes().as_ref()),
        &Validation::default(),
    )
    .map(|_| ())
    .map_err(|_| HttpResponse::Unauthorized().finish())
}

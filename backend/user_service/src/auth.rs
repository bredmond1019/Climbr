use actix_web::HttpRequest;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use log::info;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{config::get_secret_key, graphql::schema::Context};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

lazy_static! {
    static ref SECRET_KEY: Vec<u8> = get_secret_key();
}

pub fn create_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize
        + 60 * 60; // 1 hour expiration

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&SECRET_KEY),
    )
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(&SECRET_KEY),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
}

pub fn authenticate(req: &HttpRequest, ctx: &mut Context) -> Result<(), actix_web::Error> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    if let Some(auth_value) = auth_header {
        if let Some(token) = auth_value.strip_prefix("Bearer ") {
            match validate_jwt(token) {
                Ok(claims) => {
                    info!("Valid token for user: {}", claims.sub);
                    ctx.user_id = Some(claims.sub);
                    Ok(())
                }
                Err(_) => {
                    info!("Invalid token");
                    Err(actix_web::error::ErrorUnauthorized("Invalid token"))
                }
            }
        } else {
            Err(actix_web::error::ErrorUnauthorized("Invalid token format"))
        }
    } else {
        Err(actix_web::error::ErrorUnauthorized(
            "Authorization header missing",
        ))
    }
}

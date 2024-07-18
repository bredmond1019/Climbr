use actix_web::{
    dev::ServiceRequest,
    web::{self, Data},
    Error, HttpMessage, HttpRequest,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::Mutex;

use crate::{config::get_secret_key, graphql::schema::Context};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: UserResponse,
    exp: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
}

lazy_static! {
    static ref SECRET_KEY: Vec<u8> = get_secret_key();
}

pub fn create_jwt(user_response: &UserResponse) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize
        + 60 * 60; // 1 hour expiration

    let claims = Claims {
        sub: user_response.to_owned(),
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

pub async fn authenticator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    debug!("Authenticating request");
    info!("Authenticating request");
    // let auth_token = credentials.token();
    info!("Token: {:?}", credentials);
    Ok(req)
    // if let Ok(claims) = validate_jwt(auth_token) {
    //     info!("Valid token for user: {:?}", claims.sub);
    //     // req.app_data::<Context>().unwrap().user = Some(claims.sub);
    //     req.extensions_mut().insert(claims);

    //     Ok(req)
    // } else {
    //     info!("Invalid token");
    //     Err((actix_web::error::ErrorUnauthorized("Invalid token"), req))
    // }
}

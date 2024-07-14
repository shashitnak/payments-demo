use crate::routes::Error::JWTValidationFailed;
use crate::routes::{Error::JWTCreationFailed, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use sqlx::types::Uuid;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: Uuid,
    pub exp: u64,
}

#[derive(Clone)]
pub struct Jwt {
    secret: String,
}

impl Jwt {
    pub fn new(secret: &str) -> Self {
        Jwt {
            secret: secret.into(),
        }
    }
    pub fn generate_token(&self, id: Uuid) -> Result<String> {
        let claims = Claims {
            id,
            exp: (Utc::now() + Duration::from_secs(3600)).timestamp() as u64,
        };

        let header = Header {
            alg: Algorithm::HS512,
            ..Default::default()
        };

        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| JWTCreationFailed)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        decode(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(Algorithm::HS512),
        )
        .map_err(|_| JWTValidationFailed)
        .map(|token_data| token_data.claims)
    }
}

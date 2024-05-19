use jsonwebtoken::{EncodingKey, Header};
use serde::Serialize;

use crate::errors::{Error, GeneralError};

pub struct JwtService;

#[derive(Serialize)]
pub struct Claims {
    pub id: i32,
    pub sub: String,
}

impl JwtService {
    pub fn generate(user_id: i32, user_email: String) -> Result<String, Error> {
        let claims = Claims {
            id: user_id,
            sub: user_email,
        };

        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("khubsecret".as_ref()),
        ).map_err(|_| Error::GeneralError(GeneralError::Unhandled("Failed to encode token".to_string())))?;

        Ok(token)
    }
}

use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

static KEY: [u8; 16] = *include_bytes!("../../secret.key");
static ONE_DAY: i64 = 60 * 60 * 24;

#[allow(dead_code)]
pub fn decode(token: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(&KEY),
        &Validation::default(),
    )
}

pub fn generate(user_id: Uuid, now: i64) -> Result<String, Error> {
    let claims = Claims::new(user_id, now);
    jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(&KEY))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    iat: i64,

    pub user_id: Uuid,
}

impl Claims {
    pub fn new(user_id: Uuid, now: i64) -> Self {
        Claims {
            exp: now + ONE_DAY,
            iat: now,
            user_id,
        }
    }
}

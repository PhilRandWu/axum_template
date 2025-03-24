use crate::models::user::User;
use bson::oid::ObjectId;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static VALIDATION: Lazy<Validation> = Lazy::new(|| {
    let mut validation = Validation::default();
    validation.leeway = 60; // 允许1分钟的时间偏差
    validation
});
static HEADER: Lazy<Header> = Lazy::new(Header::default);

const ACCESS_TOKEN_DURATION: i64 = 2 * 60 * 60; // 2小时
const REFRESH_TOKEN_DURATION: i64 = 30 * 24 * 60 * 60; // 30天

type TokenResult = Result<TokenData<Claims>, Error>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenUser {
    pub id: ObjectId,
    pub name: String,
    pub email: String,
}

impl From<User> for TokenUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id.unwrap(),
            name: user.name.clone(),
            email: user.email,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize, // Expiration time
    pub iat: usize,
    pub user: TokenUser,
    pub token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

impl Claims {
    pub fn new(user: User, token_type: TokenType) -> Self {
        let now = chrono::Local::now();
        let duration = match token_type {
            TokenType::Access => chrono::Duration::seconds(ACCESS_TOKEN_DURATION),
            TokenType::Refresh => chrono::Duration::seconds(REFRESH_TOKEN_DURATION),
        };

        Self {
            exp: (now + duration).timestamp() as usize,
            iat: now.timestamp() as usize,
            user: TokenUser::from(user),
            token_type,
        }
    }

    pub fn is_refresh_token(&self) -> bool {
        self.token_type == TokenType::Refresh
    }
}

pub fn create(user: User, secret: &str) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    let claims = Claims::new(user, TokenType::Access);

    jsonwebtoken::encode(&HEADER, &claims, &encoding_key)
}

pub fn decode(token: &str, secret: &str) -> TokenResult {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());

    jsonwebtoken::decode::<Claims>(token, &decoding_key, &VALIDATION)
}

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{error::AppError, oauth::LinuxDoUser};

// 定义 JWT 的数据结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user: LinuxDoUser,
    pub exp: u64,
}

// 生成 JWT
pub fn create_jwt(user: LinuxDoUser) -> Result<String, AppError> {
    let my_claims = Claims {
        user,
        exp: (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 30 * 24 * 3600),
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )?;
    Ok(token)
}

// 验证 JWT
pub fn verify_jwt(token: &str) -> Result<TokenData<Claims>, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data)
}

mod sign;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::http::header;
use actix_web::{Error, HttpRequest};

use alloy::providers::{Provider, ProviderBuilder};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,        // 用户ID
    exp: usize,         // 过期时间
    address: String    //wallet address
}

/// 加载 SECRET_KEY
fn get_secret_key() -> String {
    env::var("SECRET_KEY").expect("SECRET_KEY must be set in .env file")
}

pub fn generate_token(user_id: &str, address: &str) -> String {
    let secret_key = get_secret_key();
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 3600 * 24 * 7; // 1小时有效期

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
        address: address.to_owned()
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
        .unwrap()
}

pub fn validate_token(req: &HttpRequest) -> Result<Claims, Error> {
    let token = req.headers()
        .get(header::AUTHORIZATION) // 获取 `Authorization` 头部
        .and_then(|value| value.to_str().ok()) // 将头部值转换为 `&str`
        .and_then(|auth| auth.strip_prefix("Bearer ")) // 去掉 "Bearer " 前缀
        .map(|token| token.to_string()) // 转换为 `String`
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid or missing token"))? ;// 返回错误
    let secret_key = get_secret_key();
    decode::<Claims>(
        &*token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::default(),
    )
        .map(|data| data.claims).map_err(|_| actix_web::error::ErrorUnauthorized("decode token error"))
}

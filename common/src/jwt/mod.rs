use crate::error::{role_error, CodeError};
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthInfo {
    pub id: i64,
    pub device: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    exp: u64,
    iat: u64,
    nbf: u64,
    user: AuthInfo,
}

pub fn new_jwt(id: i64, device: &str) -> Result<String, CodeError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| role_error("系统时间异常"))?
        .as_secs();
    let claims = Claims {
        exp: now + config::CFG.jwt.jwt_exp,
        iat: now,
        nbf: now,
        user: AuthInfo {
            id,
            device: device.to_string(),
        },
    };
    let header = Header {
        alg: Algorithm::HS256,
        ..Default::default()
    };
    let secret = config::CFG.jwt.jwt_secret.as_bytes();
    encode(&header, &claims, &EncodingKey::from_secret(secret))
        .map_err(|_| role_error("jwt生成失败"))
}

pub fn verify_jwt(token: &str) -> Result<AuthInfo, CodeError> {
    let secret = config::CFG.jwt.jwt_secret.as_bytes();
    let data = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| role_error("jwt验证失败"))?;
    Ok(data.claims.user)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_verify_jwt() {
        let token = new_jwt(7, "web").expect("create jwt should succeed");
        let auth_info = verify_jwt(&token).expect("verify jwt should succeed");
        assert_eq!(
            auth_info,
            AuthInfo {
                id: 7,
                device: "web".to_string()
            }
        );
    }
}

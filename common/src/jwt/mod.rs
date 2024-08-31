use crate::error::{role_error, CodeError};
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time;

// jwt 用户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthInfo {
    id: i64,   // 用户id
    device: String, // 设备
}
// jwt 配置
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: u64,    // 过期时间
    iat: u64,    // 签发时间
    nbf: u64,     // 生效时间
    user: AuthInfo,    // 用户信息
}


// 生成jwt
pub fn new_jwt(id: i64, device: &str) -> Result<String, CodeError> {
    let auth_info = AuthInfo {
        id,
        device: device.to_string(),
    };
    // 获取当前时间
    let now = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = Claims {
        exp: now + config::CFG.jwt.jwt_exp,
        iat: now,
        nbf: now,
        user: auth_info,
    };
    let header =
        Header { alg: Algorithm::HS256, ..Default::default() };
    match encode(&header, &claims, &EncodingKey::from_secret(config::CFG.jwt.jwt_secret.as_ref())) {
        Ok(token) => {
            Ok(token)
        }
        Err(_) => {
            Err(role_error("jwt生成失败"))
        }
    }
}

pub fn verify_jwt(token: &str) -> Result<AuthInfo, CodeError> {
    let key = config::CFG.jwt.jwt_secret.as_ref();
    let result = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(key),
        &Validation::new(Algorithm::HS256),
    );
    match result {
        Ok(data) => {
            Ok(data.claims.user)
        }
        Err(_) => {
            Err(role_error("jwt验证失败"))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_jwt() {
        let jwt = new_jwt(1, "web");
        match jwt {
            Ok(jwt) => {
                println!("{:?}", jwt);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    #[test]
    fn test_verify_jwt() {
        let jwt = new_jwt(1, "web");
        let jwt = jwt.unwrap();
        let auth_info = verify_jwt(&jwt);
        println!("{:?}", auth_info.unwrap());
    }
}
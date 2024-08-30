use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time;

// jwt 用户信息
#[derive(Debug, Serialize, Deserialize)]
struct AuthInfo {
    id: i64,   // 用户id
    device: String, // 设备
}
// jwt 配置
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,    // 主题
    exp: u64,    // 过期时间
    iat: u64,    // 签发时间
    user: AuthInfo,    // 用户信息
}


// 生成jwt
pub fn new_jwt(id: i64, device: &str) -> String {
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
        sub: "jwt".to_string(),
        exp: now + config::CFG.jwt.jwt_exp,
        iat: now,
        user: auth_info,
    };
    let header =
        Header { alg: Algorithm::HS256, ..Default::default() };
    match encode(&header, &claims, &EncodingKey::from_secret(config::CFG.jwt.jwt_secret.as_ref())) {
        Ok(token) => {
            token
        }
        Err(_) => {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_jwt() {
        let jwt = new_jwt(1, "web");
        println!("{:?}", jwt);
    }
}
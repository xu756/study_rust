use crate::error::errors::*;
use serde::{Deserialize, Serialize};

pub mod errors;

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeError {
    pub code: ResultError,
    pub msg: String,
}
// 权限错误
pub fn role_error(msg: &str) -> CodeError {
    CodeError {
        code: ResultError::RoleError,
        msg: msg.to_string(),
    }
}

// 警告
pub fn warn(msg: &str) -> CodeError {
    CodeError {
        code: ResultError::WarnError,
        msg: msg.to_string(),
    }
}

// 数据库错误
pub fn db_error(msg: &str) -> CodeError {
    CodeError {
        code: ResultError::SystemError,
        msg: msg.to_string(),
    }
}

// 参数错误
pub fn param_error(msg: &str) -> CodeError {
    CodeError {
        code: ResultError::ParamError,
        msg: msg.to_string(),
    }
}
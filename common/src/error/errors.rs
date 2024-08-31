use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ResultError {
    Ok,
    ParamError,
    SystemError,
    RoleError,
    WarnError,
}

pub fn error_code_to_int(err: &ResultError) -> i32 {
    match err {
        ResultError::Ok => 0,
        ResultError::ParamError => 201,
        ResultError::SystemError => 202,
        ResultError::RoleError => 203,
        ResultError::WarnError => 204,
    }
}



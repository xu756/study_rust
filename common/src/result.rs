use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
enum ResultError {
    Ok = 0,
    ParamError = 201,
    SystemError = 202,
    RoleError = 203,
    WarnError = 204,
}

fn error_code_to_int(err: &ResultError) -> i32 {
    match err {
        ResultError::Ok => 0,
        ResultError::ParamError => 201,
        ResultError::SystemError => 202,
        ResultError::RoleError => 203,
        ResultError::WarnError => 204,
    }
}

// 响应结构体
#[derive(Serialize)]
pub struct Response<T> {
    success: bool,
    err_code: i32,
    err_msg: String,
    data: T, // 使用Option来表示数据可能不存在
}

// 成功响应
pub fn success<T: Serialize>(data: T) -> Json<Value> {
    let res = Response {
        success: true,
        err_code: error_code_to_int(&ResultError::Ok),
        err_msg: "".to_string(),
        data,
    };
    Json(json!(res))
}


// 失败响应
pub fn error(err_msg: &str) -> Json<Value> {
    let res = Response {
        success: false,
        err_code: error_code_to_int(&ResultError::SystemError),
        err_msg: err_msg.to_string(),
        data: "",
    };
    Json(json!(res))
}

// 参数错误
pub fn param_error(err_msg: &str) -> Json<Value> {
    let res = Response {
        success: false,
        err_code: error_code_to_int(&ResultError::ParamError),
        err_msg: err_msg.to_string(),
        data: "",
    };
    Json(json!(res))
}

// 权限错误
pub fn role_error(err_msg: &str) -> Json<Value> {
    let res = Response {
        success: false,
        err_code: error_code_to_int(&ResultError::RoleError),
        err_msg: err_msg.to_string(),
        data: "",
    };
    Json(json!(res))
}

// 警告错误
pub fn warn_error(err_msg: &str) -> Json<Value> {
    let res = Response {
        success: false,
        err_code: error_code_to_int(&ResultError::WarnError),
        err_msg: err_msg.to_string(),
        data: "",
    };
    Json(json!(res))
}
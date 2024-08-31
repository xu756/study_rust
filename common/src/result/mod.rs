use crate::error::errors::{error_code_to_int, ResultError};
use crate::error::CodeError;
use axum::Json;
use serde::Serialize;
use serde_json::{json, Value};

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
pub fn error(error: CodeError) -> Json<Value> {
    let res = Response {
        success: false,
        err_code: error_code_to_int(&error.code),
        err_msg: error.msg,
        data: "",
    };
    Json(json!(res))
}
use actix_web::{web::Json, ResponseError as actixResponseError};
use serde::Serialize;

use crate::prelude::Error;

#[derive(Serialize)]
pub struct Response<T> {
    pub success: bool,
    pub body: ResponseResult<T>,
}

impl<ResponseError> Response<ResponseError> {
    pub fn new_error(code: u16, message: String) -> Self {
        Self {
            success: false,
            body: ResponseResult::Err(crate::api::response::ResponseError { code, message }),
        }
    }
}

impl<ResponseError> From<Error> for Response<ResponseError> {
    fn from(value: Error) -> Self {
        Response {
            success: false,
            body: ResponseResult::Err(value.into()),
        }
    }
}

impl<T> Response<T> {
    pub fn new_success(body: T) -> Self {
        Self {
            success: true,
            body: ResponseResult::Ok(body),
        }
    }
}

impl<T> From<Response<T>> for Json<Response<T>> {
    fn from(val: Response<T>) -> Self {
        Json(val)
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ResponseResult<T> {
    Ok(T),
    Err(ResponseError),
}

#[derive(Serialize)]
pub struct ResponseError {
    code: u16,
    message: String,
}

impl From<Error> for ResponseError {
    fn from(value: Error) -> Self {
        Self {
            code: value.status_code().into(),
            message: value.to_string(),
        }
    }
}

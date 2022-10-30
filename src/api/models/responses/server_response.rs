// use crate::api::models::dto;
use serde::Serialize;
use serde_with;
use axum::{
    http::StatusCode,
    Json
};

#[serde_with::skip_serializing_none]
#[derive(Serialize)]
pub struct Response<T> {
    status: u16,
    data: Option<T>,
    error: Option<String>
}

impl<T> Response<T> {
    pub fn ok(data: T) -> (StatusCode, Json<Response<T>>) {
        let status_code = StatusCode::OK;
        let response = Response {
            status: status_code.as_u16(),
            data: Some(data),
            error: None
        };

        (status_code, Json(response))
    }
    
    pub fn error(status: StatusCode, message: String) -> (StatusCode, Json<Response<T>>) {
        let response = Response { 
            status:  status.as_u16(),
            data: None,
            error: Some(message)
        };

        (status, Json(response))
    }
}
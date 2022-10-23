// use crate::api::models::dto;
use serde::Serialize;
use serde_with;
use axum::http::StatusCode;

#[serde_with::skip_serializing_none]
#[derive(Serialize)]
pub struct Response<T> {
    status: u16,
    data: Option<T>,
    error: Option<String>
}

impl<T> Response<T> {
    pub fn ok(data: T) -> Response<T> {
        Response {
            status: StatusCode::OK.as_u16(),
            data: Some(data),
            error: None
        }
    }
    
    pub fn server_error(message: String) -> Response<T> {
        Response { 
            status:  StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            data: None,
            error: Some(message)
        }
    }
}
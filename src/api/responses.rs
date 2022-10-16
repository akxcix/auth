use serde;
use serde_with;
use axum::http::StatusCode;

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize)]
pub struct Response<T> {
    status: u16,
    data: Option<T>,
    error: Option<String>
}

pub fn ok<T: serde::Serialize>(data: T) -> Response<T> {
    Response {
        status: StatusCode::OK.as_u16(),
        data: Some(data),
        error: None
    }
}

pub fn server_error<T: serde::Serialize>(message: String) -> Response<T> {
    Response { 
        status:  StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        data: None,
        error: Some(message)
    }
}

pub fn bad_request<T: serde::Serialize>(message: String) -> Response<T> {
    Response { 
        status: StatusCode::BAD_REQUEST.as_u16(), 
        data: None,
        error: Some(message)
    }
}
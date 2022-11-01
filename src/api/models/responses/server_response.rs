use serde::Serialize;
use axum::{
    http::StatusCode,
    Json, 
    response::IntoResponse,
};

pub struct Response<T: Serialize> {
    headers: Vec<(String, String)>,
    status_code: StatusCode,
    data: Option<T>,
    error: Option<String>
}

impl<T: Serialize> Response<T> {
    pub fn ok(data: T) -> Response<T> {
        let status_code = StatusCode::OK;
        let headers = vec![];

        Response {
            headers: headers,
            status_code: status_code,
            data: Some(data),
            error: None
        }
    }
    
    pub fn error(status_code: StatusCode, message: String) -> Response<T> {
        let headers = vec![];

        Response { 
            headers: headers,
            status_code:  status_code,
            data: None,
            error: Some(message)
        }
    }
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code;
        let json = Json(self.data);

        (status_code, json).into_response()
    }
}

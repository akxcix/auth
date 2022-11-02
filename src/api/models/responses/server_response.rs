use serde::Serialize;
use axum::{
    http::StatusCode,
    Json, 
    response::IntoResponse,
};

#[derive(Serialize)]
pub struct Body<T: Serialize> {
    data: Option<T>,
    error: Option<String>
}

pub struct Response<T: Serialize> {
    headers: Vec<(String, String)>,
    status_code: StatusCode,
    body: Body<T>,
}

impl<T: Serialize> Response<T> {
    pub fn ok(data: T) -> Response<T> {
        let status_code = StatusCode::OK;
        let headers = vec![];
        let body = Body {
            data: Some(data),
            error: None,
        };

        Response {
            headers: headers,
            status_code: status_code,
            body: body,
        }
    }
    
    pub fn error(status_code: StatusCode, message: String) -> Response<T> {
        let headers = vec![];
        let body = Body {
            data: None,
            error: Some(message),
        };

        Response { 
            headers: headers,
            status_code:  status_code,
            body: body,
        }
    }
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code;
        let json = Json(self.body);

        (status_code, json).into_response()
    }
}

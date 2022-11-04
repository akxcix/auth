use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use crate::api::dtos::commons::User;

// Base Response ----------------------------------------------------------------------------------
#[derive(Serialize)]
struct Body<T: Serialize> {
    data: Option<T>,
    error: Option<String>,
}

pub struct Response<T: Serialize> {
    status: StatusCode,
    body: Body<T>
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        let json = Json(self.body);

        (status, json).into_response()
    }
}

impl Response<String> {
    pub fn error(status: StatusCode, message: String) -> Response<String> {
        let body = Body { data: None, error: Some(message)};

        Response { status, body }
    }
}

impl<T: Serialize> Response<T> {
    pub fn ok(data: T) -> Response<T> {
        let status = StatusCode::OK;
        let body = Body {
            data: Some(data),
            error: None,
        };

        Response { status, body }
    }
}

// Responses --------------------------------------------------------------------------------------

#[derive(Serialize)]
pub struct CreateUser {
    pub user: User
}

#[derive(Serialize)]
pub struct VerifyUser {
    pub token: String
}

use std::ops::Deref;

// use crate::api::models::dto;
use serde::Serialize;
use serde_with;
use axum::{
    http::StatusCode,
    Json, response::IntoResponse
};

use crate::user_service::error::Error;

pub struct Status(StatusCode);

#[derive(Serialize)]
pub struct Body<T: Serialize> {
    pub data: Option<T>,
    pub error: Option<String>
}

pub struct Response<T: Serialize> {
    pub body: Body<T>,
    pub status: StatusCode,
    pub headers: Vec<(String, String)>
}

impl<T: Serialize> From<Error> for Response<T> {
    fn from(err: Error) -> Self {
        let (status, message) = parse_error(err);
        let body = Body {
            data: None,
            error: Some(message), 
        };

        let res = Response {
            body: body,
            status: status,
            headers: vec![]
        };

        res
    }
}

fn parse_error(err: Error) -> (StatusCode, String) {
        let (status_code, message) = match err {
        Error::Forbidden => {
            (StatusCode::FORBIDDEN, err.to_string())
        },
        Error::Unauthorized => {
            (StatusCode::UNAUTHORIZED, err.to_string())
        },
        Error::NotFound => {
            (StatusCode::NOT_FOUND, err.to_string())
        },
        _ => {
            (StatusCode::INTERNAL_SERVER_ERROR, String::from("Something went wrong, please try again later"))
        }};

        (status_code, message)
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        let body = self.body;
        let status = self.status;

        (status, Json(body)).into_response()
    }
}

impl<T: Serialize> Response<T> {
    pub fn ok(data: T) -> Self {
        let headers = vec![];
        let body = Body{
            data: Some(data),
            error: None,
        };

        let resp = Response {
            status: StatusCode::OK,
            body: body,
            headers: headers
        };

        resp
    }
}

// #[serde_with::skip_serializing_none]
// #[derive(Serialize)]
// pub struct Response<T> {
//     status: u16,
//     data: Option<T>,
//     error: Option<String>
// }

// impl<T: Serialize> Response<T> {
//     pub fn ok(data: T) -> impl IntoResponse {
//         let status_code = StatusCode::OK;
//         let response = Response {
//             status: status_code.as_u16(),
//             data: Some(data),
//             error: None
//         };

//         (status_code, Json(response)).into_response()
//     }
    
//     pub fn error(err: Error) -> Response<T> {
//         let (status, message) = match err {
//             Error::Forbidden => {
//                 (StatusCode::FORBIDDEN, err.to_string())
//             },
//             Error::Unauthorized => {
//                 (StatusCode::UNAUTHORIZED, err.to_string())
//             },
//             Error::NotFound => {
//                 (StatusCode::NOT_FOUND, err.to_string())
//             },
//             _ => {
//                 (StatusCode::INTERNAL_SERVER_ERROR, String::from("Something went wrong, please try again later"))
//             }
//         };
        
//         Response { 
//             status:  status.as_u16(),
//             data: None,
//             error: Some(message)
//         }
//     }
// }

// impl<T> From<Error> for Response<T> {
//     fn from(err: Error) -> Self {
//         Response::error(err)
//     }
// }

// #[derive(Serialize)]
// pub struct Response<T> {
//     status: u16,
//     data: T
// }

// impl<T> ServerResponse<T> {
//     fn new()
// }

// // pub fn ok<T: Serialize>(data: T) -> axum::response::Response {
// //     let status = StatusCode::OK;
// //     let response = OkResponse {
// //         status: status.as_u16(),
// //         data: data,
// //     };

// //     (status, Json(response)).into_response()
// // }

// impl<T: Serialize> IntoResponse for ServerResponse<T> {
//     fn into_response(self) -> axum::response::Response {
//         let status_code = StatusCode::OK;

//         (status_code, Json(self)).into_response()
//     }
// }

// #[derive(Serialize)]
// pub struct ErrResponse {
//     status: u16,
//     error: String
// }

// impl IntoResponse for Error {
//     fn into_response(self) -> axum::response::Response {
//         let (status, message) = match self {
//             Error::Forbidden => {
//                 (StatusCode::FORBIDDEN, self.to_string())
//             },
//             Error::Unauthorized => {
//                 (StatusCode::UNAUTHORIZED, self.to_string())
//             },
//             Error::NotFound => {
//                 (StatusCode::NOT_FOUND, self.to_string())
//             },
//             _ => {
//                 (StatusCode::INTERNAL_SERVER_ERROR, String::from("Something went wrong, please try again later"))
//             }
//         };
        
//         let response = ErrResponse { 
//             status:  status.as_u16(),
//             error: message,
//         };

//         (status, Json(response)).into_response()
//     }
// }

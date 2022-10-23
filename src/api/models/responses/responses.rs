// use crate::api::models::dto;
// use serde::Serialize;
// use serde_with;
// use axum::http::StatusCode;

// #[serde_with::skip_serializing_none]
// #[derive(Serialize)]
// pub struct Response<T> {
//     status: u16,
//     data: Option<T>,
//     error: Option<String>
// }

// pub fn ok<T: Serialize>(data: T) -> Response<T> {
//     Response {
//         status: StatusCode::OK.as_u16(),
//         data: Some(data),
//         error: None
//     }
// }

// pub fn server_error<T: Serialize>(message: String) -> Response<T> {
//     Response { 
//         status:  StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
//         data: None,
//         error: Some(message)
//     }
// }

// // pub fn bad_request<T: Serialize>(message: String) -> Response<T> {
// //     Response { 
// //         status: StatusCode::BAD_REQUEST.as_u16(), 
// //         data: None,
// //         error: Some(message)
// //     }
// // }

// #[derive(Serialize)]
// pub struct CreateUserResponse {
//     pub user: dto::User
// }
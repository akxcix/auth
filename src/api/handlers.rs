use axum::response::IntoResponse;
use axum::http::StatusCode;
use crate::api::dtos::responses::Response;

use crate::api::server::Server;

impl Server {
    pub async fn healthcheck() -> impl IntoResponse {
        Response::ok("like a charm!")
    }

    pub async fn not_found() -> impl IntoResponse {
        let status = StatusCode::NOT_FOUND;
        let message = String::from("The requested resource was not found on the server");
        
        Response::error(status, message)
    }
}
use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiRespnse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    #[serde(skip)]
    pub status: StatusCode,
}

impl<T: Serialize> IntoResponse for ApiRespnse<T> {
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        (status, Json(self)).into_response()
    }
}

impl<T: Serialize> ApiRespnse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
            status: StatusCode::OK,
        }
    }

    pub fn created( message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: None,
            status: StatusCode::CREATED,
        }
    }
}
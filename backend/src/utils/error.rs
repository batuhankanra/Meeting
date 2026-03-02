use axum::{Json, extract::rejection::JsonRejection, http::StatusCode, response::{IntoResponse,Response}};
use serde_json::json;



pub enum AppError{
    BadRequest(String),
    NotFound(String),
    Internal(String),
    Unauthorized,
    Conflict(String),
    Forbiden(String),
    JsonError(String),
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Unauthorized access".to_string(),
            ),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::JsonError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Forbiden(msg)=>(StatusCode::FORBIDDEN,msg),
        };

        let body = Json(json!({
            "success": false,
            "error": error_message
        }));

        (status, body).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection:JsonRejection)->Self{
        match rejection {
            JsonRejection::JsonDataError(_) => {
                AppError::BadRequest(format!("Email and password cannot be empty."))
            }
            JsonRejection::JsonSyntaxError(_) => {
                AppError::BadRequest(format!("error" ))
            }
            JsonRejection::MissingJsonContentType(_) => {
                AppError::BadRequest("Content-Type must be application/json".to_string())
            }
            _ => AppError::BadRequest("Invalid request body".to_string()),
          
        }
    }
}
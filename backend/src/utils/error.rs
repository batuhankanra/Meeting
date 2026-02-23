use axum::{Json, http::{ StatusCode}, response::{IntoResponse,Response}};
use serde_json::json;



pub enum AppError{
    BadRequest(String),
    NotFound(String),
    Internal(String),
    Unauthorized,
    Conflict(String)
}
impl IntoResponse for AppError {
    fn into_response(self)->Response{
        let (status,error_message)=match self {
            AppError::BadRequest(msg)=>(StatusCode::BAD_REQUEST,msg),
            AppError::NotFound(msg)=>(StatusCode::NOT_FOUND,msg),
            AppError::Internal(msg)=>(StatusCode::INTERNAL_SERVER_ERROR,msg),
            AppError::Unauthorized=>(StatusCode::UNAUTHORIZED,"Unauthorized access".to_string()),
            AppError::Conflict(msg)=>(StatusCode::CONFLICT,msg)
        };
        let body =Json(json!({
            "success":false,
            "error":error_message
        }));
        (status,body).into_response()
    }
}
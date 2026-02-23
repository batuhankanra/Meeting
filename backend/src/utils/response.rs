use serde::Serialize;
use axum::{Json,http::StatusCode};

#[derive(Serialize)]
pub struct ApiRespnse<T>{
    pub success:bool,
    pub message:String,
    pub data:Option<T>
}

impl <T> ApiRespnse<T> where T:Serialize {
    pub fn success(data:T,message:&str)->(StatusCode,Json<Self>){
        (
            StatusCode::OK,
            Json(ApiRespnse { success: true, message: message.to_string(), data: Some(data) })
        )
    }
    pub fn created(data:T,message:&str)->(StatusCode,Json<Self>){
        (
            StatusCode::CREATED,
            Json(ApiRespnse { success: true, message: message.to_string(), data: Some(data) })
        )
    }
}
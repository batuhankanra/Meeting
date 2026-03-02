use axum::{body::Body, extract::Request, http::header, middleware::Next, response::Response};

use crate::utils::{error::AppError, jwt::verify_token};



pub async fn jwt_auth_middleware(mut req:Request<Body>,next:Next)->Result<Response,AppError>{

    let auth_header=req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;
    
    if !auth_header.starts_with("Bearer "){
        return Err(AppError::Unauthorized);
    }
    let token =&auth_header[7..]; 
    let claims=verify_token(token).map_err(|_| AppError::Unauthorized)?;
    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}
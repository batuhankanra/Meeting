use axum::{body::Body, http::Request, middleware::Next, response::Response};

use crate::{models::user::UserRole, utils::{error::AppError, jwt::Claims}};




pub async fn role(req:Request<Body>,next:Next,req_role:UserRole)->Result<Response,AppError>{
    let claims=req.extensions().get::<Claims>().ok_or(AppError::Unauthorized)?;


    if claims.role !=req_role{
        return Err(AppError::Forbiden("You do not have permission to perform this action.".to_string()));
    }
    Ok(next.run(req).await)
}
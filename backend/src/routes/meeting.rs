use axum::{Router, middleware, routing::get};

use crate::{handlers::all_services::all_services, middleware::{auth_check::jwt_auth_middleware, role_middleware}, models::user::UserRole, state::AppState};




pub fn meeting_route()->Router<AppState>{
    let meeting_public=Router::new()
        .route("/all", get(all_services));


    let meeting_privete=Router::new()
        .route("/create_meeting", get(all_services))
        .layer(middleware::from_fn(| req,next| role_middleware::role(req,next,UserRole::Admin)));

    Router::new()
        .merge(meeting_privete)
        .merge(meeting_public)
        .layer(middleware::from_fn(jwt_auth_middleware))
    

}
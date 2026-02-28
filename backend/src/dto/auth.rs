use serde::Deserialize;
use validator::Validate;


#[derive(Deserialize,Validate)]
pub struct AuthRequest{
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 6, message = "The password must be at least 6 characters long."))]
    pub password: String,
}



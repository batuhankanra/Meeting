use axum::{
    extract::{State, Json, rejection::JsonRejection},
    http::StatusCode,
};
use bcrypt::verify;
use validator::Validate;

use crate::{
    db::query::UserRepository,
    dto::auth::AuthRequest,
    state::AppState,
    utils::{error::AppError, jwt::create_token, response::ApiRespnse},
};

// DİKKAT 1: Dönüş tipindeki 'User'ı 'String' yaptık çünkü data olarak token döneceğiz.
// DİKKAT 2: Axum standartlarına uyması için Json() içine aldık.
pub async fn login_handler(
    State(state): State<AppState>,
    payload: Result<Json<AuthRequest>, JsonRejection>,
) -> Result<Json<ApiRespnse<String>>, AppError> {

    // 1. JSON ayrıştırma hatasını güvenle yakala
    let Json(payload) = payload.map_err(|_| AppError::BadRequest("Geçersiz JSON formatı".to_string()))?;

    // 2. Doğrulama (Validation)
    payload.validate().map_err(|e| {
        let error_msg = e.field_errors()
            .iter()
            .next() // Sadece ilk hatayı al
            .and_then(|(field, errors)| {
                errors[0].message.as_ref()
                    .map(|m| m.to_string())
                    .or_else(|| Some(format!("{} alanı geçersiz", field)))
            })
            .unwrap_or_else(|| "Giriş bilgileri hatalı".to_string());
            
        AppError::BadRequest(error_msg)
    })?;

    // 3. Kullanıcıyı Bul (BURASI HAYAT KURTARAN KISIM)
    // ? operatörü sayesinde unwrap() kullanmana gerek kalmaz. 
    // Hata varsa AppError::Unauthorized fırlatır, yoksa 'user' değişkenine veriyi güvenle atar.
    let user = UserRepository::find_by_email(&state.db, &payload.email)
        .await
        .map_err(|_| AppError::BadRequest("Email or password is incorrect".to_string()))?
        .ok_or(AppError::BadRequest("Email or password is incorrect".to_string()))?;

    let is_valid = verify(&payload.password, &user.password_hash)
        .map_err(|_| AppError::Internal("Şifre kontrolünde sistem hatası".to_string()))?;

    if !is_valid {
        return Err(AppError::BadRequest("Email or password is incorrect".to_string()));
    }

    
    let token = create_token(user.id, user.role)
        .map_err(|_| AppError::Internal("Token üretilemedi".to_string()))?;

    // 6. Yanıtı Döndür
    // ApiRespnse<String> olarak tanımladığımız için data kısmına String olan 'token'ı verebiliriz.
    let response = ApiRespnse {
        success: true,
        message: "Login successfully".to_string(),
        data: Some(token),
        status: StatusCode::OK,
    };

    Ok(Json(response))
}
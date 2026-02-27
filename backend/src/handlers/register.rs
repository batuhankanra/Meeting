use axum::{Json, extract::{State, rejection::JsonRejection}, };
use bcrypt::{DEFAULT_COST, hash};
use validator::Validate;

use crate::{db::query::UserRepository, dto::auth::RegisterRequest, models::user::User, state::AppState, utils::{error::AppError, response::ApiRespnse}};



pub async fn register_handler(
    State(state): State<AppState>,
    payload: Result<Json<RegisterRequest>, JsonRejection>,
) -> Result<ApiRespnse<User>, AppError> {
    let Json(payload)=payload?;
    payload.validate().map_err(|e| {
    // Tüm hataları birleştirip tek bir metin yapmak için:
    let error_messages: Vec<String> = e
        .field_errors()
        .iter()
        .map(|(field, errors)| {
            let msg = errors[0].message.as_ref()
                .map(|m| m.to_string())
                .unwrap_or_else(|| format!("{} alanı geçersiz", field));
            msg
        })
        .collect();
    AppError::BadRequest(error_messages.first().cloned().unwrap_or_default())
})?;

    let hashed_password = hash(payload.password, DEFAULT_COST)
        .map_err(|_| AppError::Internal("Password hash error".to_string()))?;

    UserRepository::create(
        &state.db,
        &payload.email,
        &hashed_password,
    )
    .await
    .map_err(|e| {
        if e.to_string().contains("unique") {
            AppError::Conflict("Email or username already exists".to_string())
        } else {
            AppError::Internal(e.to_string())
        }
    })?;

    Ok(ApiRespnse::created( "Kayıt başarıyla oluşturuldu"))
}
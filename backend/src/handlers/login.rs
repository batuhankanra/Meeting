use crate::{db::query::UserRepository,  models::user::User, state::AppState, utils::{error::AppError, response::ApiRespnse}};
use axum::extract::{State};


pub async fn login_handler(
    State(state): State<AppState>
) -> Result<ApiRespnse<Vec<User>>, AppError> {
    let users=UserRepository::userl_all(&state.db)
    .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(ApiRespnse::success(users, "ss"))
}
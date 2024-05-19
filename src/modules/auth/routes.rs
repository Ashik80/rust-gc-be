use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use crate::{errors::{Error, GeneralError}, models::user::{Role, User, UserDto}, AppState};

use super::jwt_service::JwtService;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/signup", post(register))
}

#[derive(Serialize)]
struct AuthResponse {
    access_token: String
}

#[derive(Deserialize)]
struct RegisterData {
    email: String,
    password: String
}

async fn login(
    State(state): State<AppState>,
    Json(data): Json<RegisterData>
) -> Result<Json<AuthResponse>, Error> {
    let user = User::get_by_email(&state, &data.email).await?;

    let is_match = bcrypt::verify(&data.password, &user.password)
        .map_err(|_| Error::GeneralError(GeneralError::Unhandled("Failed to verify password".to_string())))?;

    if !is_match {
        return Err(Error::GeneralError(GeneralError::Unauthorized));
    }

    let token = JwtService::generate(user.id, user.email)
        .map_err(|_| Error::GeneralError(GeneralError::Unhandled("Failed to generate token".to_string())))?;

    Ok(Json(AuthResponse { access_token: token }))
}

async fn register(
    State(state): State<AppState>,
    Json(data): Json<RegisterData>
) -> Result<Json<AuthResponse>, Error> {
    if User::get_by_email(&state, &data.email).await.is_ok() {
        return Err(Error::GeneralError(GeneralError::AlreadyExists));
    }

    let hashed_password = bcrypt::hash(data.password, 10).unwrap();

    let user = User::create_user(&state, UserDto {
        email: data.email.clone(),
        password: hashed_password,
        role: Role::Customer,
    }).await?;

    let token = JwtService::generate(user.id, data.email)
        .map_err(|_| Error::GeneralError(GeneralError::Unhandled("Failed to generate token".to_string())))?;

    Ok(Json(AuthResponse { access_token: token }))
}

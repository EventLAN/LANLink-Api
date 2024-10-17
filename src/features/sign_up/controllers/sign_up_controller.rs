use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::{post};
use deadpool_diesel::postgres::Pool;
use crate::core::crypto::crypto_manager::generate_hash;
use crate::core::errors::error::AppError;
use crate::core::errors::error_handler::throw_error;
use crate::features::auth::models::auth_user_model::AuthUserModel;
use crate::features::auth::repos::auth_repo::create_or_update_auth_user;
use crate::features::user::models::user_model::UserModel;
use crate::features::user::repos::user_repo;

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/api/v1/sign_up/username", post(username_sign_up))

}

#[derive(serde::Deserialize)]
struct UserSignUpDto {
    username: String,
    password: String,
}

#[axum::debug_handler]
async fn username_sign_up(
    State(_pool): State<Pool>,
    Json(_user_name_sign_up): Json<UserSignUpDto>,
) -> impl IntoResponse {
    let mut user_model = UserModel::new();
    user_model.username = _user_name_sign_up.username.clone();

    let model = user_repo::create_or_update_user(&_pool, user_model).await;

    match model {
        None => {
            throw_error(AppError::InternalServerError("Could not create user".parse().unwrap()));
            return StatusCode::INTERNAL_SERVER_ERROR
        },
        _ => {}
    }

    let password_hash_result = generate_hash(& _user_name_sign_up.password).await;

    match password_hash_result {
        Err(_) => {
            throw_error(AppError::InternalServerError("Could not generate hash".parse().unwrap()));
            return StatusCode::INTERNAL_SERVER_ERROR
        },
        _ => {}
    }

    let new_auth_model = AuthUserModel {
        id: 0,
        user_id: model.unwrap().id,
        username: _user_name_sign_up.username.clone(),
        password_hash: password_hash_result.unwrap(),
        created_at: Default::default(),
        updated_at: Default::default(),
    };
    let auth_model = create_or_update_auth_user(&_pool, new_auth_model).await;

    match auth_model {
        None => {
            throw_error(AppError::InternalServerError("Could not create auth user".parse().unwrap()));
            StatusCode::INTERNAL_SERVER_ERROR
        },
        Some(_) => StatusCode::OK
    }
}
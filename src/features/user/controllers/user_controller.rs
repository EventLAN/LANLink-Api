use axum::{extract::Path, response::Json, routing::post, Router};
use axum::extract::{State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use deadpool_diesel::postgres::Pool;
use diesel::dsl::all;
use serde::Deserialize;
use uuid::Uuid;
use crate::core::errors::user_errors::UserError;
use crate::features::auth::key_creation_and_retrieval::claims::Claims;
use crate::features::user::repos::user_repo::{get_user_by_discord, get_user_by_public_id, get_user_by_steam, get_user_by_username};
use crate::features::user::models::user_model::UserModel;
use crate::features::user::repos::user_repo;
use crate::features::user::dtos::user_dto::UserDto;

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/api/v1/user/create/:username", post(create_user))
        .route("/api/v1/user/edit", post(edit_user))
        .route("/api/v1/user/get/:get_type/:value", post(get_user))
}

async fn handle_get_by_public_id(_pool: Pool, public_id: Uuid) -> Result<UserDto, UserError> {
    let option = get_user_by_public_id(&_pool, public_id).await;
    if option.is_some(){
        Ok(UserDto::from_model(&option.unwrap()))
    }else {
        Err(UserError::UserNotFound)
    }
}

async fn handle_get_by_username(_pool: Pool, username: String) -> Result<UserDto, UserError> {
    let option = get_user_by_username(&_pool, username).await;
    if option.is_some(){
        Ok(UserDto::from_model(&option.unwrap()))
    }else {
        Err(UserError::UserNotFound)
    }
}



async fn handle_get_by_discord_username(_pool: Pool, discord_username: String) -> Result<UserDto, UserError> {
    let option = get_user_by_discord(&_pool, discord_username).await;
    if option.is_some(){
        Ok(UserDto::from_model(&option.unwrap()))
    }else {
        Err(UserError::UserNotFound)
    }
}

async fn handle_get_by_steam_url(_pool: Pool, steam_url: String) -> Result<UserDto, UserError> {
    let option = get_user_by_steam(&_pool, steam_url).await;
    if option.is_some(){
        Ok(UserDto::from_model(&option.unwrap()))
    }else {
        Err(UserError::UserNotFound)
    }
}
async fn get_user(State(_pool): State<Pool>, Path((get_type, value)): Path<(String, String)>) -> Result<Json<UserDto>, UserError> {
    if get_type.is_empty() || value.is_empty() {
        return Err(UserError::MissingField);
    }

    if get_type == "username" {
        let result = handle_get_by_username(_pool, value).await;
        if result.is_ok(){
            let user = result?;
            return Ok(Json(user));
        }else{
            return Ok(Json(UserDto::from_model(&UserModel::new())))
        }
    }

    if get_type == "public_id" {
        let id_result = Uuid::parse_str(&value);
        if id_result.is_ok(){
            let result = handle_get_by_public_id(_pool, id_result.unwrap()).await;
            if result.is_ok(){
                let user = result?;
                return Ok(Json(user));
            }else{
                return Ok(Json(UserDto::from_model(&UserModel::new())))
            }
        }

    }

    if get_type == "discord_username" {
        let result = handle_get_by_discord_username(_pool, value).await;
        if result.is_ok(){
            let user = result?;
            return Ok(Json(user));
        }else{
            return Ok(Json(UserDto::from_model(&UserModel::new())))
        }
    }

    if get_type == "steam_url" {
        let result = handle_get_by_steam_url(_pool, value).await;
        if result.is_ok(){
            let user = result?;
            return Ok(Json(user));
        }else{
            return Ok(Json(UserDto::from_model(&UserModel::new())))
        }
    }
    Ok(Json(UserDto::from_model(&UserModel::new())))
}
#[derive(Debug, Deserialize)]
struct EditUserPayload {
    new_username: String,
    new_first_name: String,
    new_last_name: String,
    new_discord_username: String,
    new_steam_url: String,
}


async fn edit_user(State(_pool): State<Pool>, claims: Claims, Json(payload): Json<EditUserPayload>) -> Result<impl IntoResponse, UserError> {
    if payload.new_username.is_empty()
        || payload.new_first_name.is_empty()
        || payload.new_last_name.is_empty()
        || payload.new_discord_username.is_empty()
        || payload.new_steam_url.is_empty(){
        return Err(UserError::MissingField);
    }

    let mut user = get_user_by_public_id(&_pool, claims.user_public_id).await
        .ok_or(UserError::UserNotFound)?;

    let (username_check, discord_check, steam_check) = tokio::join!(
        get_user_by_username(&_pool, payload.new_username.clone()),
        get_user_by_discord(&_pool, payload.new_discord_username.clone()),
        get_user_by_steam(&_pool, payload.new_steam_url.clone())
    );

    for allowed_test in [username_check, discord_check, steam_check]{
        if let Some(test_user) = allowed_test{
            if test_user.publicid != user.publicid{
                return Err(UserError::UserMismatch);
            }
        }
    }

    user.username = payload.new_username;
    user.first_name = payload.new_first_name;
    user.last_name = payload.new_last_name;
    user.discord_username = payload.new_discord_username;
    user.steam_url = payload.new_steam_url;

    if user_repo::create_or_update_user(&_pool, user).await.is_none(){
        return Err(UserError::UserUpdateFailed);
    }

    Ok(Json("User updated").into_response())
}



#[axum::debug_handler]
async fn create_user(
    State(_pool): State<Pool>, Path(username): Path<String>) -> impl IntoResponse {

    let mut user = get_user_by_username(&_pool, username.clone()).await;
    if user.is_some(){
        return (StatusCode::UNAUTHORIZED, "User already exists").into_response();
    }

    let mut user = UserModel::new();
    user.username = username.clone();

    let model = user_repo::create_or_update_user(&_pool, user).await;

    (StatusCode::OK, Json(model)).into_response()
}
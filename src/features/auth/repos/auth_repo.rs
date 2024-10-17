use diesel::{ExpressionMethods, QueryDsl};
use deadpool_diesel::postgres::Pool;
use diesel::associations::HasTable;
use diesel::{RunQueryDsl, SelectableHelper};
use crate::core::db_connection::db_connection::create_connection;
use crate::core::errors::error::AppError;
use crate::features::auth::models::auth_user_model::AuthUserModel;
use crate::schema::auth_users::dsl::*;

pub async fn get_auth_user_by_username(
    pool: &Pool,
    request_username: &String,
) -> Option<AuthUserModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let match_username = request_username.clone();
    let result = conn.interact(move |c| {
        auth_users
            .filter(username.eq(match_username))
            .select(AuthUserModel::as_select())
            .first(c)
    })
    .await
    .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(user) => {
            match user {
                Ok(user) => Some(user),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

pub async fn create_or_update_auth_user(
    pool: &Pool,
    auth_model: AuthUserModel,
) -> Option<AuthUserModel> {
    if auth_model.id == 0 {
        create_auth_user(pool, auth_model).await
    } else {
        update_auth_user(pool, auth_model).await
    }
}

async fn create_auth_user(
    pool: &Pool,
    auth_model: AuthUserModel,
) -> Option<AuthUserModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(move |c| {
        diesel::insert_into(auth_users::table())
            .values(auth_model.create_new_auth_user_for_db())
            .returning(AuthUserModel::as_returning())
            .get_result(c)
    })
        .await
        .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(user) => {
            match user {
                Ok(user) => Some(user),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

async fn update_auth_user(
    pool: &Pool,
    auth_model: AuthUserModel,
) -> Option<AuthUserModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(move |c| {
        diesel::update(auth_users::table())
            .filter(id.eq(auth_model.id))
            .set(auth_model.create_update_auth_user_for_db())
            .returning(AuthUserModel::as_returning())
            .get_result(c)
    })
        .await
        .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(user) => {
            match user {
                Ok(user) => Some(user),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}
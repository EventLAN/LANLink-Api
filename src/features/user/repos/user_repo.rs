use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use diesel::associations::HasTable;
use diesel::{QueryDsl, SelectableHelper};
use uuid::Uuid;

// internal uses
use crate::core::db_connection::db_connection::create_connection;
use crate::core::errors::error::AppError;
use crate::features::user::models::user_model::UserModel;
use crate::schema::users::dsl::*;

pub async fn get_all_users(
    pool: &Pool,
) -> Option<Vec<UserModel>> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(|c| {
        users
            .select(UserModel::as_select())
            .load::<UserModel>(c)
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

pub async fn get_user_by_id(
    pool: &Pool,
    request_id: i32,
) -> Option<UserModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(move |c| {
        users::table()
            .find(request_id)
            .select(UserModel::as_select())
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



pub async fn get_user_by_public_id(
    pool: &Pool,
    public_id: Uuid,
) -> Option<UserModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(move |c| {
        users::table()
            .filter(publicid.eq(public_id))
            .select(UserModel::as_select())
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

pub async fn get_user_by_username(
    pool: &Pool,
    req_username: String,
) -> Option<UserModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(move |c| {
        users::table()
            .filter(username.eq(req_username))
            .select(UserModel::as_select())
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

pub async fn get_user_by_discord(
    pool: &Pool,
    req_discord: String,
) -> Option<UserModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(move |c| {
        users::table()
            .filter(discord_username.eq(req_discord))
            .select(UserModel::as_select())
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

pub async fn get_user_by_steam(
    pool: &Pool,
    req_steam: String,
) -> Option<UserModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(move |c| {
        users::table()
            .filter(steam_url.eq(req_steam))
            .select(UserModel::as_select())
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

pub async fn create_or_update_user(
    pool: &Pool,
    user_model: UserModel,
) -> Option<UserModel> {
    if user_model.id == 0 {
        return create_user(pool, user_model).await
    } else {
        return update_user(pool, user_model).await
    }
}

async fn create_user(
    pool: &Pool,
    user_model: UserModel,
) -> Option<UserModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };
    
    let result = conn.interact(move |c| {
        diesel::insert_into(users::table())
            .values(user_model.create_new_user_for_db())
            .returning(UserModel::as_returning())
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

async fn update_user(
    pool: &Pool,
    user_model: UserModel,
) -> Option<UserModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };
    
    let result = conn.interact(move |c| {
        diesel::update(users::table())
            .filter(id.eq(user_model.id))
            .set(user_model.create_update_user_for_db())
            .returning(UserModel::as_returning())
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

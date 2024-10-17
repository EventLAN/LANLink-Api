use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::auth_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuthUserModel{
    pub id: i32,
    pub user_id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::auth_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAuthUserModel{
    pub user_id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::auth_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateAuthUserModel{
    pub id: i32,
    pub user_id: i32,
    pub username: String,
    pub password_hash: String,
    pub updated_at: chrono::NaiveDateTime,
}

impl AuthUserModel{
    pub fn new() -> AuthUserModel{
        AuthUserModel{
            id: 0,
            user_id: 0,
            username: String::new(),
            password_hash: String::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }

    pub fn create_new_auth_user_for_db(&self) -> NewAuthUserModel {
        NewAuthUserModel{
            user_id: self.user_id.clone(),
            username: self.username.clone(),
            password_hash: self.password_hash.clone(),
        }
    }
    pub fn create_update_auth_user_for_db(&self) -> UpdateAuthUserModel {
        UpdateAuthUserModel {
            id: self.id.clone(),
            user_id: self.user_id.clone(),
            username: self.username.clone(),
            password_hash: self.password_hash.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}
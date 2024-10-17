use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserModel{
    pub id: i32,
    pub publicid: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub discord_username: String,
    pub steam_url: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUserModel{
    pub publicid: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub discord_username: String,
    pub steam_url: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateUserModel{
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub discord_username: String,
    pub steam_url: String,
}

impl UserModel{
    pub fn new() -> UserModel{
        UserModel{
            id: 0,
            publicid: Uuid::new_v4(),
            username: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            discord_username: String::new(),
            steam_url: String::new(),
        }
    }
    pub fn is_empty(&self) -> bool{
        return if self.username == String::new()
            && self.id == 0
            && self.first_name == String::new()
            && self.last_name == String::new()
            && self.discord_username == String::new()
            && self.steam_url == String::new() {
            true
        } else {
            false
        }
    }

    pub fn create_new_user_for_db(&self) -> NewUserModel {
        NewUserModel{
            publicid: Uuid::new_v4(),
            username: self.username.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            discord_username: self.discord_username.clone(),
            steam_url: self.steam_url.clone()
        }
    }
    
    pub fn create_update_user_for_db(&self) -> UpdateUserModel {
        // we are making sure not to delete the user public ID as it should be static
        UpdateUserModel{
            id: self.id.clone(),
            username: self.username.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            discord_username: self.discord_username.clone(),
            steam_url: self.steam_url.clone(),
        }
    }
}
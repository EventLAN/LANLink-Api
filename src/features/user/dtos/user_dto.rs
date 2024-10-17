use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::features::user::models::user_model::UserModel;

#[derive(Deserialize, Serialize)]
pub struct UserDto {
    pub id: i32,
    pub publicid: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub discord_username: String,
    pub steam_url: String,
}

impl UserDto {
    pub fn from_model(model: &UserModel) -> Self {
        UserDto {
            id: model.id.into(),
            publicid: model.publicid.into(),
            username: model.username.clone(),
            first_name: model.first_name.clone(),
            last_name: model.last_name.clone(),
            discord_username: model.discord_username.clone(),
            steam_url: model.steam_url.clone(),
        }
    }
}

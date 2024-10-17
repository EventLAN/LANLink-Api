use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::post;
use deadpool_diesel::postgres::Pool;
use crate::features::auth::key_creation_and_retrieval::claims::Claims;
use crate::features::user::repos::user_repo;

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/api/v1/protected", post(protected))
        .route("/api/v1/protected/test", post(test))
}

use crate::features::auth::key_creation_and_retrieval::permission_manager;
use crate::features::auth::key_creation_and_retrieval::permission_manager::FeaturePermissions;
use crate::features::auth::key_creation_and_retrieval::permissions::admin_permissions::*;
use crate::features::auth::key_creation_and_retrieval::permissions::user_permissions::*;
use crate::features::auth::key_creation_and_retrieval::permissions::equipment_permissions::*;

async fn test(State(_pool): State<Pool>, claims: Claims) -> impl IntoResponse {

    let mut perm = permission_manager::PermissionsManager{
        admin_permissions: FeaturePermissions {bits: 0},
        user_permissions: FeaturePermissions { bits: 0 },
        equipment_permissions: FeaturePermissions { bits: 0 },
    };

    perm.admin_permissions.set_permission(ADMIN_READ_PERMISSION);
    perm.user_permissions.set_permission(USER_FULL_ACCESS);
    perm.equipment_permissions.set_permission(USER_FULL_ACCESS);

    let mut output = String::new();

    if perm.user_permissions.has_permission(USER_READ_PERMISSION){
        output.push_str("You have user read permission\n");
    }else{
        output.push_str("You do not have user read permission\n");
    }
    if perm.user_permissions.has_permission(USER_WRITE_PERMISSION){
        output.push_str("You have user write permission\n");
    }else{
        output.push_str("You do not have user read permission\n");
    }
    if perm.user_permissions.has_permission(USER_EXECUTE_PERMISSION){
        output.push_str("You have user execute permission\n");
    }else{
        output.push_str("You do not have user read permission\n");
    }
    if perm.user_permissions.has_permission(USER_DELETE_PERMISSION){
        output.push_str("You have user delete permission\n");
    }else{
        output.push_str("You do not have user read permission\n");
    }

    output.push_str("\n");

    if perm.admin_permissions.has_permission(ADMIN_READ_PERMISSION){
        output.push_str("You have admin read permission\n");
    }else{
        output.push_str("You do not have admin read permission\n");
    }
    if perm.admin_permissions.has_permission(ADMIN_WRITE_PERMISSION){
        output.push_str("You have admin write permission\n");
    }else{
        output.push_str("You do not have admin read permission\n");
    }
    if perm.admin_permissions.has_permission(ADMIN_EXECUTE_PERMISSION){
        output.push_str("You have admin execute permission\n");
    }else{
        output.push_str("You do not have admin read permission\n");
    }
    if perm.admin_permissions.has_permission(ADMIN_DELETE_PERMISSION){
        output.push_str("You have admin delete permission\n");
    }else{
        output.push_str("You do not have admin read permission\n");
    }

    output.push_str("\n");

    if perm.equipment_permissions.has_permission(EQUIP_READ_PERMISSION){
        output.push_str("You have equip read permission\n");
    }else{
        output.push_str("You do not have equip read permission\n");
    }
    if perm.equipment_permissions.has_permission(EQUIP_WRITE_PERMISSION){
        output.push_str("You have equip write permission\n");
    }else{
        output.push_str("You do not have equip read permission\n");
    }
    if perm.equipment_permissions.has_permission(EQUIP_EXECUTE_PERMISSION){
        output.push_str("You have equip execute permission\n");
    }else{
        output.push_str("You do not have equip read permission\n");
    }
    if perm.equipment_permissions.has_permission(EQUIP_DELETE_PERMISSION){
        output.push_str("You have equip delete permission\n");
    }else{
        output.push_str("You do not have equip read permission\n");
    }
    output.push_str(format!("\nYour permission string is: {}", perm.to_permissions_bitwise()).as_str());

    (StatusCode::OK, output).into_response()
}

#[axum::debug_handler]
async fn protected(State(_pool): State<Pool>, claims: Claims) -> impl IntoResponse {
    
    print!("Claims: {:?}", claims);
    let full_user = user_repo::get_user_by_public_id(&_pool, claims.user_public_id).await;
    match full_user {
        Some(user) => {
            // Send the protected data to the user
            (StatusCode::OK,
             format!("Welcome to the protected area, \n Your name is: {} \n Your discord is: {} \n Your steam is: {}",
                     format!("{} {}", user.first_name, user.last_name),
                     user.discord_username,
                     user.steam_url)
            ).into_response()
        }
        None => {
            return (StatusCode::NO_CONTENT, "User not found").into_response();
        }
    }
}

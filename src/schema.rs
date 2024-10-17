// @generated automatically by Diesel CLI.

diesel::table! {
    auth_users (id) {
        id -> Int4,
        user_id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    equipments (id) {
        id -> Int4,
        publicid -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        publicid -> Uuid,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        discord_username -> Varchar,
        steam_url -> Varchar,
    }
}

diesel::joinable!(auth_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_users,
    equipments,
    users,
);

use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use diesel::associations::HasTable;
use diesel::{QueryDsl, SelectableHelper};
use uuid::Uuid;


// internal uses
use crate::core::db_connection::db_connection::create_connection;
use crate::core::errors::error::AppError;
use crate::features::equipment::models::equipment_model::EquipmentModel;
use crate::schema::equipments::dsl::*;

pub async fn get_all_equipment(
    pool: &Pool,
) -> Option<Vec<EquipmentModel>> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(|c| {
        equipments
            .select(EquipmentModel::as_select())
            .load::<EquipmentModel>(c)
    })
    .await
    .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(equipment) => {
            match equipment {
                Ok(equipment) => Some(equipment),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

pub async fn get_equipment_by_id(
    pool: &Pool,
    request_id: i32,
) -> Option<EquipmentModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(move |c| {
        equipments::table()
            .find(request_id)
            .select(EquipmentModel::as_select())
            .first(c)
    })
    .await
    .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(equipment) => {
            match equipment {
                Ok(equipment) => Some(equipment),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

pub async fn get_equipment_by_public_id(
    pool: &Pool,
    public_id: Uuid,
) -> Option<EquipmentModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(move |c| {
        equipments::table()
            .filter(publicid.eq(public_id))
            .select(EquipmentModel::as_select())
            .first(c)
    })
    .await
    .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(equipment) => {
            match equipment {
                Ok(equipment) => Some(equipment),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}


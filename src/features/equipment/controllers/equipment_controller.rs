use axum::{extract::Path, response::Json, routing::post, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use deadpool_diesel::postgres::Pool;
use serde::Serialize;
use uuid::{Uuid};
use crate::features::equipment::dtos::equipment_dto::EquipmentDto;
use crate::features::equipment::repos::equipment_repo::*;

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/api/v1/equipment", get(get_all_equipment_handler))
        .route("/api/v1/equipment/:public_id", get(get_equipment_handler))
        .route("/api/v1/equipment/hire/:id", post(hire))
        .route("/api/v1/equipment/hire-by-name/:equipmentName", post(hire_by_name))
}

#[axum::debug_handler]
async fn get_all_equipment_handler(State(_pool): State<Pool>) -> impl IntoResponse {
    let equipment_models = get_all_equipment(&_pool);

    match equipment_models.await {
        Some(vec) => {
            let dtos: Vec<EquipmentDto> = vec.iter().map(|e| EquipmentDto::from_model(e)).collect();
            (StatusCode::OK, Json(dtos)).into_response()
        },
        None => (StatusCode::NO_CONTENT).into_response(),
    }
}

#[axum::debug_handler]
async fn get_equipment_handler(
    State(_pool): State<Pool>, Path(public_id): Path<Uuid>) -> impl IntoResponse {
    let equipment_model = get_equipment_by_public_id(&_pool, public_id);

    match equipment_model.await {
        Some(item) => {
            let dtos = EquipmentDto::from_model(&item);
            (StatusCode::OK, Json(dtos)).into_response()
        },
        None => (StatusCode::NO_CONTENT).into_response(),
    }
}
#[derive(Serialize)]
struct HireEquipment {
    pub id: i32,
    pub public_id: Uuid,
    pub equipment: EquipmentDto
}

async fn hire(Path(id): Path<i32>) -> (StatusCode, Json<HireEquipment>) {
    let rsp = HireEquipment {
        id: 1,
        public_id: Uuid::new_v4(),
        equipment: EquipmentDto{
            id,
            publicid: Uuid::new_v4(),
            name: "Big wire".parse().unwrap()
        }
    };
    (StatusCode::OK, Json(rsp))
}

async fn hire_by_name(Path(equipment_name): Path<String>) -> (StatusCode, Json<HireEquipment>) {
    let rsp = HireEquipment {
        id: 2,
        public_id: Uuid::new_v4(),
        equipment: EquipmentDto{
            id: 3,
            publicid: Uuid::new_v4(),
            name: equipment_name
        }
    };
    (StatusCode::OK, Json(rsp))
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::features::equipment::models::equipment_model::EquipmentModel;

#[derive(Deserialize, Serialize)]
pub struct EquipmentDto {
    pub id: i32,
    pub publicid: Uuid,
    pub name: String,
}

impl EquipmentDto {
    pub fn from_model(model: &EquipmentModel) -> Self {
        EquipmentDto {
            id: model.id.into(),
            publicid: model.publicid.into(),
            name: model.name.clone()
        }
    }
}

use diesel::prelude::*;
use uuid::Uuid;
use crate::features::equipment::dtos::equipment_dto::EquipmentDto;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::equipments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EquipmentModel {
    pub id: i32,
    pub publicid: Uuid,
    pub name: String,
}

impl EquipmentModel {
    pub fn from_dto(dto: &EquipmentDto) -> Result<Self, uuid::Error> {
        Ok(EquipmentModel {
            id: dto.id,
            publicid: dto.publicid,
            name: dto.name.clone(),
        })
    }
}

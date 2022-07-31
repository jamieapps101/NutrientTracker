//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "consumable"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub notes: String,
    pub portion_amount: f32,
    pub portion_unit: i32,
    pub nutrients: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Name,
    Notes,
    PortionAmount,
    PortionUnit,
    Nutrients,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Nutrients,
    Units,
    ConsumptionRecord,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Name => ColumnType::String(None).def(),
            Self::Notes => ColumnType::String(None).def(),
            Self::PortionAmount => ColumnType::Float.def(),
            Self::PortionUnit => ColumnType::Integer.def(),
            Self::Nutrients => ColumnType::Integer.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Nutrients => Entity::belongs_to(super::nutrients::Entity)
                .from(Column::Nutrients)
                .to(super::nutrients::Column::Id)
                .into(),
            Self::Units => Entity::belongs_to(super::units::Entity)
                .from(Column::PortionUnit)
                .to(super::units::Column::Id)
                .into(),
            Self::ConsumptionRecord => Entity::has_many(super::consumption_record::Entity).into(),
        }
    }
}

impl Related<super::nutrients::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Nutrients.def()
    }
}

impl Related<super::units::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Units.def()
    }
}

impl Related<super::consumption_record::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConsumptionRecord.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

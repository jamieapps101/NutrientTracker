//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "composite_consumable"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub notes: String,
    pub portion_amount: f32,
    pub portion_unit: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Name,
    Notes,
    PortionAmount,
    PortionUnit,
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
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Units => Entity::belongs_to(super::units::Entity)
                .from(Column::PortionUnit)
                .to(super::units::Column::Id)
                .into(),
            Self::ConsumptionRecord => Entity::has_many(super::consumption_record::Entity).into(),
        }
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

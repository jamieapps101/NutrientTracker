//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "users"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub password_hash: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Name,
    PasswordHash,
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
    NutrientTargets,
    ConsumptionRecord,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Name => ColumnType::String(None).def(),
            Self::PasswordHash => ColumnType::String(None).def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::NutrientTargets => Entity::has_many(super::nutrient_targets::Entity).into(),
            Self::ConsumptionRecord => Entity::has_many(super::consumption_record::Entity).into(),
        }
    }
}

impl Related<super::nutrient_targets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NutrientTargets.def()
    }
}

impl Related<super::consumption_record::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConsumptionRecord.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "composite_consumable_nutrients"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub composite_consumable_id: i32,
    pub consumable: Option<i32>,
    pub composite_consumable: Option<i32>,
    pub scaling: f32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    CompositeConsumableId,
    Consumable,
    CompositeConsumable,
    Scaling,
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
    Consumable2,
    Consumable1,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::CompositeConsumableId => ColumnType::Integer.def(),
            Self::Consumable => ColumnType::Integer.def().null(),
            Self::CompositeConsumable => ColumnType::Integer.def().null(),
            Self::Scaling => ColumnType::Float.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Consumable2 => Entity::belongs_to(super::consumable::Entity)
                .from(Column::CompositeConsumable)
                .to(super::consumable::Column::Id)
                .into(),
            Self::Consumable1 => Entity::belongs_to(super::consumable::Entity)
                .from(Column::Consumable)
                .to(super::consumable::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

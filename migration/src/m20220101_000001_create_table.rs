use sea_orm_migration::prelude::*;

#[cfg(feature = "derive")]
use sea_query::Iden;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserAccounts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserAccounts::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserAccounts::Name).string().not_null())
                    .col(ColumnDef::new(UserAccounts::Salt).string().not_null())
                    .col(
                        ColumnDef::new(UserAccounts::PasswordHash)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Sessions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Sessions::UserId).integer().not_null())
                    .col(ColumnDef::new(Sessions::StartTime).date_time().not_null())
                    .col(ColumnDef::new(Sessions::LastActive).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Nutrients::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Nutrients::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Nutrients::Calories).float().not_null())
                    .col(ColumnDef::new(Nutrients::Carbs).float().not_null())
                    .col(ColumnDef::new(Nutrients::Protein).float().not_null())
                    .col(ColumnDef::new(Nutrients::Fat).float().not_null())
                    .col(ColumnDef::new(Nutrients::Source).float().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(NutrientTargets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(NutrientTargets::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(NutrientTargets::User).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_USER")
                            .from(NutrientTargets::Table, NutrientTargets::User)
                            .to(UserAccounts::Table, UserAccounts::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(NutrientTargets::TargetNutrients)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_NUTRIENTS")
                            .from(NutrientTargets::Table, NutrientTargets::TargetNutrients)
                            .to(Nutrients::Table, Nutrients::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(NutrientTargets::DateBegin)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(NutrientTargets::DateEnd)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Units::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Units::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Units::Name).string().not_null())
                    .col(ColumnDef::new(Units::Abbreviation).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Consumable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Consumable::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Consumable::Name).string().not_null())
                    .col(ColumnDef::new(Consumable::Notes).string().not_null())
                    .col(ColumnDef::new(Consumable::PortionAmount).float().not_null())
                    .col(ColumnDef::new(Consumable::PortionUnit).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_PU")
                            .from(Consumable::Table, Consumable::PortionUnit)
                            .to(Units::Table, Units::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Consumable::Nutrients).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_N")
                            .from(Consumable::Table, Consumable::Nutrients)
                            .to(Nutrients::Table, Nutrients::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CompositeConsumable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CompositeConsumable::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CompositeConsumable::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompositeConsumable::Notes)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompositeConsumable::PortionAmount)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompositeConsumable::PortionUnit)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_CC")
                            .from(CompositeConsumable::Table, CompositeConsumable::PortionUnit)
                            .to(Units::Table, Units::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CompositeConsumable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CompositeConsumable::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CompositeConsumable::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompositeConsumable::Notes)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompositeConsumable::PortionAmount)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompositeConsumable::PortionUnit)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_CC")
                            .from(CompositeConsumable::Table, CompositeConsumable::PortionUnit)
                            .to(Units::Table, Units::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CompositeConsumableNutrients::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CompositeConsumableNutrients::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CompositeConsumableNutrients::CompositeConsumableId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CompositeConsumableNutrients::Consumable).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_SUB_C_ID")
                            .from(
                                CompositeConsumableNutrients::Table,
                                CompositeConsumableNutrients::Consumable,
                            )
                            .to(Consumable::Table, Consumable::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(CompositeConsumableNutrients::CompositeConsumable).integer(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_SUB_CC_ID")
                            .from(
                                CompositeConsumableNutrients::Table,
                                CompositeConsumableNutrients::CompositeConsumable,
                            )
                            .to(Consumable::Table, Consumable::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(CompositeConsumableNutrients::Scaling)
                            .float()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ConsumptionRecord::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ConsumptionRecord::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ConsumptionRecord::User).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_USER_ID")
                            .from(ConsumptionRecord::Table, ConsumptionRecord::User)
                            .to(UserAccounts::Table, UserAccounts::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(ConsumptionRecord::Consumable).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_C_ID")
                            .from(ConsumptionRecord::Table, ConsumptionRecord::Consumable)
                            .to(Consumable::Table, Consumable::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(ConsumptionRecord::CompositeConsumable).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_CC_ID")
                            .from(
                                ConsumptionRecord::Table,
                                ConsumptionRecord::CompositeConsumable,
                            )
                            .to(CompositeConsumable::Table, CompositeConsumable::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(ConsumptionRecord::DateTime).date_time())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserAccounts::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(NutrientTargets::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Units::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Nutrients::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Consumable::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CompositeConsumable::Table).to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(CompositeConsumableNutrients::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(ConsumptionRecord::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Sessions {
    Id,
    Table,
    UserId,
    StartTime,
    LastActive,
}

#[derive(Iden)]
enum UserAccounts {
    Id,
    Table,
    Name,
    Salt,
    PasswordHash,
}

#[derive(Iden)]
enum NutrientTargets {
    Id,
    Table,
    /// UserAccounts Table Id
    User,
    /// Nutrient Table Id
    TargetNutrients,
    DateBegin,
    /// may not be needed if this can be quickly reconstructed by looking at the proceeding macro date begin
    DateEnd,
}

#[derive(Iden)]
enum Units {
    Id,
    Table,
    Name,
    Abbreviation,
}

#[derive(Iden)]
enum Nutrients {
    Id,
    Table,
    Calories,
    Carbs,
    Protein,
    Fat,
    Source,
}

#[derive(Iden)]
enum Consumable {
    Id,
    Table,
    Name,
    Notes,
    PortionAmount,
    /// Units Table Id
    PortionUnit,
    /// Nutrients Table Id
    Nutrients,
}

#[derive(Iden)]
enum CompositeConsumable {
    Id,
    Table,
    Name,
    Notes,
    PortionAmount,
    /// (Units Table Id)
    PortionUnit,
}

#[derive(Iden)]
enum CompositeConsumableNutrients {
    Id,
    Table,
    /// used to refer to the composite consumable that these nutrients belong to
    /// (Composite Consumable Table Id)
    CompositeConsumableId,
    /// (Consumable Table Id)
    /// Used to refer to the nutrients that the owning consumabled comprises. Mutually exclusive with Consumable attribute.
    Consumable,
    /// Used to refer to the nutrients that the owning consumabled comprises. Mutually exclusive with Consumable attribute.
    /// (Composite Consumable Table Id)
    CompositeConsumable,
    Scaling,
}

#[derive(Iden)]
enum ConsumptionRecord {
    Id,
    Table,
    /// (UserAccounts Table Id)
    User,
    /// (Consumable table id)
    Consumable,
    /// (Composite Consumable table id)
    CompositeConsumable,
    /// possibly a single time-date field depending on DB
    DateTime,
}

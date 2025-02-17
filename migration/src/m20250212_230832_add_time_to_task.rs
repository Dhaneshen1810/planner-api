use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add time column
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .add_column(ColumnDef::new(Tasks::Time).string().null())
                    .to_owned(),
            )
            .await?;

        // Make date column nullable
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .modify_column(ColumnDef::new(Tasks::Date).date().null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove time column
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .drop_column(Tasks::Time)
                    .to_owned(),
            )
            .await?;

        // Make date column required again
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .modify_column(ColumnDef::new(Tasks::Date).date().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Tasks {
    Table,
    Time,
    Date,
}

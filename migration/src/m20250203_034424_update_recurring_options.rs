use sea_orm_migration::{prelude::*, sea_orm::ConnectionTrait, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_backend = manager.get_database_backend();

        // Step 1: Add a new column as an array of ENUMs
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .add_column(
                        ColumnDef::new(Tasks::RecurringOptionNew)
                            .array(ColumnType::Custom(SeaRc::new(Alias::new(
                                "recurring_option",
                            ))))
                            .not_null()
                            .default("{}"),
                    )
                    .to_owned(),
            )
            .await?;

        // Step 2: Migrate data to new array column
        manager
            .get_connection()
            .execute(Statement::from_string(
                db_backend,
                r#"
                UPDATE tasks 
                SET recurring_option_new = ARRAY[recurring_option]::recurring_option[];
                "#
                .to_owned(),
            ))
            .await?;

        // Step 3: Drop the old `recurring_option` column
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .drop_column(Tasks::RecurringOption)
                    .to_owned(),
            )
            .await?;

        // Step 4: Rename the new column to `recurring_option`
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .rename_column(Tasks::RecurringOptionNew, Tasks::RecurringOption)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_backend = manager.get_database_backend();

        // Step 1: Recreate the original ENUM column
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .add_column(
                        ColumnDef::new(Tasks::RecurringOptionOld)
                            .custom(Alias::new("recurring_option")) // Corrected
                            .null()
                            .default("NONE"),
                    )
                    .to_owned(),
            )
            .await?;

        // Step 2: Convert back from ARRAY to single ENUM (taking the first value)
        manager
            .get_connection()
            .execute(Statement::from_string(
                db_backend,
                r#"
                UPDATE tasks 
                SET recurring_option_old = recurring_option[1];
                "#
                .to_owned(),
            ))
            .await?;

        // Step 3: Drop the new ENUM ARRAY column
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .drop_column(Tasks::RecurringOption)
                    .to_owned(),
            )
            .await?;

        // Step 4: Rename rollback column back to `recurring_option`
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .rename_column(Tasks::RecurringOptionOld, Tasks::RecurringOption)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Tasks {
    Table,
    RecurringOption,
    RecurringOptionNew,
    RecurringOptionOld,
}

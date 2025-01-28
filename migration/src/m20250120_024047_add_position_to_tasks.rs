use sea_orm_migration::{prelude::*, sea_orm::ConnectionTrait, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add the new `position` column
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .add_column(
                        ColumnDef::new(Tasks::Position)
                            .integer()
                            .not_null()
                            .default(0), // Temporary default to avoid null errors during updates
                    )
                    .to_owned(),
            )
            .await?;

        // Populate the `position` column with auto-incremented values
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                WITH cte AS (
                    SELECT id, ROW_NUMBER() OVER (ORDER BY id) AS position
                    FROM tasks
                )
                UPDATE tasks
                SET position = cte.position
                FROM cte
                WHERE tasks.id = cte.id;
                "#
                .to_owned(),
            ))
            .await?;

        // Set the `position` column as auto-increment for new entries
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .modify_column(
                        ColumnDef::new(Tasks::Position)
                            .integer()
                            .not_null()
                            .auto_increment(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove the `position` column
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .drop_column(Tasks::Position)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Tasks {
    Table,
    Position,
}

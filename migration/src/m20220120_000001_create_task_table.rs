use sea_orm::{ConnectionTrait, Statement};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE TYPE recurring_option AS ENUM (
                    'MONDAY',
                    'TUESDAY',
                    'WEDNESDAY',
                    'THURSDAY',
                    'FRIDAY',
                    'SATURDAY',
                    'SUNDAY',
                    'NONE'
                )
                "#
                .to_owned(),
            ))
            .await
            .map(|_| ())?;

        manager
            .create_table(
                Table::create()
                    .table(Tasks::Table)
                    .if_not_exists()
                    .col(pk_auto(Tasks::Id))
                    .col(ColumnDef::new(Tasks::Name).string().not_null())
                    .col(ColumnDef::new(Tasks::Date).date().not_null())
                    .col(
                        ColumnDef::new(Tasks::RecurringOption)
                            .custom(RecurringOption::RecurringOption) // Use enum variant directly
                            .null()
                            .default("NONE"),
                    )
                    .col(
                        ColumnDef::new(Tasks::IsCompleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tasks::Table).to_owned())
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"DROP TYPE recurring_option"#.to_owned(),
            ))
            .await
            .map(|_| ())?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Tasks {
    Table,
    Id,
    Name,
    Date,
    RecurringOption,
    IsCompleted,
}

#[derive(DeriveIden)]
enum RecurringOption {
    RecurringOption, // The variant name will be converted to "recurring_option"
}

pub use sea_orm_migration::prelude::*;

mod m20220120_000001_create_task_table;
mod m20250116_011832_rename_name_to_title;
mod m20250120_024047_add_position_to_tasks;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220120_000001_create_task_table::Migration),
            Box::new(m20250116_011832_rename_name_to_title::Migration),
            Box::new(m20250120_024047_add_position_to_tasks::Migration),
        ]
    }
}

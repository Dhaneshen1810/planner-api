use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    pub date: Option<Date>,
    pub time: Option<String>,
    #[serde(default)]
    pub recurring_option: Vec<RecurringOption>,
    pub is_completed: bool,
    pub position: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, Serialize, Deserialize, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "recurring_option")]
pub enum RecurringOption {
    #[sea_orm(string_value = "MONDAY")]
    Monday,
    #[sea_orm(string_value = "TUESDAY")]
    Tuesday,
    #[sea_orm(string_value = "WEDNESDAY")]
    Wednesday,
    #[sea_orm(string_value = "THURSDAY")]
    Thursday,
    #[sea_orm(string_value = "FRIDAY")]
    Friday,
    #[sea_orm(string_value = "SATURDAY")]
    Saturday,
    #[sea_orm(string_value = "SUNDAY")]
    Sunday,
}

impl ActiveModelBehavior for ActiveModel {}

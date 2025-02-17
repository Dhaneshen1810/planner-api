use entity::task::RecurringOption;
use sea_orm::prelude::Date;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateTaskRequest {
    pub id: i32,
    pub title: String,
    pub date: Option<Date>,
    pub time: Option<String>,
    pub recurring_option: Vec<RecurringOption>,
    pub is_completed: bool,
    pub position: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateTaskByIdRequest {
    pub title: String,
    pub date: Option<Date>,
    pub time: Option<String>,
    pub recurring_option: Vec<RecurringOption>,
    pub is_completed: bool,
    pub position: i32,
}
